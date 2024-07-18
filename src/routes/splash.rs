use crate::database;
use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlite::State;
use tracing::{error, info};

pub fn route() -> Router {
    Router::new()
        .route("/splash", get(splash))
        .route("/splashes", get(splashes))
        .route("/splashes/:id", get(splash_by_id))
}

pub static NO_SPLASHES: &str = "No splashes found.";
pub static NO_SUCH_SPLASH: &str = "No such splash found.";

#[derive(Serialize)]
struct Splash {
    id: String,
    splash: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
enum ReturnFormat {
    PlainText,
    Json,
}

#[derive(Deserialize, Debug)]
struct SplashGetParams {
    format: Option<ReturnFormat>,
    exclude_id: Option<String>,
}

async fn splash(Query(params): Query<SplashGetParams>) -> Response {
    let conn = database::initialise_sqlite_connection();
    let mut statement = match params.exclude_id {
        Some(eid) => {
            let eid = eid.as_str();
            let q = "
                SELECT * FROM splashes
                WHERE id != :exclude_id
                ORDER BY RANDOM() LIMIT 1";
            let mut statement = conn.prepare(q).unwrap();
            statement.bind((":exclude_id", eid)).unwrap();

            statement
        }
        None => {
            let q = "SELECT * FROM splashes ORDER BY RANDOM() LIMIT 1";
            let statement = conn.prepare(q).unwrap();

            statement
        }
    };

    match statement.next() {
        Ok(State::Row) => match params.format {
            Some(ReturnFormat::Json) => {
                return Json(Splash {
                    id: statement.read("id").unwrap(),
                    splash: statement.read("splash").unwrap(),
                })
                .into_response()
            }
            Some(ReturnFormat::PlainText) | None => {
                let splash: String = statement.read("splash").unwrap();
                return splash.into_response();
            }
        },
        Ok(State::Done) => {
            info!("No splashes could be returned from GET /splash - none in database.");
            return (StatusCode::NOT_FOUND, NO_SPLASHES).into_response();
        }
        Err(e) => {
            error!("Returned 500 in GET /splash due to error: {e}");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    }
}

async fn splash_by_id(Path(id): Path<String>) -> Response {
    let conn = database::initialise_sqlite_connection();
    let query = "SELECT * FROM splashes WHERE id = :id";
    let mut statement = conn.prepare(query).unwrap();
    statement.bind((":id", id.as_str())).unwrap();

    match statement.next() {
        Ok(State::Row) => {
            return Json(Splash {
                id: statement.read("id").unwrap(),
                splash: statement.read("splash").unwrap(),
            })
            .into_response()
        }
        Ok(State::Done) => return (StatusCode::NOT_FOUND, NO_SUCH_SPLASH).into_response(),
        Err(e) => {
            error!("Returned 500 in GET /splashes/:id due to error: {e}");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    }
}

async fn splashes() -> Response {
    let conn = database::initialise_sqlite_connection();
    let query = "SELECT * FROM splashes LIMIT :limit";
    let mut statement = conn.prepare(query).unwrap();
    statement.bind((":limit", 200)).unwrap();

    let mut splashes: Vec<Splash> = Vec::new();
    loop {
        match statement.next() {
            Ok(State::Row) => splashes.push(Splash {
                id: statement.read("id").unwrap(),
                splash: statement.read("splash").unwrap(),
            }),
            Ok(State::Done) => match splashes.is_empty() {
                true => return (StatusCode::NOT_FOUND, NO_SPLASHES).into_response(),
                false => return Json(splashes).into_response(),
            },
            Err(e) => {
                error!("Returned 500 in GET /splashes due to error: {e}");
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
        }
    }
}
