use actix_web::{web, HttpResponse, Responder};
use crate::api::fetch_crypto_news;
use crate::models::NewsItem;

pub async fn get_news(path: web::Path<String>) -> impl Responder {
    let symbol = path.into_inner();

    println!("🟡 Fetching news for: {}", symbol);

    match fetch_crypto_news(&symbol).await {
        Ok(news) if !news.is_empty() => {
            println!(" Found {} articles", news.len());
            HttpResponse::Ok().json(news)
        }
        Ok(_) => {
            println!(" No news found");
            HttpResponse::Ok().body("No news found")
        }
        Err(err) => {
            eprintln!(" Error fetching news: {}", err);
            HttpResponse::InternalServerError().body("Failed to fetch news")
        }
    }
}
