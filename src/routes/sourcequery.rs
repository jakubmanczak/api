use a2s::{info::ServerOS, A2SClient};
use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde::Serialize;
use std::net::Ipv4Addr;

#[derive(Serialize)]
struct ServerInfoResult {
    hostname: String,
    map: String,
    server_os: String,
    game_version: String,
    maxplayers: u8,
    players: u8,
    bots: u8,
    vac: bool,
    passworded: bool,
    playerlist: Option<Vec<ServerPlayer>>,
}
#[derive(Serialize)]
struct ServerPlayer {
    name: String,
    score: i32,
    duration: f32,
}

pub fn route() -> Router {
    Router::new().route("/tf2-server-info/:ip", get(tfserverinfo))
}

async fn tfserverinfo(Path(ip): Path<Ipv4Addr>) -> Response {
    let client = match A2SClient::new() {
        Ok(cl) => cl,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };
    let addr = (ip.to_string(), 27015);
    let info = match client.info(addr.clone()) {
        Ok(info) => info,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };
    let players = match client.players(addr.clone()) {
        Ok(p) => p,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    let ret = ServerInfoResult {
        hostname: info.name,
        map: info.map,
        server_os: match info.server_os {
            ServerOS::Windows => "Windows",
            ServerOS::Linux => "Linux",
            ServerOS::Mac => "Mac",
        }
        .to_string(),
        game_version: info.version,
        maxplayers: info.max_players,
        players: info.players,
        bots: info.bots,
        vac: info.vac,
        passworded: info.visibility,
        playerlist: Some(
            players
                .into_iter()
                .map(|p| ServerPlayer {
                    name: p.name,
                    score: p.score,
                    duration: p.duration,
                })
                .collect(),
        ),
    };

    (StatusCode::OK, Json(ret)).into_response()
}
