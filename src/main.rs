use std::sync::Arc;
use axum::{middleware, Extension, Router};
use tokio::sync::RwLock;
mod api;
use crate::api::resources;
use crate::api::database::db;
use crate::api::utils::route_capture::capture_route;

#[tokio::main]
async fn main() {
    let db = db::setup().await.expect("Failed to set up MongoDB.");
    let shared_db = Arc::new(RwLock::new(db));

    let app = Router::new()
        .nest("/", resources::ping::router())
        .route_layer(middleware::from_fn(capture_route))
        .layer(Extension(shared_db));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}