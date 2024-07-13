use crate::setup;
use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde::Serialize;
use sqlite::State;
use tracing::error;

pub fn route() -> Router {
    Router::new()
        .route("/splash", get(random_splash))
        .route("/splash/:id", get(particular_splash))
        .route("/splash-json", get(random_splash_json))
        .route("/splashes", get(all_splashes))
}

#[derive(Serialize)]
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

async fn particular_splash(Path(id): Path<String>) -> Response {
    let conn = setup::initialise_sqlite_connection();
    let query = "SELECT * FROM splashes WHERE id = :id";

    let mut statement = conn.prepare(query).unwrap();
    statement.bind((":id", id.as_str())).unwrap();

    match statement.next() {
        Ok(State::Row) => (),
        Ok(State::Done) => return (StatusCode::NOT_FOUND).into_response(),
        Err(e) => {
            error!("Error on statement.next() /splash/{id} -> {e}");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let splash = Splash {
        id: statement.read("id").unwrap(),
        splash: statement.read("splash").unwrap(),
    };
    return Json(splash).into_response();
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
