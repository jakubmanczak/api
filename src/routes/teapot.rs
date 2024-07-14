use axum::{http::StatusCode, routing::get, Router};

pub fn route() -> Router {
    Router::new()
        .route("/brew-coffee", get(refuse_to_brew_coffee()))
        .route("/brew", get(refuse_to_brew_coffee()))
}

static TEAPOT_RESPONSE: &str = "I'm a teapot.";

fn refuse_to_brew_coffee() -> (StatusCode, &'static str) {
    (StatusCode::IM_A_TEAPOT, TEAPOT_RESPONSE)
}
