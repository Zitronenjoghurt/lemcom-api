use axum::{
    Json, 
    routing::get, 
    Router
};
use crate::api::models::base_models::MessageResponse;
use crate::api::security::authentication::ExtractUser;

async fn get_ping(ExtractUser(_): ExtractUser) -> Json<MessageResponse> {
    let response = MessageResponse {
        message: "Pong".to_string(),
    };
    Json(response)
}

pub fn router() -> Router {
    Router::new()
        .route("/", get(get_ping))
}