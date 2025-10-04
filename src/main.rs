use std::env;

use anyhow::Result;
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let discord_webhook_url =
        env::var("DISCORD_WEBHOOK_URL").expect("DISCORD_WEBHOOK_URL must be set in .env file");

    let client = Client::new();
    let payload = serde_json::json!({
        "content": "🦀 Hello from Rust!",
    });

    let response = client
        .post(&discord_webhook_url)
        .json(&payload)
        .send()
        .await?;

    println!("Response: {:?}", response);

    Ok(())
}
