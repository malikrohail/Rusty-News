# Rust News Aggregator

## Overview
Rust News Aggregator is a comprehensive news aggregation service leveraging the robustness of Rust programming. It's designed to fetch and display articles from a multitude of sources in real-time, powered by the NewsAPI service.

The project is a testament to Rust's capability in handling web services, utilizing a suite of libraries for web serving, requests handling, serialization, environment management, and templating.

## Features
- Aggregation of live news articles from various sources.
- NewsAPI integration for a vast selection of news feeds.
- Dynamic web serving with `actix-web`.
- Seamless serialization and deserialization of data with `serde`.
- Environment configuration management with `dotenv`.
- Templating with `askama` for a dynamic front-end.

## Table of Contents
- [Installation](#installation)
- [Configuration](#configuration)
- [Usage](#usage)
- [Development](#development)
- [Contributing](#contributing)
- [License](#license)

## Installation
To run the Rust News Aggregator, ensure that Rust is installed on your system. Follow the installation guide [here](https://www.rust-lang.org/tools/install) if you haven't installed Rust.

To install the Rust News Aggregator:
```bash
git clone https://github.com/yourusername/news-aggregator.git
cd news-aggregator
```

## Configuration
```
NEWS_API_KEY=your_news_api_key
```

## Development

The project is modularized into different components:

main.rs: Initializes the HTTP server, sets up routes, and prepares the application data.
handlers.rs: Handles the fetching of articles from NewsAPI.
models.rs: Defines the data structures for articles.
utils.rs: Contains utility functions for article formatting.
Templates for the application's front-end are located in the templates directory.

## License

Rust News Aggregator is made available under the MIT License, allowing for open and free software use, modification, and distribution.



