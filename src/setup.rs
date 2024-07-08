use sqlite::Connection;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use tokio::net::TcpListener;
use tracing::Level;
use tracing::{error, info, trace};
use tracing_subscriber::FmtSubscriber;

pub fn initialise_logging() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    info!("jakubmanczak/api says hello - tracing crate initialised");
}

pub fn initialise_dotenv() {
    match dotenvy::dotenv() {
        Ok(_) => info!("loaded .env"),
        Err(e) => {
            if e.not_found() {
                trace!(".env file not found; skipping...");
            } else {
                error!("error while loading .env: {e}");
            }
        }
    };
}
pub fn initialise_sqlite_connection() -> Connection {
    let conn = match sqlite::open("sqlite.db") {
        Ok(conn) => conn,
        Err(e) => {
            error!("error establishing sqlite db connection: {e}");
            panic!();
        }
    };
    return conn;
}

pub fn initialise_sqlite_db_tables() {
    let conn = initialise_sqlite_connection();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS splashes (
            id      TEXT NOT NULL UNIQUE PRIMARY KEY,
            splash  TEXT NOT NULL
        )",
    )
    .unwrap();
}

fn get_port() -> u16 {
    let portstr = match std::env::var("PORT") {
        Ok(port) => port,
        Err(_) => "2004".to_owned(),
    };

    match portstr.parse() {
        Ok(num) => num,
        Err(e) => {
            error!("error while parsing environment variable PORT from str to u16: {e}");
            panic!();
        }
    }
}

pub fn get_socket_addr() -> SocketAddrV4 {
    let addr = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), get_port());
    trace!("desired socket address is {}", addr.to_string());
    return addr;
}

pub fn get_listener_socket_addr(listener: &TcpListener) -> SocketAddr {
    let addr = match listener.local_addr() {
        Ok(addr) => addr,
        Err(e) => {
            error!("error while getting listener socket address: {e}");
            panic!();
        }
    };
    return addr;
}
