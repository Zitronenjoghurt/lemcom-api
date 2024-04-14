use api::database::db::DB;
use axum::extract::State;
use axum::{middleware, Router};
use tokio::sync::RwLock;

mod api;

use crate::api::database::db;
use crate::api::resources;
use crate::api::utils::route_capture::capture_route;

type AppStateInner = &'static RwLock<DB>;
type AppState = State<AppStateInner>;

#[tokio::main]
async fn main() {
    let db = db::setup().await.expect("Failed to set up MongoDB.");

    // It is fine to "leak" the state since it lives until the end of the program.
    let shared_db = Box::leak(Box::new(RwLock::new(db)));

    let app = Router::new()
        .nest("/", resources::ping::router())
        .nest("/user", resources::user::router())
        .route_layer(middleware::from_fn(capture_route))
        .with_state(shared_db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
