use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use crate::models::{Article, ArticlesResponse};

pub async fn fetch_articles(
    client: &Client,
    api_key: &str,
    category: Option<String>,
    keyword: Option<String>,
) -> Result<Vec<Article>, Box<dyn std::error::Error>> {
    let mut url = format!("https://newsapi.org/v2/top-headlines?country=us&apiKey={}", api_key);

    if let Some(category) = category {
        url.push_str(&format!("&category={}", category));
    }

    if let Some(keyword) = keyword {
        url.push_str(&format!("&q={}", keyword));
    }

    // Create a new HeaderMap and add a User-Agent header

    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("news-aggregator"));

    // Send a GET request to the NewsAPI

    let response = client.get(&url).headers(headers).send().await?;

    println!("Received response with status code {}", response.status());

    if response.status().is_success() {
        let articles_response = response.json::<ArticlesResponse>().await?;

        println!("Successfully parsed JSON response");

        Ok(articles_response.articles)
    } else {
        let status = response.status();
        let body = response.text().await?;
        println!("Request failed with status code {}: {}", status, body);

        Err(format!("Request failed with status code {}: {}", status, body).into())
    }
}
