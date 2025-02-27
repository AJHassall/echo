use futures::channel;
use lazy_static::lazy_static;
use neon::event::Channel;
use neon::handle::Root;
use neon::types::JsFunction;
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
}

pub struct AudioRecorder;

impl AudioRecorder {
    pub fn send_transcription<T: Into<String>>(transcription: T) {
        EventPublisher::publish_if_available("transcription".to_string(), transcription.into());
    }

    pub fn send_most_recent_energy<T: Into<String>>(energy: T) {
        return;
        EventPublisher::publish_if_available(
            "new_energy".to_string(),
            energy.into(),
        );
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


            let new_audio = processor.get_current_audio();
            for data in new_audio.iter().cloned() {
                AudioRecorder::send_transcription(&transcribe(data));
            }

            let energy = processor.get_most_recent_energy();
            AudioRecorder::send_most_recent_energy(energy.to_string());
            tokio::time::sleep(Duration::from_millis(10)).await;
        }

        jack_client.stop_processing();
    }

    pub fn start(silence_threshhold: f64, duration_threshold: f64) -> Result<(), String> {
        if IS_RECORDING.load(Ordering::SeqCst) {
            return Err("recorder already running".to_string());
        }

        IS_RECORDING.store(true, Ordering::SeqCst);

        let runtime_guard = RUNTIME.lock().unwrap();
        if let Some(runtime) = &*runtime_guard {
            let vad = VoiceActivityDetector::new();
            let mut processor = AudioChunkProcessor::new(vad);

            processor.set_silence_duration_threshold(silence_threshhold);
            processor.set_silence_threshold(duration_threshold);

            runtime.spawn(AudioRecorder::run_recorder(processor));
        } 
        else {
            return Err("eprintlnError: Tokio runtime not initialized!".to_string());
        }

        Ok(())
    }

    pub fn stop() -> Result<(), String> {
        if !IS_RECORDING.load(Ordering::SeqCst) {
            return Err("Recording is already stopped.".to_string());
        }

        IS_RECORDING.store(false, Ordering::SeqCst);
        Ok(())
    }

    pub fn initialise(call_back: Root<JsFunction>, channel: Channel) -> Result<(), String> {
    initialise_event_publisher(Arc::new(Mutex::new(call_back)), channel);

        let runtime = Runtime::new().unwrap();
        {
            // Scope for MutexGuard
            let mut runtime_guard = RUNTIME.lock().unwrap();
            *runtime_guard = Some(runtime);
        }


        let transcription_engine =
            TranscriptionEngine::new("whisper-models/ggml-tiny.en.bin").unwrap();
        {
            let mut engine_guard = TRANSCRIPTION_ENGINE.lock().unwrap();
            *engine_guard = Some(transcription_engine);
        }

        Ok(())
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
