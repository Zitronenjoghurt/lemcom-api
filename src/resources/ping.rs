use crate::api::security::authentication::ExtractUser;
use crate::{api::models::response_models::MessageResponse, AppState};
use axum::{routing::get, Json, Router};

/// Ping the API for a response.
///
/// This endpoint returns a simple pong message to indicate that the API is responsive.
#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "Pong", body = MessageResponse),
        (status = 401, description = "Invalid API Key"),
    ),
    security(
        ("api_key" = [])
    ),
    tag = "Misc"
)]
async fn get_ping(ExtractUser(_): ExtractUser) -> Json<MessageResponse> {
    let response = MessageResponse {
        message: "Pong".to_string(),
    };
    Json(response)
}

pub fn router() -> Router<AppState> {
    Router::<AppState>::new().route("/", get(get_ping))
}
