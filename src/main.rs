use api::database::db::DB;
use axum::extract::State;
use axum::Router;
use std::io;

mod api;

use crate::api::database::db;
use crate::api::resources;

type AppStateInner = &'static DB;
type AppState = State<AppStateInner>;

#[tokio::main]
async fn main() -> io::Result<()> {
    let db = db::setup().await.expect("Failed to set up MongoDB.");

    // It is fine to "leak" the state since it lives until the end of the program.
    let shared_db = Box::leak(Box::new(db));

    let app = Router::new()
        .nest("/", resources::ping::router())
        .nest("/user", resources::user::router())
        .with_state(shared_db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await
}
