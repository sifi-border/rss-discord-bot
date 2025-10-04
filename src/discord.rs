use anyhow::Result;
use reqwest::Client;

pub async fn post_to_discord(webhook_url: &str, content: &str) -> Result<()> {
    let client = Client::new();
    let payload = serde_json::json!({
        "content": content,
    });

    let response = client.post(webhook_url).json(&payload).send().await?;

    println!("Response: {:?}", response);
    Ok(())
}
