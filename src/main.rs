use axum::Router;
use std::io;
mod api;
use crate::api::database::db;
use crate::api::resources;

#[derive(Clone)]
struct AppState {
    database: db::DB,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let db = db::setup().await.expect("Failed to set up MongoDB.");

    let app_state = AppState { database: db };

    let app = Router::<AppState>::new()
        .nest("/", resources::ping::router())
        .nest("/", resources::user::router())
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await
}
