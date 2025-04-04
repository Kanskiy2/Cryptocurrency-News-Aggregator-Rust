use actix_web::{web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use reqwest::Client;
use dotenvy::dotenv;
use std::env;

#[derive(Deserialize, Serialize)]
struct NewsArticle {
    title: String,
    source: String,
    date: String,
    summary: String,
    link: String,
}

#[derive(Deserialize)]
struct NewsAPIResponse {
    articles: Vec<NewsAPIArticle>,
}

#[derive(Deserialize)]
struct NewsAPIArticle {
    title: String,
    source: NewsAPISource,
    description: Option<String>,
    url: String,
    published_at: String,
}

#[derive(Deserialize)]
struct NewsAPISource {
    name: String,
}

async fn fetch_news_from_newsapi(query: &str) -> Result<Vec<NewsArticle>, reqwest::Error> {
    dotenv().ok();
    let api_key = env::var("NEWSAPI_KEY").expect("NEWSAPI_KEY must be set");
    let url = format!(
        "https://newsapi.org/v2/everything?q={}&apiKey={}",
        query, api_key
    );

    let client = Client::new();
    let res = client.get(&url)
        .header("User-Agent", "CryptoNewsApp/1.0") // Добавляем заголовок User-Agent
        .send()
        .await?;

    // Выводим ответ от API для отладки
    let res_json: serde_json::Value = res.json().await?;
    println!("Response JSON: {:?}", res_json);

    let articles = res_json["articles"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .map(|article| {
            NewsArticle {
                title: article["title"].as_str().unwrap_or("No title").to_string(),
                source: article["source"]["name"].as_str().unwrap_or("No source").to_string(),
                date: article["publishedAt"].as_str().unwrap_or("N/A").to_string(),
                summary: article["description"].as_str().unwrap_or("No description").to_string(),
                link: article["url"].as_str().unwrap_or("N/A").to_string(),
            }
        })
        .collect();

    Ok(articles)
}



async fn fetch_news(query: web::Path<String>) -> impl Responder {
    match fetch_news_from_newsapi(&query.into_inner()).await {
        Ok(articles) => web::Json(articles),
        Err(e) => web::Json(vec![
            NewsArticle {
                title: "Error".to_string(),
                source: "Error".to_string(),
                date: "N/A".to_string(),
                summary: e.to_string(),
                link: "N/A".to_string(),
            }
        ]),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Server running at http://127.0.0.1:8080/");

HttpServer::new(|| {
    App::new()
        .route("/news/{coin}", web::get().to(fetch_news))
})
.bind("127.0.0.1:8080")?
.run()
.await

}
