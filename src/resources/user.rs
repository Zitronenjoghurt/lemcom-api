use std::sync::Arc;

use axum::{Extension, extract::Query, Json, response::IntoResponse, routing::get, Router, http::StatusCode};
use tokio::sync::RwLock;
use crate::api::database::db::DB;
use crate::api::models::{user, response_models::UserPrivateInformation, query_models::UserName};
use crate::api::security::authentication::ExtractUser;

async fn get_user(ExtractUser(user): ExtractUser) -> Json<UserPrivateInformation> {
    Json(user.private_information())
}

async fn get_user_search(
    ExtractUser(_): ExtractUser,
    Extension(db): Extension<Arc<RwLock<DB>>>,
    query: Query<UserName>
) -> impl IntoResponse {
    let db = db.read().await;
    match user::find_user_by_name(&db.user_collection, &query.name).await {
        Ok(Some(user)) => (StatusCode::OK, Json(user.public_information())).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "User not found").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    }
}

pub fn router() -> Router {
    Router::new()
        .route("/", get(get_user))
        .route("/search", get(get_user_search))
}