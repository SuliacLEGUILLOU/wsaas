use std::io::Error;
use rand::Rng;
use std::collections::HashMap;
use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{
    future, pin_mut,
    stream::TryStreamExt,
    StreamExt,
};
use tokio::net::{TcpListener, TcpStream};

#[derive(Debug)]
pub struct WebsocketEngine {
    connection: HashMap<u32, String>,
    listener: TcpListener
}

impl WebsocketEngine {
    pub async fn new() -> Result<WebsocketEngine, Error> {
        let addr = "127.0.0.1:8080".to_string();
        let try_socket = TcpListener::bind(&addr).await;
        let listener = try_socket.expect("Failed to bind");
        
        let engine = WebsocketEngine {
            connection: HashMap::new(),
            listener: listener,
        };

        return Ok(engine)
    }

    pub async fn start(&mut self) {
        while let Ok((stream, addr)) = self.listener.accept().await {
            tokio::spawn(async move {
                println!("Incoming TCP connection: {}", addr);
                let ws_stream = tokio_tungstenite::accept_async(stream)
                    .await
                    .expect("Error during the websocket handshake occurred");
                println!("New WebSocket connection: {}", addr);

                let (write, read) = ws_stream.split();
                read.forward(write)
                    .await
                    .expect("Failed to forward message")
                // write.unbounded_send("Hello")
            });
        }
    }

    pub async fn new_connection(&mut self, stream: TcpStream) {
        let id = self.get_connection_id();

        self.connection.insert(id, String::from("Yo"));
    }

    fn get_connection_id(&self) -> u32 {
        loop {
            let i: u32 = rand::thread_rng().gen();
            match self.connection.get(&i) {
                Some(_) => return i,
                None => continue,
            }
        }
    }
}
