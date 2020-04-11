use hyper::{Body, Method, Request};
use hyper::Client;
use hyper_tls::HttpsConnector;

#[derive(Clone)]
pub struct HttpClient {
    target_uri: String,
}

impl HttpClient {
    pub fn new(uri: String) -> HttpClient {
        HttpClient{
            target_uri: uri,
        }
    }

    pub async fn on_connect(&self, id: &String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        let req = Request::builder()
            .method(Method::POST)
            .uri(self.target_uri.clone())
            .header("Content-Type", "application/json")
            .body(Body::from("{\"code\": \"OK\"}"))
            .unwrap();

        let resp = client.request(req).await?;
        println!("{}", resp.status());
        Ok(())
    }

    pub async fn on_message(self, id: String, msg: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        let req = Request::builder()
            .method(Method::PUT)
            .uri(self.target_uri)
            .header("Content-Type", "application/json")
            .body(Body::from(msg))
            .unwrap();

        let resp = client.request(req).await?;
        println!("{}", resp.status());
        Ok(())
    }

    pub async fn on_disconnect(self, id: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        let req = Request::builder()
            .method(Method::DELETE)
            .uri(self.target_uri)
            .body(Body::from(""))
            .unwrap();

        let resp = client.request(req).await?;
        println!("{}", resp.status());
        Ok(())
    }
}