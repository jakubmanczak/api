use std::env::VarError;
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

fn get_port() -> u16 {
    let portstr = match std::env::var("PORT") {
        Ok(port) => port,
        Err(e) => match e {
            VarError::NotPresent => {
                trace!("PORT environment variable not found");
                return 2004;
            }
            _ => {
                error!("{e}");
                panic!();
            }
        },
    };

    match portstr.parse() {
        Ok(num) => num,
        Err(e) => {
            error!("error while parsing environment variable PORT: {e}");
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
