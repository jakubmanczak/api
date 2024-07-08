use axum::{http::StatusCode, routing::get, Router};

pub fn route() -> Router {
    Router::new()
        .route("/brew-coffee", get(refuse_to_brew_coffee()))
        .route("/brew", get(refuse_to_brew_coffee()))
}

fn refuse_to_brew_coffee() -> StatusCode {
    StatusCode::IM_A_TEAPOT
}
