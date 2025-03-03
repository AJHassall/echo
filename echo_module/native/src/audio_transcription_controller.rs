use serde::Serialize;
use std::fmt;
use whisper_rs::WhisperError;

use crate::transcription_engine::TranscriptionEngine;

// Define a custom error type
#[derive(Debug)]
pub enum CustomError {
    Whisper(WhisperError),
    TranscriptionInProgress,
    EngineNotInitialized,
}

impl std::error::Error for CustomError {}

// Add this implementation
impl From<WhisperError> for CustomError {
    fn from(err: WhisperError) -> Self {
        CustomError::Whisper(err)
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustomError::Whisper(e) => write!(f, "Whisper error: {}", e),
            CustomError::TranscriptionInProgress => write!(f, "Transcription in progress"),
            CustomError::EngineNotInitialized => write!(f, "Engine not initialized"),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TranscribeResponse {
    pub transcription: String,
}

pub trait Transcriber {
    fn transcribe(&mut self, audio: Vec<f32>) -> Result<TranscribeResponse, CustomError>;
}

impl Transcriber for TranscriptionEngine {
    fn transcribe(&mut self, audio: Vec<f32>) -> Result<TranscribeResponse, CustomError> {
        let audio = resample(audio);
        self.process_audio(&audio)?;

        let transcription = self.get_segments()?.join(" ");

        Ok(TranscribeResponse { transcription })
    }
}

pub struct ConcurrentTranscriber<T: Transcriber> {
    inner: T,
}

impl<T: Transcriber> ConcurrentTranscriber<T> {
    pub fn new(inner: T) -> Self {
        ConcurrentTranscriber { inner }
    }
}

impl<T: Transcriber> Transcriber for ConcurrentTranscriber<T> {
    fn transcribe(&mut self, audio: Vec<f32>) -> Result<TranscribeResponse, CustomError> {
        self.inner.transcribe(audio)
    }
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
        SincFixedIn::<f32>::new(16000_f64 / 48000_f64, 2.0, params, waves_in.len(), 1).unwrap();
    let input_channels: Vec<Vec<f32>> = vec![waves_in];
    let waves_out = resampler.process(&input_channels, None).unwrap();

    waves_out.into_iter().flatten().collect()
}
