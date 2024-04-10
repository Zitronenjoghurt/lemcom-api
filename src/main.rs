use axum::Router;

mod resources {
    pub mod ping;
}

#[tokio::main]
async fn main() {
    let ping_router: Router = resources::ping::router();

    let app = Router::new()
        .nest("/ping", ping_router);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}