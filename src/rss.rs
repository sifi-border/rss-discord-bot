use anyhow::Result;
use feed_rs::model::Feed;

pub async fn fetch_rss_feed(url: &str) -> Result<Feed> {
    let content = reqwest::get(url).await?.bytes().await?;
    let feed = feed_rs::parser::parse(&content[..])?;
    Ok(feed)
}
