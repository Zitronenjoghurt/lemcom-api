use axum::Router;
mod api;
use crate::api::resources;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest("/ping", resources::ping::router());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}