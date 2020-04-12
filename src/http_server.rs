use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Error, Server, Uri, Method};
use hyper::service::{make_service_fn, service_fn};
use futures_util::future;

use std::{
    sync::Arc
};

use super::ws_engine::*;

pub struct HttpServer {
    port: u16,
    ws_engine: Arc<WebsocketEngine>,
}

impl HttpServer {
    pub fn new(port: u16, ws_engine: WebsocketEngine) -> HttpServer {
        HttpServer {
            port: port,
            ws_engine: Arc::new(ws_engine),
        }
    }

    pub async fn start(&mut self){
        let ws_engine = self.ws_engine.clone();
        let addr = SocketAddr::from(([127, 0, 0, 1], self.port));

        let make_svc = make_service_fn(move |_| {
            let ws = ws_engine.clone();

            async move {
            Ok::<_, Error>(service_fn(move |req| {
                let id = get_id(req.uri());

                match req.method() {
                    &Method::PUT => ws.send_msg(id, String::from("Yo")),
                    &Method::DELETE => ws.close_ws(id),
                    _ => println!("Not found")
                };
                
                async move {
                    let res = Response::builder()
                        .header("Content-Type", "application/json")
                        .body(Body::from("{\"code\": \"OK\"}"));

                    Ok::<_, Error>(res.unwrap())
                }
            }))
        }
        });

        let server = Server::bind(&addr).serve(make_svc);

        // TODO: Make this a future::select
        // TODO: This return some error that could be useful to debug
        future::join(server, self.ws_engine.clone().start()).await;
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

