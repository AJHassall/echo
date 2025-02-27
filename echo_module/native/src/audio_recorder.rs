use futures::channel;
use lazy_static::lazy_static;
use neon::prelude::*;
use serde_json::Number;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::runtime::Runtime;
// Import Runtime

use crate::event_publisher::{initialise_event_publisher, EventPublisher}; // Import the EventPublisher type.
use crate::transcription_engine::TranscriptionEngine;
use crate::{api, vad};
use crate::{
    audio::AudioDataReceiver,
    jack::JackClient,
    vad::{AudioChunkProcessor, VoiceActivityDetector},
};

lazy_static! {
    static ref IS_RECORDING: AtomicBool = AtomicBool::new(false);
    static ref RUNTIME: Mutex<Option<Runtime>> = Mutex::new(None);
    static ref TRANSCRIPTION_ENGINE: Mutex<Option<TranscriptionEngine>> = Mutex::new(None);
    static ref TRANSCRIPTIONS: Mutex<Option<Vec<String>>> = Mutex::new(None);
    static ref MOST_RECENT_ENERGY: Mutex<f64> = Mutex::new(0.0);
}

pub struct AudioRecorder;

impl AudioRecorder {
    pub fn get_transcriptions(mut cx: FunctionContext) -> JsResult<JsArray> {
        let transcription_guard = TRANSCRIPTIONS.lock().unwrap(); // No need for mut, just reading
        let a;

        if let Some(transcriptions_ref) = transcription_guard.as_ref() {
            // Get a reference
            let transcriptions = transcriptions_ref.clone(); // Clone the Vec for JS array
            a = JsArray::new(&mut cx, transcriptions.len());

            for (i, s) in transcriptions.iter().enumerate() {
                let v = cx.string(s);
                a.set(&mut cx, i as u32, v)?;
            }
        } else {
            a = JsArray::new(&mut cx, 0);
        }

        Ok(a)
    }

    pub fn clear_transcriptions(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let mut transcription_guard = TRANSCRIPTIONS.lock().unwrap();

        transcription_guard.take();

        *transcription_guard = Some(Vec::new()); // Uncomment if you want to re-initialize

        Ok(cx.undefined()) // Return undefined, as clearing doesn't return data
    }

    fn push_transcription(transcription: &str) {
        let mut transcription_guard = TRANSCRIPTIONS.lock().unwrap();

        let transcriptions = transcription_guard.get_or_insert_with(Vec::new);
        transcriptions.push(String::from(transcription));
    }

    pub fn get_most_recent_energy(mut cx: FunctionContext) -> JsResult<JsNumber> {
        let transcription_guard = MOST_RECENT_ENERGY.lock().unwrap(); // No need for mut, just reading
        let transcriptions = transcription_guard;

        Ok(cx.number(*transcriptions))
    }

    fn set_energy(energy: f64) {
        let transcription_guard = MOST_RECENT_ENERGY.lock().unwrap();

        let mut transcriptions = transcription_guard;
        *transcriptions = energy;
    }

    async fn run_recorder(audio_chunk_processor: AudioChunkProcessor) {
        println!("Recording task started");

        let mut jack_client = JackClient::new();
        let receiver_arc = jack_client.get_audio_receiver();
        jack_client.start_processing();

        let mut processor = audio_chunk_processor;

        while IS_RECORDING.load(Ordering::SeqCst) {
            if let Ok(receiver_guard) = receiver_arc.lock() {
                if let Ok(audio_data) = receiver_guard.recv() {
                    processor.process_chunk(audio_data);
                } else {
                    break;
                }
            }

            //sending audio back to client

            EventPublisher::publish_if_available(String::from("Hello from Rust!"));

            let new_audio = processor.get_current_audio();
            for data in new_audio.iter().cloned() {
                AudioRecorder::push_transcription(&transcribe(data));
            }

            let energy = processor.get_most_recent_energy();
            AudioRecorder::set_energy(energy);

            tokio::time::sleep(Duration::from_millis(10)).await;
        }

        jack_client.stop_processing();
    }

    pub fn start(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let silence_threshhold = cx.argument::<JsNumber>(0)?;
        let duration_threshold = cx.argument::<JsNumber>(1)?;

        let silence_threshhold = silence_threshhold.value(&mut cx);
        let duration_threshold = duration_threshold.value(&mut cx);

        if IS_RECORDING.load(Ordering::SeqCst) {
            return Ok(cx.undefined());
        }

        IS_RECORDING.store(true, Ordering::SeqCst);

        let runtime_guard = RUNTIME.lock().unwrap();
        if let Some(runtime) = &*runtime_guard {
            let vad = VoiceActivityDetector::new();
            let mut processor = AudioChunkProcessor::new(vad);

            processor.set_silence_duration_threshold(silence_threshhold);
            processor.set_silence_threshold(duration_threshold);

            runtime.spawn(AudioRecorder::run_recorder(processor));
        } else {
            eprintln!("Error: Tokio runtime not initialized!");
        }

        Ok(cx.undefined())
    }

    pub fn stop(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        if !IS_RECORDING.load(Ordering::SeqCst) {
            println!("Recording is already stopped.");
            return Ok(cx.undefined()); // Or return an error
        }

        IS_RECORDING.store(false, Ordering::SeqCst);
        println!("Stop recording requested");

        Ok(cx.undefined())
    }

    pub fn initialise(mut cx: FunctionContext) -> JsResult<JsObject> {
        let event_receiver = cx.argument::<JsFunction>(0)?.root(&mut cx);
        let channel = cx.channel();

        initialise_event_publisher(Arc::new(Mutex::new(event_receiver)), channel);

        let runtime = Runtime::new().unwrap();
        {
            // Scope for MutexGuard
            let mut runtime_guard = RUNTIME.lock().unwrap();
            *runtime_guard = Some(runtime);
        }

        let transcriptions = vec![];
        {
            let mut runtime_guard = TRANSCRIPTIONS.lock().unwrap();
            *runtime_guard = Some(transcriptions);
        }

        let transcription_engine =
            TranscriptionEngine::new("whisper-models/ggml-tiny.en.bin").unwrap();
        {
            let mut engine_guard = TRANSCRIPTION_ENGINE.lock().unwrap();
            *engine_guard = Some(transcription_engine);
        }

        let obj = JsObject::new(&mut cx);

        Ok(obj)
    }
}

fn transcribe(data: Vec<f32>) -> String {
    let mut engine_guard = TRANSCRIPTION_ENGINE.lock().unwrap();
    if let Some(engine) = engine_guard.as_mut() {
        let response = api::transcribe_stream(data, engine);

        match response {
            Ok(text) => {
                return text.transcription;
            }
            Err(error) => {
                println!("transcription error: {}", error);
            }
        }
    } else {
        eprintln!("Error: Tokio runtime not initialized!");
    }
    "".to_owned()
}

#[cfg(test)]
mod tests {}
