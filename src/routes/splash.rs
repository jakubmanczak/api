use crate::setup;
use axum::{http::StatusCode, routing::get, Router};
use sqlite::State;

pub fn route() -> Router {
    Router::new().route("/splash", get(random_splash))
}

async fn random_splash() -> (StatusCode, String) {
    let conn = setup::initialise_sqlite_connection();
    let query = "SELECT splash FROM splashes ORDER BY RANDOM() LIMIT 1";

    let mut statement = conn.prepare(query).unwrap();

    while let Ok(State::Row) = statement.next() {
        return (StatusCode::OK, statement.read::<String, _>(0).unwrap());
    }

    return (StatusCode::INTERNAL_SERVER_ERROR, "".to_owned());
}
