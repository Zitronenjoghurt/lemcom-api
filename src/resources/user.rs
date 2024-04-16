use crate::api::models::query_models::UserSettingsEdit;
use crate::api::models::user_settings::UserSettings;
use crate::api::models::{query_models::UserName, response_models::UserPrivateInformation, user};
use crate::api::security::authentication::ExtractUser;
use crate::AppState;
use axum::extract::State;
use axum::response::Response;
use axum::routing::patch;
use axum::{extract::Query, http::StatusCode, response::IntoResponse, routing::get, Json, Router};

/// Retrieve own user information.
///
/// This endpoint returns your private user information.
#[utoipa::path(
    get,
    path = "/user",
    responses(
        (status = 200, description = "Personal private user information", body = UserPrivateInformation),
        (status = 400, description = "Missing API Key or Invalid API Key header format"),
        (status = 401, description = "Invalid API Key"),
    ),
    security(
        ("api_key" = [])
    ),
    tag = "User"
)]
async fn get_user(ExtractUser(user): ExtractUser) -> Json<UserPrivateInformation> {
    Json(user.private_information())
}

/// Retrieve public user information.
///
/// This endpoint returns the user information of the specified username, should they exist.
#[utoipa::path(
    get,
    path = "/user/search",
    params(UserName),
    responses(
        (status = 200, description = "Personal private user information", body = UserPrivateInformation),
        (status = 400, description = "Missing API Key or Invalid API Key header format"),
        (status = 401, description = "Invalid API Key"),
        (status = 404, description = "User does not exist"),
    ),
    security(
        ("api_key" = [])
    ),
    tag = "User"
)]
async fn get_user_search(
    ExtractUser(_): ExtractUser,
    State(state): State<AppState>,
    query: Query<UserName>,
) -> Response {
    let query = query.sanitize();
    match user::find_user_by_name(&state.database.user_collection, &query.name).await {
        Ok(Some(user)) => Json(user.public_information()).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "User not found").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    }
}

/// Retrieve own user settings.
///
/// This endpoint returns your user settings.
#[utoipa::path(
    get,
    path = "/user/settings",
    responses(
        (status = 200, description = "Your user settings", body = UserSettings),
        (status = 400, description = "Missing API Key or Invalid API Key header format"),
        (status = 401, description = "Invalid API Key"),
    ),
    security(
        ("api_key" = [])
    ),
    tag = "User"
)]
async fn get_user_settings(ExtractUser(user): ExtractUser) -> Json<UserSettings> {
    Json(user.settings)
}

/// Edit own user settings.
///
/// This endpoint allows you to edit your own user settings.
#[utoipa::path(
    patch,
    path = "/user/settings",
    params(UserSettingsEdit),
    responses(
        (status = 204, description = "Successfully changed"),
        (status = 400, description = "Missing API Key or Invalid API Key header format"),
        (status = 401, description = "Invalid API Key"),
    ),
    security(
        ("api_key" = [])
    ),
    tag = "User"
)]
async fn patch_user_settings(
    ExtractUser(mut user): ExtractUser,
    State(state): State<AppState>,
    query: Query<UserSettingsEdit>,
) -> Response {
    user.settings.update(query);
    match user.save(&state.database.user_collection).await {
        Ok(_) => (StatusCode::NO_CONTENT, "").into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to save user settings",
        )
            .into_response(),
    }
}

pub fn router() -> Router<AppState> {
    Router::<AppState>::new()
        .route("/user", get(get_user))
        .route("/user/search", get(get_user_search))
        .route("/user/settings", get(get_user_settings))
        .route("/user/settings", patch(patch_user_settings))
}
