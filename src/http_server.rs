use std::net::SocketAddr;
use hyper::{Body, Request, Response, Error, Server, Uri, Method};
use hyper::service::{make_service_fn, service_fn};
use futures_util::{future, pin_mut};

use std::{
    sync::Arc
};

use super::ws_engine::*;

pub struct HttpServer {
    port: u16,
    ws_engine: Arc<WebsocketEngine>,
}

impl HttpServer {
    pub fn new(port: String, ws_engine: WebsocketEngine) -> HttpServer {
        HttpServer {
            port: port.parse::<u16>().unwrap(),
            ws_engine: Arc::new(ws_engine),
        }
    }

    pub async fn start(&mut self){
        let ws_engine = self.ws_engine.clone();
        let addr = SocketAddr::from(([127, 0, 0, 1], self.port));

        let make_svc = make_service_fn(move |_| {
            let ws = ws_engine.clone();

            async move {
                Ok::<_, Error>(service_fn(move |req: Request<Body>| {
                    let id = get_id(req.uri());
                    let ws = ws.clone();

                    async move {
                        let code = match req.method() {
                            &Method::PUT => ws.send_msg(id, req.into_body()).await,
                            &Method::DELETE => ws.close_ws(id).await,
                            _ => String::from("BAD_REQUEST")
                        };

                        let res = Response::builder()
                            .header("Content-Type", "application/json")
                            .body(Body::from(format!("{{\"code\": \"{}\"}}", code)));

                        Ok::<_, Error>(res.unwrap())
                    }
                }))
            }
        });

        let server = Server::bind(&addr).serve(make_svc);
        let ws = self.ws_engine.start();

        pin_mut!(server, ws);

        // TODO: return of the future is holding some error that I should display
        future::select(server, ws).await;
    }
}

fn get_id(uri: &Uri) -> String {
    let mut id = uri.to_string();
    id.remove(0);

    return id;
}
