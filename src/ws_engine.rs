use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};

use tokio::net::{TcpListener, TcpStream};
use tungstenite::protocol::Message;

use uuid::Uuid;

use super::http_client::*;

type Tx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<String, Tx>>>;

pub struct WebsocketEngine {
    addr: String,
    connections: PeerMap,
    http_client: HttpClient,
}

impl WebsocketEngine {
    pub fn new(addr: String, client: HttpClient) -> WebsocketEngine {
        WebsocketEngine {
            addr: addr,
            connections: PeerMap::new(Mutex::new(HashMap::new())),
            http_client: client,
        }
    }

    pub async fn start(&self) {
        let try_socket = TcpListener::bind(&self.addr).await;
        let mut listener = try_socket.expect("Failed to bind");
        println!("Listening on: {}", self.addr);

        while let Ok((stream, addr)) = listener.accept().await {
            let id = Uuid::new_v4().to_string();
            let auth = self.http_client.on_connect(&id).await;

            println!("New connection from {}, {}", addr, id);

            tokio::spawn(self::handle_connection(self.connections.clone(), self.http_client.clone(), id, stream, addr));
        }
    }

    pub fn send_msg(&self, id: String, msg: String) {
        let peer = self.connections.clone();
        let peer_map = peer.lock().unwrap();
        let connection = peer_map.get(&id).unwrap();

        connection.unbounded_send(Message::from(msg));
    }

    pub fn close_ws(&self, id: String) {
        let peer = self.connections.clone();
        let mut peer_map = peer.lock().unwrap();
        let connection = peer_map.get(&id).unwrap();

        connection.close_channel();
        peer_map.remove(&id);
    }
}

// TODO: Current use of the HttpClient and id makes a lot of cloning
async fn handle_connection(peer_map: PeerMap, client: HttpClient, id: String, raw_stream: TcpStream, addr: SocketAddr) {
    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("WebSocket connection established: {}", addr);

    let (tx, rx) = unbounded();
    peer_map.lock().unwrap().insert(id.clone(), tx);

    let (outgoing, incoming) = ws_stream.split();
    let msg_in = incoming.try_for_each(|msg| {
        println!("{}", msg);
        tokio::spawn(client.clone().on_message(id.clone(), msg.to_string()));
        future::ok(())
    });
    let msg_out = rx.map(Ok).forward(outgoing);
    pin_mut!(msg_in, msg_out);
    future::select(msg_in, msg_out).await;

    println!("{} disconnected", &addr);
    peer_map.lock().unwrap().remove(&id);
    client.on_disconnect(id).await;
}
