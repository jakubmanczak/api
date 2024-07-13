use axum::Router;

mod health;
mod splash;
mod teapot;
mod version;

pub fn routes() -> Router {
    Router::new()
        .merge(health::route())
        .merge(splash::route())
        .merge(teapot::route())
        .merge(version::route())
}
