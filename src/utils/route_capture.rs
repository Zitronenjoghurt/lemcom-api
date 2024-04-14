use axum::{
    http::{Method, Request},
    middleware::Next,
    response::Response,
};

pub async fn capture_route(req: Request<axum::body::Body>, next: Next) -> Response {
    let path = req.uri().path().to_owned();
    let method = req.method().clone();
    let mut req = req;
    req.extensions_mut().insert(RoutePath { method, path });
    next.run(req).await
}

#[derive(Clone)]
pub struct RoutePath {
    method: Method,
    path: String,
}

impl RoutePath {
    pub fn as_str(&self) -> String {
        format!("{} {}", self.method, self.path)
    }
}
