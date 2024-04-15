use crate::api::models::response_models::MessageResponse;
use crate::api::security::authentication::ExtractUser;
use axum::{routing::get, Json, Router};

async fn get_ping(ExtractUser(_): ExtractUser) -> Json<MessageResponse> {
    let response = MessageResponse {
        message: "Pong".to_string(),
    };
    Json(response)
}

pub fn router() -> Router {
    Router::new().route("/", get(get_ping))
}
