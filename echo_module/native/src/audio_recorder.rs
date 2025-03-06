use lazy_static::lazy_static;
use neon::event::Channel;
use neon::handle::Root;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
use tokio::task;
use webrtc_vad::VadMode;

use crate::audio_manager::AsyncAudioChunkProcessor;
use crate::audio_transcription_controller::{
    ConcurrentTranscriber, CustomError, TranscribeResponse, Transcriber,
};
use crate::event_publisher::{initialise_event_publisher, EventPublisher};
use crate::jack::JackClient;
use crate::transcription_engine::TranscriptionEngine;
use crate::util::Timer;
use crate::web_rtc_vad;

use neon::prelude::*;
use once_cell::sync::OnceCell;

lazy_static! {
    static ref IS_RECORDING: AtomicBool = AtomicBool::new(false);
    static ref TRANSCRIPTION_ENGINE: Mutex<Option<ConcurrentTranscriber<TranscriptionEngine>>> =
        Mutex::new(None);
}

fn runtime() -> &'static Runtime {
    static RUNTIME: OnceCell<Runtime> = OnceCell::new();

    if let Ok(runtime) = RUNTIME.get_or_try_init(Runtime::new) {
        println!("runtime intiialised");

        runtime
    } else {
        println!("error initialising runtime");
        panic!();
    }
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

    async fn run_recorder(duration_threshold: f64, audio_souces: Vec<String>) {
        println!("Recording task started");

        let vad = web_rtc_vad::WebRtcVadFacade::new(48000, VadMode::Quality).expect("msg");
        let processor = AsyncAudioChunkProcessor::new(vad);

        processor.set_silence_duration_threshold(2.0).await;

        let mut jack_client = JackClient::new(audio_souces);
        let receiver_arc = jack_client.get_audio_receiver();
        jack_client.start_processing();

        let (transcribe_tx, mut transcribe_rx) =
            tokio::sync::mpsc::channel::<(uuid::Uuid, Vec<f32>)>(32);

        let transcribe_running = std::sync::Arc::new(tokio::sync::Mutex::new(false));
        let transcribe_running_clone = std::sync::Arc::clone(&transcribe_running);

        task::spawn(async move {
            while let Some((uuid, audio)) = transcribe_rx.recv().await {
                let t = Timer::new("inside transcription loop".to_string());
                {
                    let t = Timer::new("awaiting lock".to_string());

                    let mut running = transcribe_running_clone.lock().await;
                    *running = true;
                }

                if let Ok(response) = try_transcribe(audio).await {
                    Self::send_transcription(response.transcription, uuid.to_string())
                }
                {
                    let t = Timer::new("awaiting lock".to_string());

                    let mut running = transcribe_running_clone.lock().await;
                    *running = false;
                }
            }
        });

        let mut buffer: Vec<f32> = vec![];
        while IS_RECORDING.load(Ordering::SeqCst) {
            let mut receiver_guard = receiver_arc.lock().await;
            let audio_data = &*receiver_guard.recv().await.expect("msg");

            buffer.extend_from_slice(audio_data);
            if buffer.len() >= processor.get_frame_size().await {
                processor.process_chunk(std::mem::take(&mut buffer)).await;
            }

            {
                let running = transcribe_running.lock().await;

                if !*running && processor.has_new_audio().await {
                    let to_process = processor.get_current_audio().await;

                    for (uuid, audio) in to_process {
                        transcribe_tx
                            .send((uuid, audio))
                            .await
                            .expect("failed to send");
                    }

                    processor.clear_current_audio().await;
                }
            }
        }

        jack_client.stop_processing();
    }

    pub fn start(duration_threshold: f64, audio_souces: Vec<String>) -> Result<(), String> {
        if IS_RECORDING.load(Ordering::SeqCst) {
            return Err("recorder already running".to_string());
        }

        IS_RECORDING.store(true, Ordering::SeqCst);

        let rt = runtime();

        rt.spawn(async move { AudioRecorder::run_recorder(duration_threshold, audio_souces).await });
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

        let transcription_engine = ConcurrentTranscriber::new(
            TranscriptionEngine::new("../whisper-models/ggml-tiny.en.bin").unwrap(),
        );
        {
            let mut engine_guard = TRANSCRIPTION_ENGINE.lock().unwrap();
            *engine_guard = Some(transcription_engine);
        }

        println!("init success");

        Ok(())
    }
}

async fn try_transcribe(data: Vec<f32>) -> Result<TranscribeResponse, CustomError> {
    let mut engine_guard = TRANSCRIPTION_ENGINE.lock().unwrap();
    if let Some(engine) = engine_guard.as_mut() {
        engine.transcribe(data)
    } else {
        Err(CustomError::EngineNotInitialized)
    }
}

#[cfg(test)]
mod tests {}
