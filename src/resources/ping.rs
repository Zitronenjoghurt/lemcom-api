use axum::{ Json, routing::get, Router };
use crate::models::base_models::MessageResponse;

async fn get_ping() -> Json<MessageResponse> {
    let response = MessageResponse {
        message: "Pong".to_string(),
    };
    Json(response)
}

pub fn router() -> Router {
    Router::new()
        .route("/", get(get_ping))
}