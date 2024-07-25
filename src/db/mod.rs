use sqlite::Connection;
use std::env;
use tracing::error;

mod tables;

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
    use tables::*;

    let conn = initialise_sqlite_connection();
    for query in [USERS, SPLASHES] {
        conn.execute(query).unwrap();
    }
}
