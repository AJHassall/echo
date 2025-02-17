mod transcription_engine;
mod api;

use actix_web::{web, App, HttpServer}; // Actix Web for API
use std::sync::{Arc, Mutex}; // For shared state
use std::fs::File;
use std::io::Read;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let model_path = "whisper-models/ggml-base.bin";
    let transcription_engine = Arc::new(Mutex::new(
        transcription_engine::TranscriptionEngine::new(model_path).expect("error loading model"),
    )); // Shared state

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(transcription_engine.clone())) // Share the state
            .service(api::transcribe) // Mount the API endpoint
    })
    .bind("127.0.0.1:8080")? // Bind to port 8080
    .run()
    .await
}