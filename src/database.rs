use std::env;

use sqlite::Connection;
use tracing::error;

pub fn initialise_sqlite_connection() -> Connection {
    let path = match env::var("DBPATH") {
        Ok(env) => env,
        Err(_) => "sqlite.db".to_owned(),
    };
    let conn = match sqlite::open(path) {
        Ok(conn) => conn,
        Err(e) => {
            error!("error establishing sqlite db connection: {e}");
            panic!();
        }
    };
    return conn;
}

pub fn execute_migration_queries() {
    let conn = initialise_sqlite_connection();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS splashes (
                id      TEXT NOT NULL UNIQUE PRIMARY KEY,
                splash  TEXT NOT NULL
            )",
    )
    .unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
                id      TEXT NOT NULL UNIQUE PRIMARY KEY,
                name    TEXT NOT NULL UNIQUE,
                pass    TEXT NOT NULL
        )",
    )
    .unwrap();
}
