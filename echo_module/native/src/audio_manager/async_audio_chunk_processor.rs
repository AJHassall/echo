use std::
    collections::HashMap
;
use uuid::Uuid;
use tokio::sync::Mutex;

use crate::{audio_manager::AudioChunkProcessor, web_rtc_vad::WebRtcVadFacade};

pub struct AsyncAudioChunkProcessor {
    processor: Mutex<AudioChunkProcessor>,
}

impl AsyncAudioChunkProcessor {
    pub fn new(vad: WebRtcVadFacade) -> Self {
        AsyncAudioChunkProcessor {
            processor: Mutex::new(AudioChunkProcessor::new(vad)),
        }
    }

    pub async fn process_chunk(&self, audio_chunk: Vec<f32>) {
        let mut processor = self.processor.lock().await;
        processor.process_chunk(audio_chunk);
    }

    pub async fn set_silence_duration_threshold(&self, new_threshold: f64) {
        let mut processor = self.processor.lock().await;
        processor.set_silence_duration_threshold(new_threshold);
    }

    pub async fn get_current_audio(&self) -> HashMap<Uuid, Vec<f32>> {
        let mut processor = self.processor.lock().await;
        processor.get_current_audio()
    }

    pub async fn clear_current_audio(&self) {
        let mut processor = self.processor.lock().await;
        processor.clear_current_audio();
    }


    pub async fn get_frame_size(&self) -> usize {
        let mut processor = self.processor.lock().await;
        processor.get_frame_size()
    }
}