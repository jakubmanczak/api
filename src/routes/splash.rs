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

static NO_SPLASHES: &str = "No splashes found.";
static NO_SUCH_SPLASH: &str = "No such splash found.";

#[derive(Serialize)]
struct Splash {
    id: String,
    splash: String,
}

async fn random_splash() -> Response {
    let conn = setup::initialise_sqlite_connection();
    let query = "SELECT splash FROM splashes ORDER BY RANDOM() LIMIT 1";

    let mut statement = conn.prepare(query).unwrap();

    match statement.next() {
        Ok(State::Row) => {
            let splash: String = statement.read("splash").unwrap();
            return splash.into_response();
        }
        Ok(State::Done) => return (StatusCode::NOT_FOUND, NO_SPLASHES).into_response(),
        Err(e) => {
            error!("Error on statement.next() /splash -> {e}");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    }
}

async fn random_splash_json() -> Response {
    let conn = setup::initialise_sqlite_connection();
    let query = "SELECT * FROM splashes ORDER BY RANDOM() LIMIT 1";

    let mut statement = conn.prepare(query).unwrap();

    match statement.next() {
        Ok(State::Row) => (),
        Ok(State::Done) => return (StatusCode::NOT_FOUND, NO_SPLASHES).into_response(),
        Err(e) => {
            error!("Error on statement.next() /splash-json -> {e}");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    }

    let splash = Splash {
        id: statement.read("id").unwrap(),
        splash: statement.read("splash").unwrap(),
    };
    return Json(splash).into_response();
}

async fn particular_splash(Path(id): Path<String>) -> Response {
    let conn = setup::initialise_sqlite_connection();
    let query = "SELECT * FROM splashes WHERE id = :id";

    let mut statement = conn.prepare(query).unwrap();
    statement.bind((":id", id.as_str())).unwrap();

    match statement.next() {
        Ok(State::Row) => (),
        Ok(State::Done) => return (StatusCode::NOT_FOUND, NO_SUCH_SPLASH).into_response(),
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

    loop {
        match statement.next() {
            Ok(State::Row) => (),
            Ok(State::Done) => match splashes.is_empty() {
                true => return (StatusCode::NOT_FOUND, NO_SPLASHES).into_response(),
                false => return Json(splashes).into_response(),
            },
            Err(e) => {
                error!("Error on statement.next() /splashes -> {e}");
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
        }

        splashes.push(Splash {
            id: statement.read("id").unwrap(),
            splash: statement.read("splash").unwrap(),
        });
    }
}
