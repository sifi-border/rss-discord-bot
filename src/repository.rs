use std::{collections::HashMap, fs, path::PathBuf};

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatestArticle {
    title: String,
    url: String,
    posted_at: DateTime<Utc>,
}

pub trait ArticleRepository {
    fn new(path_str: impl Into<PathBuf>) -> Result<Self>
    where
        Self: Sized;

    fn is_new_article(&self, feed_url: &str, article_url: &str) -> bool;

    fn update_latest(
        &mut self,
        feed_url: &str,
        article_title: &str,
        article_url: &str,
    ) -> Result<()>;

    fn save(&self) -> Result<()>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonArticleRepository {
    #[serde(skip)]
    file_path: PathBuf,
    article_by_feed: HashMap<String, LatestArticle>,
}

impl ArticleRepository for JsonArticleRepository {
    fn new(path_str: impl Into<PathBuf>) -> Result<Self> {
        let file_path = path_str.into();
        if !file_path.exists() {
            fs::File::create(&file_path)?;

            return Ok(Self {
                file_path,
                article_by_feed: HashMap::new(),
            });
        }
        let content = fs::read_to_string(&file_path)?;
        let article_by_feed = serde_json::from_str(&content)?;

        Ok(JsonArticleRepository {
            file_path,
            article_by_feed,
        })
    }

    fn is_new_article(&self, feed_url: &str, article_url: &str) -> bool {
        match self.article_by_feed.get(feed_url) {
            Some(article) => article.url != article_url,
            None => true,
        }
    }

    fn update_latest(
        &mut self,
        feed_url: &str,
        article_title: &str,
        article_url: &str,
    ) -> Result<()> {
        self.article_by_feed.insert(
            feed_url.to_string(),
            LatestArticle {
                title: article_title.to_string(),
                url: article_url.to_string(),
                posted_at: Utc::now(),
            },
        );
        Ok(())
    }

    fn save(&self) -> Result<()> {
        let json = serde_json::to_string_pretty(&self.article_by_feed)?;
        fs::write(&self.file_path, json)?;
        Ok(())
    }
}
