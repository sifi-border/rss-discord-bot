mod discord;
mod rss;

use anyhow::Result;
use std::env;

use crate::{
    discord::post_to_discord,
    rss::{strip_html_tags, truncate_summary},
};

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let discord_webhook_url =
        env::var("DISCORD_WEBHOOK_URL").expect("DISCORD_WEBHOOK_URL must be set in .env file");

    let rss_feed_url = "https://this-week-in-rust.org/rss.xml";
    let feed = rss::fetch_rss_feed(rss_feed_url).await?;

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
        let summary = entry.summary.as_ref().map_or("No summary", |s| &s.content);
        println!(
            "Summary: {}",
            truncate_summary(&strip_html_tags(summary), 140)
        );
        println!("---");
    }

    post_to_discord(&discord_webhook_url, "🦀 Hello from Rust!").await?;

    Ok(())
}
