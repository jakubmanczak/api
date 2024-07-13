use axum::{routing::get, Router};

pub fn route() -> Router {
    Router::new().route("/", get(root))
}

async fn root() -> () {}
