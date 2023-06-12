use crate::models::Article;
use serde::Serialize;


// Define a struct to hold the formatted article data

#[derive(Debug, Serialize)]
pub struct FormattedArticle{
    pub title: String,
    pub author: String,
    pub source: String,
    pub published_at: String,
    pub url: String, 
}

// Function to format the articles into the desired structure

pub fn format_articles_response(articles: &[Article]) -> Vec<FormattedArticle> {
    articles
        // Iterate over the articles and map each article to a formatted article

        .iter()
        .map(|article| {

        // Create a formatted article instance

            let formatted_article = FormattedArticle {
                title: article.title.clone(),
                author: article.author.clone().unwrap_or_else(|| "Unknown".to_owned()),
                source: article.source.name.clone(),
                published_at: article.publishedAt.clone(),
                url: article.url.clone(), // Assign the `url` field
            };
            formatted_article  
        })
        .collect() // Collect the formatted articles into a vector
}
