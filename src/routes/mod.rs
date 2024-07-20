use axum::Router;

mod argon;
mod health;
mod root;
mod splash;
mod teapot;
mod version;

pub fn routes() -> Router {
    Router::new()
        // .merge(argon::route())
        .merge(health::route())
        .merge(root::route())
        .merge(splash::route())
        .merge(teapot::route())
        .merge(version::route())
}
