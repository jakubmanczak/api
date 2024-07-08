use axum::{routing::get, Router};

pub fn randsplash_route() -> Router {
    Router::new().route("/splash", get(random_splash()))
}

fn random_splash() -> String {
    "Hello".to_owned()
}
