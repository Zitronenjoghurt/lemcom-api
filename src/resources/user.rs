use crate::api::models::{query_models::UserName, response_models::UserPrivateInformation, user};
use crate::api::security::authentication::ExtractUser;
use crate::{AppState, AppStateInner};
use axum::extract::State;
use axum::{extract::Query, http::StatusCode, response::IntoResponse, routing::get, Json, Router};

async fn get_user(ExtractUser(user): ExtractUser) -> Json<UserPrivateInformation> {
    Json(user.private_information())
}

async fn get_user_search(
    ExtractUser(_): ExtractUser,
    State(state): AppState,
    query: Query<UserName>,
) -> impl IntoResponse {
    let query = query.sanitize();
    match user::find_user_by_name(&state.user_collection, &query.name).await {
        Ok(Some(user)) => Json(user.public_information()).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "User not found").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    }
}

pub fn router() -> Router<AppStateInner> {
    Router::new()
        .route("/", get(get_user))
        .route("/search", get(get_user_search))
}
