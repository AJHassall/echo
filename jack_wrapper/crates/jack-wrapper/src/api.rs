use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::error::Error as StdError;
use std::fmt;
use std::sync::{Arc, Mutex};

use crate::transcription_engine::{self, TranscriptionEngine};

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
) -> Result<TranscribeResponse, String> {
    println!("Processed audio length: {}", audio.len());

    let audio = resample(audio);

    println!("Processed audio length: {}", audio.len());

    engine.process_audio(&audio);

    let transcription = engine
        .get_segments()
        .map_err(|err| format!("Whisper error: {}", err))?
        .join(" ");


    println!("{}", transcription);
    return Ok(TranscribeResponse { transcription }); // Example success case
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
        SincFixedIn::<f32>::new(16000 as f64 / 48000 as f64, 2.0, params, waves_in.len(), 1)
            .unwrap();
    let input_channels: Vec<Vec<f32>> = vec![waves_in]; // Wrap input in a Vec<Vec<f32>>
    let waves_out = resampler.process(&input_channels, None).unwrap();

    waves_out.into_iter().flatten().collect() // Return the resampled data for the single channel
}
// fn base64_to_f32(base64_string: &str) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
//     // 1. Base64 Decode
//     let decoded_bytes = decode(base64_string)?;

//     // 2. Byte Array to f32 Conversion
//     if decoded_bytes.len() % 4 != 0 {
//         return Err("Decoded byte array length is not a multiple of 4".into());
//     }

//     let mut f32_vector = Vec::with_capacity(decoded_bytes.len() / 4);

//     for chunk in decoded_bytes.chunks_exact(4) {
//         let bytes: [u8; 4] = chunk.try_into()?; // Convert slice to array
//         let f32_value = f32::from_le_bytes(bytes); // Convert bytes to f32 (little-endian)
//         f32_vector.push(f32_value);
//     }

//     Ok(f32_vector)
// }
