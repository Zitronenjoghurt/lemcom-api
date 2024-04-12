use std::sync::Arc;
use axum::{Extension, Router};
use tokio::sync::Mutex;
mod api;
use crate::api::resources;
use crate::api::database::db;

#[tokio::main]
async fn main() {
    let db = db::setup().await.expect("Failed to set up MongoDB.");
    let shared_db = Arc::new(Mutex::new(db));

    let app = Router::new()
        .nest("/", resources::ping::router())
        .layer(Extension(shared_db));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}