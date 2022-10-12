use hyper::{body::Buf, Body, Client, Request};
use hyper_tls::HttpsConnector;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Quote {
    body: String,
    author: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let https = HttpsConnector::new();
    let client = Client::builder().build(https);
    // https://stoicquotesapi.com/v1/api/quotes/random
    // https://stoic-wisdom.com/api
    let base_uri = "https://stoicquotesapi.com/v1/api/quotes".to_string();
    let uri = base_uri + "/random";

    let req = Request::get(uri).body(Body::empty())?;
    let res = client.request(req).await?;

    let body = hyper::body::aggregate(res).await?;

    let quote: Quote = serde_json::from_reader(body.reader())?;

    println!("\"{}\" - {}", quote.body, quote.author);

    Ok(())
}
