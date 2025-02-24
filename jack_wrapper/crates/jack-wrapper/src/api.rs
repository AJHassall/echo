use serde::Serialize;
use whisper_rs::WhisperError;
use std::fmt;

use crate::transcription_engine::TranscriptionEngine;

// Define a custom error type
#[derive(Debug, Serialize)]
pub struct CustomError {
    pub message: String,
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[derive(Debug, Serialize)]
pub struct TranscribeResponse {
    pub transcription: String,
}

pub fn transcribe_stream(
    audio: Vec<f32>,
    engine: &mut TranscriptionEngine,
) -> Result<TranscribeResponse, WhisperError> {
    println!("Processed audio length: {}", audio.len());

    let audio = resample(audio);

    println!("Processed audio length: {}", audio.len());

    engine.process_audio(&audio)?;

    let transcription = engine
        .get_segments()?
        .join(" ");


    println!("{}", transcription);
    Ok(TranscribeResponse { transcription })// Example success case
}

fn resample(waves_in: Vec<f32>) -> Vec<f32> {
    use rubato::{
        Resampler, SincFixedIn, SincInterpolationParameters, SincInterpolationType, WindowFunction,
    };
    let params = SincInterpolationParameters {
        sinc_len: 256,
        f_cutoff: 0.95,
        interpolation: SincInterpolationType::Linear,
        oversampling_factor: 256,
        window: WindowFunction::BlackmanHarris2,
    };

    let mut resampler =
        SincFixedIn::<f32>::new(16000_f64 / 48000_f64, 2.0, params, waves_in.len(), 1)
            .unwrap();
    let input_channels: Vec<Vec<f32>> = vec![waves_in]; // Wrap input in a Vec<Vec<f32>>
    let waves_out = resampler.process(&input_channels, None).unwrap();

    waves_out.into_iter().flatten().collect() // Return the resampled data for the single channel
}
