use std::{io::Error};

pub mod websocket_engine;
use websocket_engine::WebsocketEngine;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut ws_engine = WebsocketEngine::new().await.unwrap();

    ws_engine.start().await;
    Ok(())
}

