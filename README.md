# Rust News Aggregator

Rust News Aggregator is a news aggregation service built using Rust. It fetches and displays news articles from various sources, with the help of the NewsAPI service.

This project demonstrates the use of various Rust libraries such as `actix-web`, `reqwest`, `serde`, `dotenv`, and `askama` for creating a dynamic web application.

## Table of Contents
- [Installation](#installation)
- [Configuration](#configuration)
- [Usage](#usage)
- [Development](#development)
- [Contributing](#contributing)
- [License](#license)

## Installation
Make sure you have Rust installed on your system. If not, follow the instructions [here](https://www.rust-lang.org/tools/install).

1. Clone this repository to your local system:
```bash
git clone https://github.com/yourusername/news-aggregator.git
cd news-aggregator
```

## Configuration 

This project requires a NewsAPI key for fetching the articles. This can be set in the .env file in the root directory of the project:

```bash
NEWS_API_KEY=your_news_api_key
```


## Usage 

Once the server is running, you can access it from a web browser:

Open http://localhost:8080/ to view the top news articles.
Open http://localhost:8080/articles to view all articles and search using filter/category.


## Development 

The project is organized into different modules:

main.rs: This is the main file that starts the HTTP server and sets up the routes and data for the application.
handlers.rs: This file handles fetching articles from NewsAPI.
models.rs: This file defines the data structures for the articles.
utils.rs: This file contains utility functions for formatting the articles.
The HTML templates for the application are in the templates directory.

## Contributing

Contributions are welcome! Please fork this repository and open a pull request to add enhancements, bug fixes, or other improvements.

## License

This project is licensed under the MIT License.

