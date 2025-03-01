use lazy_static::lazy_static;
use neon::event::Channel;
use neon::handle::Root;
use neon::types::JsFunction;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, MutexGuard, TryLockError};
use std::time::Duration;
use tokio::runtime::Runtime;
use webrtc_vad::VadMode;

use crate::audio_transcription_controller::{
    ConcurrentTranscriber, CustomError, TranscribeResponse, Transcriber,
};
use crate::event_publisher::{initialise_event_publisher, EventPublisher};
use crate::transcription_engine::TranscriptionEngine;
use crate::{api, audio, web_rtc_vad};
use crate::{audio::AudioDataReceiver, audio_manager::AudioChunkProcessor, jack::JackClient};

lazy_static! {
    static ref IS_RECORDING: AtomicBool = AtomicBool::new(false);
    static ref RUNTIME: Mutex<Option<Runtime>> = Mutex::new(None);
    static ref TRANSCRIPTION_ENGINE: Mutex<Option<ConcurrentTranscriber<TranscriptionEngine>>> =
        Mutex::new(None);
}

pub struct AudioRecorder;

impl AudioRecorder {
    pub fn send_transcription<T: Into<String>>(transcription: T, event_id: String) {
        EventPublisher::publish_if_available(
            "transcription".to_string(),
            transcription.into(),
            event_id,
        );
    }

    pub fn send_most_recent_energy<T: Into<String>>(energy: T) {
        EventPublisher::publish_if_available(
            "new_energy".to_string(),
            energy.into(),
            "todo".to_string(),
        );
    }

    async fn run_recorder(mut processor: AudioChunkProcessor) {
        println!("Recording task started");

        let mut jack_client = JackClient::new();
        let receiver_arc = jack_client.get_audio_receiver();
        jack_client.start_processing();

        let mut buffer = vec![];

        while IS_RECORDING.load(Ordering::SeqCst) {
            if let Ok(receiver_guard) = receiver_arc.lock() {
                if let Ok(audio_data) = receiver_guard.recv() {
                    buffer.extend(audio_data.clone());
                    if buffer.len() >= processor.get_frame_size() {
                        processor.process_chunk(buffer.clone());
                        buffer.clear();
                    }
                } else {
                    break;
                }
            }

            let new_audio = processor.get_current_audio();
            for (uuid, audio) in new_audio.iter() {
                if audio.is_empty() {
                    break;
                };

                if let Ok(transcription) = try_transcribe(audio.clone()) {
                    AudioRecorder::send_transcription(
                        transcription.transcription,
                        uuid.to_string(),
                    );
                    processor.clear_current_audio();
                } else {
                    println!("Transcriber is not ready");
                }
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
            let vad = web_rtc_vad::WebRtcVadFacade::new(48000, VadMode::Quality)?;
            let mut processor = AudioChunkProcessor::new(vad);

            processor.set_silence_duration_threshold(silence_threshhold);

            runtime.spawn(AudioRecorder::run_recorder(processor));
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
            let mut runtime_guard = RUNTIME.lock().unwrap();
            *runtime_guard = Some(runtime);
        }

        let transcription_engine = ConcurrentTranscriber::new(
            TranscriptionEngine::new("../whisper-models/ggml-tiny.en.bin").unwrap(),
        );
        {
            let mut engine_guard = TRANSCRIPTION_ENGINE.lock().unwrap();
            *engine_guard = Some(transcription_engine);
        }

        Ok(())
    }
}

fn try_transcribe(data: Vec<f32>) -> Result<TranscribeResponse, CustomError> {
    let mut engine_guard = TRANSCRIPTION_ENGINE.lock().unwrap();
    if let Some(engine) = engine_guard.as_mut() {
        engine.transcribe(data)
    } else {
        Err(CustomError::EngineNotInitialized)
    }
}

#[cfg(test)]
mod tests {}
