use axum::Router;

mod health;
mod root;
mod sourcequery;
mod splash;
mod teapot;
mod version;

pub fn routes() -> Router {
    Router::new()
        .merge(health::route())
        .merge(root::route())
        .merge(splash::route())
        .merge(teapot::route())
        .merge(version::route())
        .merge(sourcequery::route())
}
