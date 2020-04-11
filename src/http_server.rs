use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server, Uri, Method};
use hyper::service::{make_service_fn, service_fn};

use super::ws_engine::*;

pub struct HttpServer {
    addr: String,
    ws_engine: WebsocketEngine,
}

impl HttpServer {
    pub fn new(addr: String, ws_engine: WebsocketEngine) -> HttpServer {
        HttpServer {
            addr: addr,
            ws_engine: ws_engine,
        }
    }
}

fn get_id(uri: &Uri) -> String {
    let mut id = uri.to_string();
    id.remove(0);

    return id;
}

async fn hello_world(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let id = get_id(req.uri());

    println!("{}: {}", req.method(), id);

    let body = match *req.method() {
        Method::PUT => String::from("{\"code\": \"OK\"}"),
        Method::DELETE => String::from("{\"code\": \"OK\"}"),
        _ => String::from("{\"code\": \"INVALID_METHOD\"}"),
    };
    let res = Response::builder()
        .header("Content-Type", "application/json")
        .body(body.into());

    Ok(res.unwrap())
}

pub async fn start() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(hello_world))
    });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
