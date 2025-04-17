use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewsItem {
    pub title: String,
    pub source: String,
    pub published_at: String,
    pub summary: String,
    pub url: String,
}
