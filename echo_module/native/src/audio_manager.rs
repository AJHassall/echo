use std::{
    collections::HashMap,
    time::{self, Duration, Instant},
};

use uuid::Uuid;

use crate::web_rtc_vad::WebRtcVadFacade;

pub struct AudioChunkProcessor {
    vad: WebRtcVadFacade,
    //current_chunk: Vec<f32>,
    current_chunk_uuid: Uuid,
    output_chunks: HashMap<Uuid, Vec<f32>>,
    last_silence_time: Instant,
    silence_duration_threshold: Duration,
    most_recent_energy: f64,
}

//TODO see if i can remove this
//currently, when i use an audioprocesser i create it first then move it to a thread
//probably makes more sense to create it in the thread
unsafe impl Send for AudioChunkProcessor {}

impl<'a> AudioChunkProcessor {
    pub fn new(vad: WebRtcVadFacade) -> Self {
        AudioChunkProcessor {
            vad,
            //current_chunk: Vec::new(),
            current_chunk_uuid: Uuid::new_v4(),
            last_silence_time: Instant::now(),
            silence_duration_threshold: Duration::from_secs(1),
            most_recent_energy: 0.0,
            output_chunks: HashMap::new(),
        }
    }

    pub fn process_chunk(&mut self, audio_chunk: Vec<f32>) {
        if self
            .vad
            .is_speech(&audio_chunk[0..1440])
            .expect("VAD processing error")
        {
            self.create_or_extend(&audio_chunk[..]);
            self.last_silence_time = time::Instant::now();
        } else if time::Instant::now().duration_since(self.last_silence_time)
            >= self.silence_duration_threshold
        {
            if self.output_chunks.contains_key(&self.current_chunk_uuid) {
                self.current_chunk_uuid = Uuid::new_v4();
            }
        }
    }
    pub fn create_or_extend(&mut self, audio: &[f32]) {
        match self.output_chunks.get_mut(&self.current_chunk_uuid) {
            Some(vec) => {
                vec.extend_from_slice(audio); // Use extend_from_slice for efficiency with slices
            }
            None => {
                self.output_chunks
                    .insert(self.current_chunk_uuid, audio.to_vec());
            }
        }
    }

    pub fn set_silence_duration_threshold(&mut self, new_threshhold: f64) {
        self.silence_duration_threshold = Duration::from_millis((new_threshhold * 1000.0) as u64)
    }

    pub fn get_current_audio(&mut self) -> HashMap<Uuid, Vec<f32>> {
        let out = self.output_chunks.clone();
        out
    }

    pub fn clear_current_audio(&mut self) {
        self.output_chunks
            .retain(|&key, _| key == self.current_chunk_uuid);
    }
    pub fn get_most_recent_energy(&mut self) -> f64 {
        self.most_recent_energy
    }
    pub fn get_frame_size(&mut self) -> usize {
        self.vad.calculate_expected_frame_size()
    }
}

#[cfg(test)]
mod tests {}
