use axum::http;

pub async fn health() -> http::StatusCode {
    http::StatusCode::OK
}
