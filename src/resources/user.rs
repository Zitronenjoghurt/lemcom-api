use std::sync::Arc;

use crate::api::database::db::DB;
use crate::api::models::{query_models::UserName, response_models::UserPrivateInformation, user};
use crate::api::security::authentication::ExtractUser;
use axum::{
    extract::Query, http::StatusCode, response::IntoResponse, routing::get, Extension, Json, Router,
};
use tokio::sync::RwLock;

async fn get_user(ExtractUser(user): ExtractUser) -> Json<UserPrivateInformation> {
    Json(user.private_information())
}

async fn get_user_search(
    ExtractUser(_): ExtractUser,
    Extension(db): Extension<Arc<RwLock<DB>>>,
    query: Query<UserName>,
) -> impl IntoResponse {
    let query = query.sanitize();
    let db = db.read().await;
    match user::find_user_by_name(&db.user_collection, &query.name).await {
        Ok(Some(user)) => Json(user.public_information()).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "User not found").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    }
}

pub fn router() -> Router {
    Router::new()
        .route("/", get(get_user))
        .route("/search", get(get_user_search))
}
