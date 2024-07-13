use axum::{
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde::Serialize;

pub fn route() -> Router {
    Router::new()
        .route("/version", get(version))
        .route("/version-details", get(version_details))
        .route("/info", get(version_details))
}

#[derive(Serialize)]
struct VersionDetails {
    version: &'static str,
    version_bits: VersionBits,
    git_commit_hash: &'static str,
}

#[derive(Serialize)]
struct VersionBits {
    major: &'static str,
    minor: &'static str,
    patch: &'static str,
}

async fn version() -> Response {
    env!("CARGO_PKG_VERSION").into_response()
}

async fn version_details() -> Response {
    Json(VersionDetails {
        version: env!("CARGO_PKG_VERSION"),
        version_bits: VersionBits {
            major: env!("CARGO_PKG_VERSION_MAJOR"),
            minor: env!("CARGO_PKG_VERSION_MINOR"),
            patch: env!("CARGO_PKG_VERSION_PATCH"),
        },
        git_commit_hash: env!("GIT_HASH"),
    })
    .into_response()
}
