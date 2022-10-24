use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Quote {
    body: String,
    author: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // https://stoicquotesapi.com/v1/api/quotes/random
    // https://stoic-wisdom.com/api
    let base_uri = "https://stoicquotesapi.com/v1/api/quotes".to_string();
    let uri = base_uri + "/random";

    let resp = reqwest::get(uri).await?.json::<Quote>().await?;

    println!("\"{}\" - {}", resp.body, resp.author);

    Ok(())
}
