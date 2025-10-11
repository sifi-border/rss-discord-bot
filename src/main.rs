mod discord;
mod repository;
mod rss;

use anyhow::Result;
use chrono::Utc;
use dotenvy::dotenv;
use std::env;

use crate::discord::post_embed;
use crate::repository::{ArticleRepository, JsonArticleRepository};
use crate::{
    discord::{DiscordEmbedPost, SourceCategory},
    rss::{fetch_rss_feed, strip_html_tags, truncate_summary},
};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let discord_webhook_url =
        env::var("DISCORD_WEBHOOK_URL").expect("DISCORD_WEBHOOK_URL must be set in .env file");

    let mut repo = JsonArticleRepository::new("posted_articles.json")?;

    let rss_feed_url = "https://this-week-in-rust.org/rss.xml";
    let feed = fetch_rss_feed(rss_feed_url).await?;

    if let Some(entry) = feed.entries.first() {
        let url = entry.links.first().map_or("", |link| link.href.as_str());

        if !repo.is_new_article(rss_feed_url, &url) {
            println!("article already posted");
            return Ok(());
        }

        let title = entry
            .title
            .as_ref()
            .map_or("No Title", |t| t.content.as_str());

        let summary_string = entry
            .summary
            .as_ref()
            .map_or("No Summary".to_string(), |s| {
                truncate_summary(&strip_html_tags(&s.content), 140)
            });
        let summary = summary_string.as_str();

        eprintln!("Title: {}, URL: {}, Summary: {}", title, url, summary);

        let embed = DiscordEmbedPost::new(title, url, summary, SourceCategory::Rust, Utc::now());

        match post_embed(&discord_webhook_url, embed).await {
            Ok(_) => {
                println!("Posted: {}", title);
                repo.update_latest(&rss_feed_url, title, url)?;
                repo.save()?;
            }
            Err(e) => {
                eprintln!("Failed to post embed: {}", e);
            }
        }
    }

    Ok(())
}
