#[macro_use]
extern crate log;

use std::{env, io::Error};

mod http_client;
mod http_server;
mod ws_engine;

use http_client::LocalHttpClient;
use http_server::*;
use ws_engine::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    info!("*** WebSocket As A Service starting ***");

    let ws_timeout = match env::var("WS_TIMEOUT") { Ok(t) => t, Err(_) => String::from("30000") };
    let max_page = match env::var("MAX_PAGE") { Ok(t) => t, Err(_) => String::from("4") };
    let ws_port = match env::var("WS_PORT") { Ok(p) => p, Err(_) => String::from("8080") };
    let http_port = match env::var("HTTP_PORT") { Ok(p) => p, Err(_) => String::from("8081") };
    let local_address = match env::var("LOCAL_ADDRESS") { Ok(a) => a, Err(_) => String::from("http://localhost:8081") };
    let target_address = match env::var("TARGET_ADDRESS") { Ok(a) => a, Err(_) => String::from("http://localhost:3000/websocket") };

    info!("Local address: {}", local_address);
    info!("Target address: {}", target_address);
    info!("Max message page: {}", max_page);
    info!("Connection timeout (not implemented): {}", ws_timeout);

    let http_client = LocalHttpClient::new(target_address, local_address);
    let ws_engine = WebsocketEngine::new(
        ws_port,
        ws_timeout.parse::<u16>().unwrap(),
        max_page.parse::<usize>().unwrap(),
        http_client
    );
    let mut http_server = HttpServer::new(http_port, ws_engine);

    http_server.start().await;

    Ok(())
}