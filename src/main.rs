mod routes;
mod setup;
use axum::Router;
use tokio::net::TcpListener;
use tracing::{error, info};

#[tokio::main]
async fn main() {
    setup::initialise_logging();
    setup::initialise_dotenv();
    setup::initialise_sqlite_db_tables();

    let app = Router::new()
        .merge(routes::health::route())
        .merge(routes::splash::randsplash_route());

    let addr = setup::get_socket_addr();
    let listener = match TcpListener::bind(&addr).await {
        Ok(listener) => listener,
        Err(e) => {
            error!("error creating a listener: {e}");
            panic!();
        }
    };
    let addr = setup::get_listener_socket_addr(&listener);
    info!("listener socket addr is {}", addr.to_string());

    match axum::serve(listener, app).await {
        Ok(..) => info!("now serving!"),
        Err(e) => {
            error!("axum could not serve listener & app: {e}");
            panic!();
        }
    }
}
