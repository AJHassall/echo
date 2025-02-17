use actix_web::{post, web, Error, HttpResponse, http::StatusCode, ResponseError};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::fmt;
use std::error::Error as StdError;

use crate::transcription_engine;


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

impl StdError for CustomError {}

impl ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(StatusCode::BAD_REQUEST).json(self)
    }
}

#[derive(Debug, Deserialize)]
pub struct TranscribeRequest {
    pub audio: String,
}

#[derive(Debug, Serialize)]
pub struct TranscribeResponse {
    pub transcription: String,
}

#[post("/transcribe")]
async fn transcribe(
    req: web::Json<TranscribeRequest>,
    engine: web::Data<Arc<Mutex<transcription_engine::TranscriptionEngine>>>,
) -> Result<HttpResponse, Error> {
    let audio_bytes = base64::decode(&req.audio).map_err(|err| {
        Error::from(CustomError {
            message: format!("Base64 decoding error: {}", err),
        })
    })?;


    let samples: Vec<i16> = audio_bytes
        .chunks_exact(2)
        .map(|chunk| i16::from_le_bytes([chunk[0], chunk[1]]))
        .collect();

    let mut audio = vec![0.0f32; samples.len().try_into().unwrap()];

    whisper_rs::convert_integer_to_float_audio(&samples, &mut audio).expect("Conversion error");

    let mut engine_lock = engine.lock().unwrap();
    engine_lock.process_audio(audio.as_slice()).expect("error processing audio");

    let segments = engine_lock.get_segments().map_err(|err| {
        Error::from(CustomError {
            message: format!("Whisper error: {}", err),
        })
    })?;

    let transcription = segments.join(" ");

    Ok(HttpResponse::Ok().json(TranscribeResponse { transcription }))
}