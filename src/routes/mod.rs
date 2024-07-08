use axum::Router;

pub mod health;
pub mod splash;
pub mod teapot;

pub fn routes() -> Router {
    Router::new()
        .merge(health::route())
        .merge(splash::route())
        .merge(teapot::route())
}
