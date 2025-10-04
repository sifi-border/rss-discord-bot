use std::env;

use anyhow::Result;
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let discord_webhook_url =
        env::var("DISCORD_WEBHOOK_URL").expect("DISCORD_WEBHOOK_URL must be set in .env file");

    post_to_discord(&discord_webhook_url, "🦀 Hello from Rust!").await?;

    Ok(())
}

async fn post_to_discord(webhook_url: &str, content: &str) -> Result<()> {
    let client = Client::new();
    let payload = serde_json::json!({
        "content": content,
    });

    let response = client.post(webhook_url).json(&payload).send().await?;

    println!("Response: {:?}", response);
    Ok(())
}
