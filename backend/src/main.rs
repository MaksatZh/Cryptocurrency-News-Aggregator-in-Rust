use actix_web::{web, App, HttpServer, Responder};
use actix_files::{Files, NamedFile};

mod handlers;
mod api;
mod models;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    println!("🚀 Server running at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/api/news/{symbol}", web::get().to(handlers::get_news))
            .service(Files::new("/static", "../frontend")) //  путь к стилям и JS
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn index() -> impl Responder {
    NamedFile::open_async("../frontend/index.html").await //  путь к HTML
}
