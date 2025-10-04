use std::env;

use anyhow::Result;
use feed_rs::model::Feed;
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let discord_webhook_url =
        env::var("DISCORD_WEBHOOK_URL").expect("DISCORD_WEBHOOK_URL must be set in .env file");

    let rss_feed_url = "https://this-week-in-rust.org/rss.xml";
    let feed = fetch_rss_feed(rss_feed_url).await?;

    println!(
        "Feed Title: {}",
        feed.title.as_ref().map_or("No title", |t| &t.content)
    );
    println!("Number of Entries: {}", feed.entries.len());

    for entry in feed.entries.iter().take(3) {
        println!(
            "Title: {}",
            entry.title.as_ref().map_or("No title", |t| &t.content)
        );
        println!(
            "Link: {}",
            entry.links.first().map_or("No link", |l| &l.href)
        );
        println!("---");
    }

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

async fn fetch_rss_feed(url: &str) -> Result<Feed> {
    let content = reqwest::get(url).await?.bytes().await?;
    let feed = feed_rs::parser::parse(&content[..])?;
    Ok(feed)
}
