use std::{
    collections::HashMap,
    net::{SocketAddr},
    sync::{Arc},
};

use hyper::Body;
use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};

use tokio::sync::Mutex;
use tokio::net::{TcpListener, TcpStream};
use tokio::time::Instant;
use tungstenite::protocol::Message;
use tungstenite::handshake::server::{Request, Response};

use uuid::Uuid;

use super::http_client::*;

type Tx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<String, Tx>>>;

pub struct WebsocketEngine {
    timeout: u16,
    max_page: usize,
    port: String,
    connections: PeerMap,
    http_client: LocalHttpClient,
}

// TODO: Graceful shutdown
impl WebsocketEngine {
    pub fn new(port: String, timeout: u16, max_page: usize, client: LocalHttpClient) -> WebsocketEngine {
        WebsocketEngine {
            timeout: timeout,
            max_page: max_page,
            port: port,
            connections: PeerMap::new(Mutex::new(HashMap::new())),
            http_client: client,
        }
    }

    pub async fn start(&self) {
        let try_socket = TcpListener::bind(format!("127.0.0.1:{}", &self.port)).await;
        let mut listener = try_socket.expect("Failed to bind");
        println!("Listening on post: {}", self.port);

        while let Ok((stream, addr)) = listener.accept().await {
            let id = Uuid::new_v4().to_string();

            let task = WebsocketEngine::handle_connection(self.connections.clone(), self.http_client.clone(), id, stream, addr, self.timeout);
            tokio::spawn(task);
        }
    }

    // TODO: Current use of the LocalHttpClient and id makes a lot of cloning
    // TODO: Figure out the borrow issue that prevent making this a method
    // TODO: Find a way to make that function prototype lighter
    // TODO: Too much todo
    async fn handle_connection(peer_map: PeerMap, client: LocalHttpClient, id: String, raw_stream: TcpStream, addr: SocketAddr, timeout: u16) {
        let start_time = Instant::now();
        let auth_middleware_callback = |req: &Request, mut res: Response| {
            let auth = match req.headers().get("Authorization") {
                Some(s) => String::from(s.to_str().unwrap()),
                None => String::from("none")
            };

            if !client.on_connect(id.clone(), auth, timeout) {
                res.headers_mut().remove("upgrade");
            }
            Ok(res)
        };
        let ws_stream = tokio_tungstenite::accept_hdr_async(raw_stream, auth_middleware_callback)
            .await
            .expect("Error during the websocket handshake occurred");
        println!("WebSocket connection established: {}", addr);

        let (tx, rx) = unbounded();
        peer_map.lock().await.insert(id.clone(), tx);

        let (outgoing, incoming) = ws_stream.split();
        let msg_in = incoming.try_for_each(|msg| {
            let message_length = msg.len() / (1024*32) + 1;
            println!("new incoming message from {} ({} page)", id, message_length);

            if message_length > 4 {
                println!("Message too long");
            } else {
                tokio::spawn(client.clone().on_message(id.clone(), msg.to_string()));
            }
            future::ok(())
        });
        let msg_out = rx.map(Ok).forward(outgoing);
        pin_mut!(msg_in, msg_out);
        future::select(msg_in, msg_out).await;

        peer_map.lock().await.remove(&id);
        println!("closing {}, duration {}s", id, start_time.elapsed().as_secs());
        match client.clone().on_disconnect(id).await {
            Ok(_) => println!("{} disconnected", &addr),
            Err(_) => eprintln!("Error while sending disconnection request: {}", &addr)
        };
    }

    async fn handle_msg(connection: UnboundedSender<Message>, body: Vec<u8>) {
        match connection.unbounded_send(Message::from(body)) {
            Err(e) => println!("{}", e),
            _ => {}
        };
    }

    pub async fn send_msg(&self, id: String, body: Body) -> String {
        let peer = self.connections.clone();
        let peer_map = peer.lock().await;

        let tmp_body = hyper::body::to_bytes(body).await.unwrap();
        let full_body = tmp_body.iter().cloned().collect::<Vec<u8>>();

        let message_length = full_body.len() / (1024*32) + 1;

        if message_length > self.max_page {
            return String::from("MESSAGE_TOO_LONG")
        }

        println!("new outgoing message ({} page)", message_length);

        match peer_map.get(&id) {
            Some(connection) => {
                WebsocketEngine::handle_msg(connection.clone(), full_body).await;
                String::from("OK")
            }
            None => String::from("NOT_FOUND")
        }
    }

    pub async fn close_ws(&self, id: String)  -> String {
        let peer = self.connections.clone();
        let mut peer_map = peer.lock().await;

        match peer_map.get(&id) {
            Some(connection) => {
                connection.close_channel();
                peer_map.remove(&id);
                String::from("OK")
            }
            None => String::from("NOT_FOUND")
        }
        
    }
}
