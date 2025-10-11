mod discord;
mod rss;

use anyhow::Result;
use chrono::Utc;
use dotenvy::dotenv;
use std::env;

use crate::discord::post_embed;
use crate::{
    discord::{DiscordEmbedPost, SourceCategory},
    rss::{fetch_rss_feed, strip_html_tags, truncate_summary},
};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let discord_webhook_url =
        env::var("DISCORD_WEBHOOK_URL").expect("DISCORD_WEBHOOK_URL must be set in .env file");

    let rss_feed_url = "https://this-week-in-rust.org/rss.xml";
    let feed = fetch_rss_feed(rss_feed_url).await?;

    if let Some(entry) = feed.entries.first() {
        let title = entry
            .title
            .as_ref()
            .map_or("No Title".to_string(), |t| t.content.clone());

        let url = entry
            .links
            .first()
            .map_or(String::new(), |link| link.href.clone());

        let summary = entry
            .summary
            .as_ref()
            .map_or("No Summary".to_string(), |s| {
                truncate_summary(&strip_html_tags(&s.content), 140)
            });

        eprintln!("Title: {}, URL: {}, Summary: {}", title, url, summary);

        let embed = DiscordEmbedPost::new(title, url, summary, SourceCategory::Rust, Utc::now());

        post_embed(&discord_webhook_url, embed).await?;
    }

    Ok(())
}
