use crate::api::security::authentication::ExtractUser;
use crate::{api::models::response_models::MessageResponse, AppState};
use axum::{routing::get, Json, Router};

async fn get_ping(ExtractUser(_): ExtractUser) -> Json<MessageResponse> {
    let response = MessageResponse {
        message: "Pong".to_string(),
    };
    Json(response)
}

pub fn router() -> Router<AppState> {
    Router::<AppState>::new().route("/", get(get_ping))
}
