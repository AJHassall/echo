use std::time::{Duration, Instant};

// Trait for VAD calculation
pub trait VadCalculator {
    fn calculate_energy(&self, frame: &[f32]) -> f64;
}

pub struct VoiceActivityDetector {
    audio: Vec<f32>,
}

impl VoiceActivityDetector {
    pub fn new() -> Self {
        VoiceActivityDetector { audio: Vec::new() }
    }
}

impl VadCalculator for VoiceActivityDetector {
    fn calculate_energy(&self, frame: &[f32]) -> f64 {
        frame.iter().map(|&sample| (sample as f64).powi(2)).sum()
    }
}

pub struct AudioChunkProcessor {
    vad: VoiceActivityDetector,
    output_chunks:Vec<Vec<f32>>,
    current_chunk: Vec<f32>,
    last_speech_time: Instant,
    last_silence_time: Instant,
    silence_threshold: f64,
    silence_duration_threshold: Duration,
    frame_size: usize,
    sample_rate: u32,
    silence_start_time: Option<Instant>, // New field to track silence start
}

impl AudioChunkProcessor {
    pub fn new(vad: VoiceActivityDetector, sample_rate: u32) -> Self {
        AudioChunkProcessor {
            vad,
            current_chunk: Vec::new(),
            last_speech_time: Instant::now(),
            last_silence_time: Instant::now(),
            silence_threshold: 0.01,
            silence_duration_threshold: Duration::from_secs(1),
            silence_start_time: None, // Initialize to None
            frame_size: 512,
            sample_rate,
            output_chunks: Vec::new()
        }
    }

    pub fn process_chunk(&mut self, audio_chunk: Vec<f32>){

        let energy = self.vad.calculate_energy(&audio_chunk);

        if energy > self.silence_threshold {
            println!("active");
            // Speech detected
            self.last_speech_time = Instant::now();
            self.silence_start_time = None; // Reset silence start time
            self.current_chunk.extend(audio_chunk);
        } else {
            println!("silence");
            // Silence detected.
            let now = Instant::now();

            if self.silence_start_time.is_none() {
                // First silent chunk, record start time
                self.silence_start_time = Some(now);
            }

            if let Some(silence_start) = self.silence_start_time {
                if now.duration_since(silence_start) >= self.silence_duration_threshold {
                    // Silence duration exceeded threshold
                    if !self.current_chunk.is_empty() {
                        self.output_chunks.push(self.current_chunk.clone());
                        self.current_chunk.clear();
                    }
                    self.silence_start_time = None; // Reset silence start time
                }
            }
            self.current_chunk.extend(audio_chunk);
        }

    }

    pub fn finalize(&mut self) -> Vec<Vec<f32>> { //Renamed to finalize
        if !self.current_chunk.is_empty() {
            self.output_chunks.push(self.current_chunk.clone());
            self.current_chunk.clear();
        }
        std::mem::take(&mut self.output_chunks) // return output_chunks
    }
}

#[cfg(test)]
mod tests {
    use super::*;


   
}