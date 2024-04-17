use crate::api::security::authentication::ExtractUser;
use crate::AppState;
use axum::{routing::get, Json, Router};
use std::collections::HashMap;

/// Retrieve usage data.
///
/// This endpoint returns information about how often you used which endpoint.
#[utoipa::path(
    get,
    path = "/metrics/usage",
    responses(
        (status = 200, description = "Endpoint usage data", body = HashMap<String, u64>),
        (status = 401, description = "Invalid API Key"),
    ),
    security(
        ("api_key" = [])
    ),
    tag = "Misc"
)]
async fn get_metrics_usage(ExtractUser(user): ExtractUser) -> Json<HashMap<String, u64>> {
    Json(user.endpoint_usage)
}

pub fn router() -> Router<AppState> {
    Router::<AppState>::new().route("/metrics/usage", get(get_metrics_usage))
}
