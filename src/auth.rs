use axum::http::{header::AUTHORIZATION, HeaderMap, StatusCode};
use base64::{engine::general_purpose, Engine as _};
use sqlite::State;
use tracing::error;

use crate::db;

pub struct BasicAuth {
    pub username: String,
    pub password: String,
}

pub fn get_basic_auth_from_headers(headers: &HeaderMap) -> Option<BasicAuth> {
    let encoded = match headers.contains_key(AUTHORIZATION) {
        true => match String::from_utf8(headers.get(AUTHORIZATION).unwrap().as_bytes().to_vec()) {
            Ok(authstr) => match authstr.strip_prefix("Basic ") {
                Some(stripped) => stripped.to_string(),
                None => return None,
            },
            Err(e) => {
                let msg = "Failed to parse string from AUTHORIZATION header!";
                error!(
                    "{msg} -> error: {e} offending header: {:?}",
                    headers.get(AUTHORIZATION).unwrap()
                );
                return None;
            }
        },
        false => return None,
    };
    let decoded = match general_purpose::STANDARD.decode(encoded) {
        Ok(vector) => match String::from_utf8(vector) {
            Ok(string) => string,
            Err(e) => {
                error!("could not parse decoded bytes from string: {e}");
                return None;
            }
        },
        Err(e) => {
            error!("could not decode base64 auth: {e}");
            return None;
        }
    };

    let split: Vec<&str> = decoded.split(":").collect();
    match split.len() {
        2 => (),
        _ => return None,
    }

    Some(BasicAuth {
        username: split[0].to_owned(),
        password: split[1].to_owned(),
    })
}

pub fn validate_password_hash_from_basic_auth(auth: &BasicAuth) -> StatusCode {
    let conn = db::initialise_sqlite_connection();
    let query = "SELECT pass FROM users WHERE name = :name";
    let mut statement = conn.prepare(query).unwrap();
    statement.bind((":name", auth.username.as_str())).unwrap();

    match statement.next() {
        Ok(State::Row) => (),
        Ok(State::Done) => return StatusCode::UNAUTHORIZED,
        Err(e) => {
            error!("Returned 500 in POST /splashes due to error : {e}");
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    }

    let hash: String = statement.read("pass").unwrap();

    match argon2::verify_encoded(hash.as_str(), auth.password.as_bytes()) {
        Ok(bool) => match bool {
            true => return StatusCode::OK,
            false => return StatusCode::UNAUTHORIZED,
        },
        Err(e) => {
            error!("Returned 500 in POST /splashes due to error: {e}");
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    }
}
