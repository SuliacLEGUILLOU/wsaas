use std::{env, io::Error};

mod http_client;
mod http_server;
mod ws_engine;

use http_client::LocalHttpClient;
use http_server::*;
use ws_engine::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let _log_level = match env::var("LOG_LEVEL") { Ok(l) => l, Err(_) => String::from("INFO") };
    let ws_timeout = match env::var("WS_TIMEOUT") { Ok(t) => t, Err(_) => String::from("30000") };
    let ws_port = match env::var("WS_PORT") { Ok(p) => p, Err(_) => String::from("8080") };
    let http_port = match env::var("HTTP_PORT") { Ok(p) => p, Err(_) => String::from("8081") };
    let local_address = match env::var("LOCAL_ADDRESS") { Ok(a) => a, Err(_) => String::from("http://localhost:8081") };
    let target_address = match env::var("TARGET_ADDRESS") { Ok(a) => a, Err(_) => String::from("http://localhost:3000/websocket") };

    let http_client = LocalHttpClient::new(target_address, local_address);
    let ws_engine = WebsocketEngine::new(ws_port, ws_timeout.parse::<u16>().unwrap(), http_client);
    let mut http_server = HttpServer::new(http_port, ws_engine);

    http_server.start().await;

    Ok(())
}