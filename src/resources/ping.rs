use axum::{
    Json, 
    routing::get, 
    Router
};
use crate::models::base_models::MessageResponse;
use crate::security::authentication::ExtractUser;

async fn get_ping(ExtractUser(user): ExtractUser) -> Json<MessageResponse> {
    let response = MessageResponse {
        message: user.key.to_string(),
    };
    Json(response)
}

pub fn router() -> Router {
    Router::new()
        .route("/", get(get_ping))
}