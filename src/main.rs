use axum::Router;
use std::io;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;
mod api;
mod docs;
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
        .nest("/", resources::friend::router())
        .nest("/", resources::metrics::router())
        .nest("/", resources::notification::router())
        .nest("/", resources::ping::router())
        .nest("/", resources::timezone::router())
        .nest("/", resources::user::router())
        .nest("/", resources::users::router())
        .merge(SwaggerUi::new("/swagger").url("/api-docs/openapi.json", docs::ApiDoc::openapi()))
        .merge(Redoc::with_url("/redoc", docs::ApiDoc::openapi()))
        .merge(RapiDoc::new("/api-docs/openapi.json").path("/docs"))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await
}
