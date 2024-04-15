use crate::api::models::query_models::UserSettingsEdit;
use crate::api::models::user_settings::UserSettings;
use crate::api::models::{query_models::UserName, response_models::UserPrivateInformation, user};
use crate::api::security::authentication::ExtractUser;
use crate::AppState;
use axum::extract::State;
use axum::response::Response;
use axum::routing::patch;
use axum::{extract::Query, http::StatusCode, response::IntoResponse, routing::get, Json, Router};

async fn get_user(ExtractUser(user): ExtractUser) -> Json<UserPrivateInformation> {
    Json(user.private_information())
}

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

async fn get_user_settings(ExtractUser(user): ExtractUser) -> Json<UserSettings> {
    Json(user.settings)
}

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
