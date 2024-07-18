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
