use crate::api::models::query_models::TimezoneQuery;
use crate::api::utils::time_operations::{get_timezone_from_name, get_timezone_names};
use crate::{api::security::authentication::ExtractUser, AppState};
use crate::{unpack_option, unpack_result};
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::put;
use axum::{routing::get, Json, Router};

/// Get all available timezones.
// region: get_timezone
/// This endpoint returns all available timezones.
#[utoipa::path(
    get,
    path = "/timezone",
    responses(
        (status = 200, description = "All available timezones", body = Vec<String>),
        (status = 401, description = "Invalid API Key"),
    ),
    security(
        ("api_key" = [])
    ),
    tag = "Misc"
)]
async fn get_timezone(ExtractUser(_): ExtractUser) -> Json<Vec<String>> {
    Json(get_timezone_names())
}
// endregion: get_timezone

/// Customize your timezone.
// region: put_timezone
/// This endpoint returns all available timezones.
#[utoipa::path(
    put,
    path = "/timezone",
    params(TimezoneQuery),
    responses(
        (status = 200, description = "Timezone updated"),
        (status = 401, description = "Invalid API Key"),
        (status = 404, description = "Timezone not found"),
    ),
    security(
        ("api_key" = [])
    ),
    tag = "Misc"
)]
async fn put_timezone(
    ExtractUser(mut user): ExtractUser,
    State(state): State<AppState>,
    tz_query: Query<TimezoneQuery>,
) -> Response {
    let tz_name = &tz_query.timezone;
    let timezone = unpack_option!(
        get_timezone_from_name(tz_name),
        StatusCode::NOT_FOUND,
        "Timezone not found"
    );

    user.timezone = *timezone;
    unpack_result!(
        user.save(&state.database.user_collection).await,
        "An error occured while saving user"
    );

    Json((StatusCode::OK, "Timezone updated")).into_response()
}
// endregion: put_timezone

pub fn router() -> Router<AppState> {
    Router::<AppState>::new()
        .route("/timezone", get(get_timezone))
        .route("/timezone", put(put_timezone))
}
