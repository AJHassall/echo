use base64::encode;
use neon::prelude::*;
use reqwest::{Body, Client, Error as ReqwestError};
use serde::Serialize;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::runtime::Runtime;
use tokio::task::{self, JoinHandle};
use tokio_util::codec::{BytesCodec, FramedRead}; // Import Runtime

use crate::api;
use crate::transcription_engine::{self, TranscriptionEngine};
use crate::{
    audio::AudioDataReceiver,
    jack::JackClient,
    vad::{AudioChunkProcessor, VoiceActivityDetector},
};

pub struct AudioRecorder;

// Use AtomicBool to control recording state safely across threads
static IS_RECORDING: AtomicBool = AtomicBool::new(false);
// Store the Tokio Runtime in a static variable. **Caution: Static Mutability!**
//  It's generally better to avoid static mutability if possible.
//  For a more robust approach, consider passing the runtime through context if your module structure allows.
static RUNTIME: Mutex<Option<Runtime>> = Mutex::new(None);
static TRANSCRIPTION_ENGINE: Mutex<Option<TranscriptionEngine>> = Mutex::new(None);
static TRANSCRIPTION_CALLBACK: Mutex<Option<Root<JsFunction>>> = Mutex::new(None); // Store JS callback

impl AudioRecorder {
    // Core recording logic, now controlled by IS_RECORDING flag
    async fn run_recorder() {
        println!("Recording task started");

        let mut jack_client = JackClient::new();
        let receiver_arc = jack_client.get_audio_receiver();
        jack_client.start_processing();

        let vad = VoiceActivityDetector::new();
        let sample_rate = 16000;
        let mut processor = AudioChunkProcessor::new(vad, sample_rate);

        //   let mut h = vec![];

        while IS_RECORDING.load(Ordering::SeqCst) {
            // Check recording state in the loop
            if let Ok(receiver_guard) = receiver_arc.lock() {
                if let Ok(audio_data) = receiver_guard.recv() {
                    processor.process_chunk(audio_data);
                } else {
                    break; // Handle disconnection or other errors
                }
            }

            let new_audio = processor.get_current_audio();
            for data in new_audio.to_vec() {
                transcribe(data);
            }

            tokio::time::sleep(Duration::from_millis(10)).await;
        }

        println!("Stopping recording tasks...");
        // for i in h{
        //     let _s = i.await; // Await sending tasks to finish, but ignore result in stop case for now
        // }

        jack_client.stop_processing();
        println!("Recording task finished");
    }

    pub fn start(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        if IS_RECORDING.load(Ordering::SeqCst) {
            println!("Recording is already started.");
            return Ok(cx.undefined()); // Or return an error if you prefer
        }

        IS_RECORDING.store(true, Ordering::SeqCst);
        println!("Start recording requested");

        // Get the runtime from the static Mutex
        let runtime_guard = RUNTIME.lock().unwrap();
        if let Some(runtime) = &*runtime_guard {
            // Spawn the recording task on the existing runtime
            runtime.spawn(AudioRecorder::run_recorder()); // Use runtime.spawn!
        } else {
            eprintln!("Error: Tokio runtime not initialized!");
            //  return Err(neon::result::Throw("Tokio runtime not initialized")); // Return an error to JS
        }
        drop(runtime_guard); // Explicitly drop the mutex guard

        Ok(cx.undefined())
    }

    pub fn stop(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        if !IS_RECORDING.load(Ordering::SeqCst) {
            println!("Recording is already stopped.");
            return Ok(cx.undefined()); // Or return an error
        }

        IS_RECORDING.store(false, Ordering::SeqCst);
        println!("Stop recording requested");
        // No need to explicitly stop the task, `run_recorder` will exit gracefully

        Ok(cx.undefined())
    }

    pub fn initialise(mut cx: FunctionContext) -> JsResult<JsObject> {
        // Return JsObject not JsUndefined
        // Initialize Tokio Runtime **once** when the module is initialized
        let runtime = Runtime::new().unwrap(); // Handle error properly in production
        {
            // Scope for MutexGuard
            let mut runtime_guard = RUNTIME.lock().unwrap();
            *runtime_guard = Some(runtime);
        }

        // let callback_js_function = cx.argument::<JsFunction>(0)?;
        // {
        //     let mut callback_guard = TRANSCRIPTION_CALLBACK.lock().unwrap();
        //     *callback_guard = Some(Root::new(&mut cx, &callback_js_function)); // Store the callback (already rooted correctly)
        // }

        let transcription_engine =
            TranscriptionEngine::new("whisper-models/ggml-tiny.en.bin").unwrap(); // Handle error properly in production
        {
            // Scope for MutexGuard
            let mut engine_guard = TRANSCRIPTION_ENGINE.lock().unwrap();
            *engine_guard = Some(transcription_engine);
        }

        // let start_fn = cx.function(AudioRecorder::start)?;
        // let stop_fn = cx.function(AudioRecorder::stop)?;

        let obj = JsObject::new(&mut cx);
        // obj.set(&mut cx, "start", start_fn)?;
        // obj.set(&mut cx, "stop", stop_fn)?;

        Ok(obj) // Return the object
    }
}

fn transcribe(data: Vec<f32>) {
    println!("here");
    let mut engine_guard = TRANSCRIPTION_ENGINE.lock().unwrap();
    if let Some(engine) = engine_guard.as_mut() {
        // Spawn the recording task on the existing runtime

        let response = api::transcribe_stream(data, engine);

        match response {
            Ok(text) => {
                // **Call the JavaScript callback with the transcription result here!**

                println!("{}", text.transcription);
            }
            Err(error) => {
                println!("transcription error: {}", error);
            }
        }
    } else {
        eprintln!("Error: Tokio runtime not initialized!");
        //  return Err(neon::result::Throw("Tokio runtime not initialized")); // Return an error to JS
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
