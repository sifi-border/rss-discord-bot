use anyhow::Result;
use feed_rs::model::Feed;
use regex::Regex;

pub async fn fetch_rss_feed(url: &str) -> Result<Feed> {
    let content = reqwest::get(url).await?.bytes().await?;
    let feed = feed_rs::parser::parse(&content[..])?;
    Ok(feed)
}

pub fn strip_html_tags(input: &str) -> String {
    let re = Regex::new(r"<[^>]*>").unwrap();
    re.replace_all(input, "").to_string()
}

pub fn truncate_summary(summary: &str, max_len: usize) -> String {
    if summary.len() <= max_len {
        summary.to_string()
    } else {
        let mut truncated = summary[..max_len].to_string();
        truncated.push_str("...");
        truncated
    }
}
