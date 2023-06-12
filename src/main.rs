use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use reqwest::Client;
use std::env;
use dotenv::dotenv;
use crate::utils::FormattedArticle;
use askama::Template;

// Import application-specific modules
mod handlers;
mod models;
mod utils;

// askama macros to specify HTML templates used for rendering instances of these structs.
#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    articles: Vec<FormattedArticle>,
}

#[derive(Template)]
#[template(path = "articles.html")]
struct ArticlesTemplate {
    articles: Vec<FormattedArticle>,
}

// Defined the "/" route handler, which fetches articles and passes them to the index template
#[get("/")]
async fn index(
    client: web::Data<Client>,
    api_key: web::Data<String>,
) -> impl Responder {
    match handlers::fetch_articles(&client, &api_key, None, None).await {
        Ok(top_articles) => {
            let formatted_articles = utils::format_articles_response(&top_articles);
            let template = IndexTemplate {
                articles: formatted_articles,
            };
            HttpResponse::Ok().body(template.render().unwrap())
        },
        Err(error) => {
            println!("Failed to fetch articles: {}", error);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/articles")]
async fn articles(
    client: web::Data<Client>,
    api_key: web::Data<String>,
    web::Query(query_params): web::Query<QueryParams>,
) -> impl Responder {
    let category = query_params.category.clone();
    let keyword = query_params.keyword.clone();

    match handlers::fetch_articles(&client, &api_key, category, keyword).await {
        Ok(other_articles) => {
            let formatted_articles = utils::format_articles_response(&other_articles);
            let template = ArticlesTemplate {
                articles: formatted_articles,
            };
            HttpResponse::Ok().body(template.render().unwrap())
        },
        Err(error) => {
            println!("Failed to fetch articles: {}", error);
            HttpResponse::InternalServerError().finish()
        }
    }
}

// Query parameters for the /articles endpoint
#[derive(serde::Deserialize)]
struct QueryParams {
    category: Option<String>,
    keyword: Option<String>,
}

// sets up the HTTP server and the applications route and data
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Get the NewsAPI key from the environment variables
    let api_key = env::var("NEWS_API_KEY").expect("NEWS_API_KEY is not set in .env");
    println!("API Key: {}", api_key);
    let client = Client::new();

    // Start an HTTP server
    HttpServer::new(move || {
        App::new()
            .service(index)
            .service(articles)
            .app_data(web::Data::new(client.clone()))
            .app_data(web::Data::new(api_key.clone()))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
