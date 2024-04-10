use axum::{
    routing::get,
    Router,
};

async fn get_ping() -> &'static str {
    "Pong"
}

pub fn router() -> Router {
    Router::new()
        .route("/", get(get_ping))
}