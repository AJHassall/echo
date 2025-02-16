mod api; // Import the api module
mod model; // Import the model module
//mod utils;

use actix_web::{App, HttpServer};
use api::{AppState, configure_api}; // Use the api config and AppState

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Create the model
    let model = model::create_model();

    // Create AppState
    let app_state = web::Data::new(AppState { model });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(configure_api) // Configure the API routes
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}