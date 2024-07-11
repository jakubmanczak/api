use crate::setup;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlite::State;

pub fn route() -> Router {
    Router::new()
        .route("/splash", get(random_splash))
        .route("/splash-json", get(random_splash_json))
        .route("/splashes", get(all_splashes))
}

#[derive(Serialize, Deserialize)]
struct Splash {
    id: String,
    splash: String,
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

async fn random_splash_json() -> Response {
    let conn = setup::initialise_sqlite_connection();
    let query = "SELECT * FROM splashes ORDER BY RANDOM() LIMIT 1";

    let mut statement = conn.prepare(query).unwrap();

    while let Ok(State::Row) = statement.next() {
        let splash = Splash {
            id: statement.read::<String, _>("id").unwrap(),
            splash: statement.read::<String, _>("splash").unwrap(),
        };
        return Json(splash).into_response();
    }

    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
}

async fn all_splashes() -> Response {
    let conn = setup::initialise_sqlite_connection();
    let query = "SELECT * FROM splashes";

    let mut statement = conn.prepare(query).unwrap();
    let mut splashes: Vec<Splash> = Vec::new();

    while let Ok(State::Row) = statement.next() {
        splashes.push(Splash {
            id: statement.read::<String, _>("id").unwrap(),
            splash: statement.read::<String, _>("splash").unwrap(),
        });
    }

    if !splashes.is_empty() {
        return Json(splashes).into_response();
    }

    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
}
