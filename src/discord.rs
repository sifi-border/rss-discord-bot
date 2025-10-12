use anyhow::Result;
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SourceCategory {
    Rust,
    Aws,
    Elixir,
    Architecture,
    Career,
}

impl SourceCategory {
    fn color(&self) -> u32 {
        match self {
            Self::Rust => 0xFF5733,
            Self::Aws => 0x4885ED,
            Self::Elixir => 0x3498DB,
            Self::Architecture => 0x2ECC71,
            Self::Career => 0xE67E22,
        }
    }

    fn emoji(&self) -> &str {
        match self {
            Self::Rust => "🦀",
            Self::Aws => "☁️",
            Self::Elixir => "💎",
            Self::Architecture => "🏗️",
            Self::Career => "💼",
        }
    }

    fn footer_text(&self, source_name: &str) -> String {
        format!("{} {}", self.emoji(), source_name)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordEmbedPost {
    title: String,
    url: String,
    description: String,
    color: u32,
    timestamp: DateTime<Utc>,
    footer: EmbedFooter,
}

impl DiscordEmbedPost {
    pub fn new(
        title: &str,
        url: &str,
        description: &str,
        category: SourceCategory,
        timestamp: DateTime<Utc>,
    ) -> Self {
        Self {
            title: title.to_string(),
            url: url.to_string(),
            description: description.to_string(),
            color: category.color(),
            timestamp,
            footer: EmbedFooter::new(category.footer_text(title)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbedFooter {
    text: String,
}

impl EmbedFooter {
    pub fn new(text: String) -> Self {
        Self { text }
    }
}

#[allow(unused)]
pub async fn post_to_discord(webhook_url: &str, content: &str) -> Result<()> {
    let client = Client::new();
    let payload = serde_json::json!({
        "content": content,
    });

    let response = client.post(webhook_url).json(&payload).send().await?;

    println!("Response: {:?}", response);
    Ok(())
}

pub async fn post_embed(webhook_url: &str, embed: DiscordEmbedPost) -> Result<()> {
    let client = reqwest::Client::new();
    let payload = json!({
        "embeds": [embed]
    });

    eprintln!("{:?}", payload);

    let response = client.post(webhook_url).json(&payload).send().await?;

    println!("Status: {}", response.status());
    Ok(())
}
