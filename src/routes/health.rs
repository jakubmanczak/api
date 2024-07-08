use axum::{http::StatusCode, routing::get, Router};

pub fn route() -> Router {
    Router::new().route("/health", get(health()))
}

fn health() -> StatusCode {
    StatusCode::OK
}
