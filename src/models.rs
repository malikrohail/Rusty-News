use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ArticlesResponse {
    pub articles: Vec<Article>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Article {
    pub source: Source,
    pub author: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub url: String,
    pub urlToImage: Option<String>,
    pub publishedAt: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Source {
    pub id: Option<String>,
    pub name: String,
}
