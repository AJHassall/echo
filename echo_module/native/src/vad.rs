use std::time::{self, Duration, Instant};

// Trait for VAD calculation
pub trait VadCalculator {
    fn calculate_energy(&self, frame: &[f32]) -> f64;
}

pub struct VoiceActivityDetector;

impl VoiceActivityDetector {
    pub fn new() -> Self {
        VoiceActivityDetector { }
    }
}

impl VadCalculator for VoiceActivityDetector {
    fn calculate_energy(&self, frame: &[f32]) -> f64 {
        frame.iter().map(|&sample| (sample as f64).powi(2)).sum()
    }
}

pub struct AudioChunkProcessor {
    vad: VoiceActivityDetector,
    output_chunks: Vec<Vec<f32>>,
    current_chunk: Vec<f32>,
    last_silence_time: Instant,
    silence_threshold: f64,
    silence_duration_threshold: Duration,
}
impl AudioChunkProcessor {
    pub fn new(vad: VoiceActivityDetector) -> Self {
        AudioChunkProcessor {
            vad,
            current_chunk: Vec::new(),
            last_silence_time: Instant::now(),
            silence_threshold: 0.01,
            silence_duration_threshold: Duration::from_secs(1),

            output_chunks: Vec::new(),
        }
    }

    pub fn process_chunk(&mut self, audio_chunk: Vec<f32>) {
        let energy = self.vad.calculate_energy(&audio_chunk);

        if energy > self.silence_threshold {
            self.current_chunk.extend(audio_chunk);
            self.last_silence_time = time::Instant::now();
        } else if std::time::Instant::now().duration_since(self.last_silence_time) >= self.silence_duration_threshold && !self.current_chunk.is_empty() {
            self.output_chunks.push(self.current_chunk.clone());
            self.current_chunk.clear();
        }
    }

    pub fn get_current_audio(&mut self) -> Vec<Vec<f32>> {
        let out = self.output_chunks.clone();
        self.output_chunks.clear();
        out
    }

}

#[cfg(test)]
mod tests {}
