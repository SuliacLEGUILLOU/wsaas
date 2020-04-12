use std::{env, io::Error};

mod http_client;
mod http_server;
mod ws_engine;

use http_client::HttpClient;
use http_server::*;
use ws_engine::*;



#[tokio::main]
async fn main() -> Result<(), Error> {
    let ws_addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    let http_client = HttpClient::new(String::from("http://localhost:8081/websocket"));
    let ws_engine = WebsocketEngine::new(ws_addr, http_client);
    let mut http_server = HttpServer::new(3000, ws_engine);

    http_server.start().await;

    Ok(())
}