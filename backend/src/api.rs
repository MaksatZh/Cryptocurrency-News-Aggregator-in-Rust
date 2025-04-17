use serde::Deserialize;
use crate::models::NewsItem;
use std::error::Error;

#[derive(Debug, Deserialize)]
struct CryptoNewsResponse {
    data: Vec<CryptoNewsArticle>,
}

#[derive(Debug, Deserialize)]
struct CryptoNewsArticle {
    title: String,
    date: String,
    text: String,
    news_url: String,
    source_name: String,
}

pub async fn fetch_crypto_news(symbol: &str) -> Result<Vec<NewsItem>, Box<dyn Error>> {
    let (mut crypto_news, mut backup_news) = tokio::join!(
        fetch_from_cryptonews(symbol),
        fetch_from_backup_api(symbol)
    );

    if crypto_news.is_err() && backup_news.is_err() {
        return Err("Both APIs failed".into());
    }

    let mut combined = Vec::new();

    if let Ok(mut news) = crypto_news {
        combined.append(&mut news);
    }

    if let Ok(mut news) = backup_news {
        combined.append(&mut news);
    }

    Ok(combined)
}

async fn fetch_from_cryptonews(symbol: &str) -> Result<Vec<NewsItem>, Box<dyn Error>> {
    let api_key = std::env::var("CRYPTO_API_KEY")?;
    let url = format!(
        "https://cryptonews-api.com/api/v1?tickers={}&items=3&token={}",
        symbol,
        api_key
    );

    let response = reqwest::get(&url).await?;
    let body = response.text().await?;
    println!("🔍 CryptoNews API response:\n{}", body);

    if body.contains("not a valid API token")
        || body.contains("error")
        || body.contains("Activate your plan")
    {
        return Err("CryptoNews API error or limit".into());
    }

    let parsed: CryptoNewsResponse = serde_json::from_str(&body)?;

    let result: Vec<NewsItem> = parsed.data.into_iter().map(|a| NewsItem {
        title: a.title,
        source: format!("CryptoNews API ({})", a.source_name),
        published_at: a.date,
        summary: a.text,
        url: a.news_url,
    }).collect();

    Ok(result)
}

//  NewsData.io
#[derive(Debug, Deserialize)]
struct NewsDataArticle {
    title: String,
    link: String,
    pubDate: String,
    description: Option<String>,
    source_id: String,
}

#[derive(Debug, Deserialize)]
struct NewsDataResponse {
    results: Vec<NewsDataArticle>,
}

async fn fetch_from_backup_api(symbol: &str) -> Result<Vec<NewsItem>, Box<dyn Error>> {
    let backup_key = std::env::var("NEWSDATA_API_KEY")?;
    let url = format!(
        "https://newsdata.io/api/1/news?apikey={}&q={}&language=en&category=business",
        backup_key,
        symbol
    );

    let response = reqwest::get(&url).await?;
    let body = response.text().await?;
    println!("📡 NewsData.io response:\n{}", body);

    if body.contains("\"status\":\"error\"") {
        return Err("NewsData.io returned an error".into());
    }

    let parsed: NewsDataResponse = serde_json::from_str(&body)?;

    let news: Vec<NewsItem> = parsed.results.into_iter().map(|a| NewsItem {
        title: a.title,
        source: format!("NewsData.io ({})", a.source_id),
        published_at: a.pubDate,
        summary: a.description.unwrap_or_else(|| "No summary provided".to_string()),
        url: a.link,
    }).collect();

    Ok(news)
}
