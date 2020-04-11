use std::{env, io::Error};

mod event_engine;
mod http_client;
mod http_server;
mod ws_engine;

use event_engine::EventEngine;
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
    // let http_server = HttpServer::new(String::from("localhost:3000"), ws_engine);

    ws_engine.start().await;

    Ok(())
}