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
            .uri(format!("{}/{}", self.target_uri, id))
            .header("Content-Type", "application/json")
            .body(Body::from("{\"code\": \"NEW_CONNECTION\"}"))
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
            .uri(format!("{}/{}", self.target_uri, id))
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
            .uri(format!("{}/{}", self.target_uri, id))
            .body(Body::from(""))
            .unwrap();

        let resp = client.request(req).await?;
        println!("{}", resp.status());
        Ok(())
    }
}