use actix_web::{web, Error, HttpResponse, Responder};
use futures_util::{StreamExt, SinkExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio_tungstenite::tungstenite::Message;

use crate::model::YourModel; // Import the model
use crate::utils::{preprocess_text, postprocess_predictions}; // Import utils

#[derive(Deserialize, Serialize)]
pub struct PredictionRequest {
    input_text: String,
    // ... other input fields
}

async fn websocket_route(
    req: web::HttpRequest,
    data: web::Data<AppState>,
) -> Result<impl Responder, Error> {
    // ... (WebSocket handling logic - same as before, but using the imported model) ...
    let upgraded = web::ws::start(req)?;
    let model = data.model.clone();

    Ok(HttpResponse::Ok().streaming(upgraded.map_err(Error::from).map(move |msg| {
        match msg {
            Ok(Message::Text(text)) => {
                let request: PredictionRequest = match serde_json::from_str(&text) {
                    Ok(req) => req,
                    Err(e) => return Ok(Message::Text(format!("Error parsing request: {}", e))),
                };

                let processed_input = preprocess_text(&request.input_text);

                let prediction = {
                    let model_guard = model.lock().unwrap();
                    model_guard.predict(&processed_input)
                };

                let output = postprocess_predictions(&prediction);
                Ok(Message::Text(serde_json::to_string(&output).unwrap()))
            }
            Ok(Message::Close(_)) => {
                println!("Client disconnected");
                Ok(Message::Close(None))
            }
            _ => Ok(Message::Text("Unsupported message type".to_string())),
        }
    })))
}

async fn predict_route(
    data: web::Data<AppState>,
    req: web::Json<PredictionRequest>,
) -> Result<impl Responder, Error> {
    let input_data = &req.input_text;
    let processed_input = preprocess_text(input_data);

    let prediction = {
        let model_guard = data.model.lock().unwrap();
        model_guard.predict(&processed_input)
    };

    let output = postprocess_predictions(&prediction);

    Ok(HttpResponse::Ok().json(output))
}

pub struct AppState {
    pub model: Arc<std::sync::Mutex<YourModel>>,
}

// Function to configure the API routes
pub fn configure_api(cfg: &mut web::ServiceConfig) {
    cfg.route("/predict", web::post().to(predict_route))
        .route("/ws", web::get().to(websocket_route));
}