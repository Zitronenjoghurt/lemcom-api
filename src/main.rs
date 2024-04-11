use std::sync::Arc;
use axum::{Extension, Router};
mod api;
use crate::api::resources;
use crate::api::database::db;

#[tokio::main]
async fn main() {
    let mongodb_client = db::setup().await.expect("Failed to set up Database client."); 
    let shared_client = Arc::new(mongodb_client);

    let app = Router::new()
        .nest("/ping", resources::ping::router())
        .layer(Extension(shared_client));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}