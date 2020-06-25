use hyper::{Body, Method, Request, StatusCode};
use hyper::Client;
use hyper_tls::HttpsConnector;

use isahc::prelude::*;
use isahc::prelude::Request as SyncRequest;


#[derive(Clone)]
pub struct LocalHttpClient {
    target_uri: String,
    local_uri: String
}

impl LocalHttpClient {
    pub fn new(target_uri: String, local_uri: String) -> LocalHttpClient {
        LocalHttpClient{
            target_uri: target_uri,
            local_uri: local_uri,
        }
    }

    /**
     * TODO: Ho god so much wrong stuff here but it works so...
     * Request use isahc client instead of the hyper one because I can't figure out how to async this properly
     * (See https://github.com/snapview/tokio-tungstenite/issues/98)
     */
    pub fn on_connect(&self, id: String, auth_header: String, timeout: u16) -> bool {
        let uri = format!("{}/{}", self.target_uri, id);
        let body = format!("{{\"code\": \"NEW_CONNECTION\",\"ws_uri\":\"{}/{}\",\"timeout\":{}}}", self.local_uri, id, timeout);
        let response = SyncRequest::post(uri)
            .header("content-type", "application/json")
            .header("Authorization", auth_header)
            .body(body).unwrap()
            .send().unwrap();

        debug!("Client {} connect [{}]",id, response.status());
        match response.status() {
            StatusCode::OK => true,
            _ => false,
        }
    }

    // TODO: Make the content-type variable
    pub async fn on_message(self, id: String, msg: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        let req = Request::builder()
            .method(Method::PUT)
            .uri(format!("{}/{}", self.target_uri, id))
            .header("Content-Type", "application/json")
            .body(Body::from(msg))
            .unwrap();

        let resp = client.request(req).await?;
        debug!("Client {} message [{}]", id, resp.status());
        Ok(())
    }

    pub async fn on_disconnect(self, id: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        let body = format!("{{\"code\": \"CONNECTION_CLOSE\", \"ws_uri\": \"{}/{}\"}}", self.local_uri, id);

        let req = Request::builder()
            .method(Method::DELETE)
            .uri(format!("{}/{}", self.target_uri, id))
            .header("Content-Type", "application/json")
            .body(Body::from(body))
            .unwrap();

        let resp = client.request(req).await?;
        debug!("Sending client {} close event to server: ({})", id, resp.status());
        Ok(())
    }
}