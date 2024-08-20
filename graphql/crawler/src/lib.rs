//#![forbid(unsafe_code)]
#![deny(dead_code, unused_imports, unused_mut, missing_docs)]
//! Fetches news from RSS feeds provided by various media outlets.
//! It then uses a [Polymath](https://github.com/Lubmminy/Polymath) extension (crawler) to retrieve the full news content.

pub mod cache;
pub mod scraper;

use cache::Cache;
use chrono::{DateTime, FixedOffset};
use futures::future::join_all;
use polymath_crawler::Crawler as Polymath;
use reqwest::Client;
use rss::Channel;
use scraper::{Extract, Extractor};
use std::{
    collections::HashMap, convert::Infallible, sync::Arc, time::Duration,
};
use tokio::{
    sync::{mpsc::Sender, Mutex, RwLock},
    task::spawn,
    time::{interval, sleep},
};
use tracing::{error, info};
use url::Url;

/// Represents a news article in an RSS feed.
#[derive(Debug)]
pub struct RssNews {
    /// Author written text.
    pub content: String,
    /// The title of the news article.
    pub title: String,
    /// A brief description or summary of the news article.
    pub description: Option<String>,
    /// The URL where the full news article can be accessed.
    pub url: String,
    /// A list of authors who contributed to the news article.
    pub authors: Option<Vec<String>>,
    /// The publication date and time of the news article.
    pub date: Option<DateTime<FixedOffset>>,
    /// Image URL of article.
    pub image: Option<String>,
}

/// Crawl manager.
pub struct Crawler {
    cache: Arc<RwLock<cache::Cache>>,
    client: Client,
    crawler: Arc<Mutex<Polymath>>,
    delay: Duration,
    _content_crawl_delay: Duration,
    feeds: Vec<String>,
    channel: Option<Sender<RssNews>>,
    /// Helps the scraper obtain the written content of the article.
    pub extraction: Arc<RwLock<HashMap<String, Extract>>>,
}

impl Crawler {
    /// Create a new [`Crawler`] with a specified delay between each RSS feed crawl.
    pub fn new(delay: Duration) -> Self {
        let crawler = Polymath::new()
        .follow_redirects(true)
        .timeout(Duration::from_secs(5))
        .retry(3)
        .user_agent(
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/127.0.0.0 Safari/537.36"
            .to_owned()
        );

        Crawler {
            cache: Arc::new(RwLock::new(Cache::new(100))),
            client: Client::new(),
            crawler: Arc::new(Mutex::new(crawler)),
            delay,
            _content_crawl_delay: Duration::from_secs(60),
            feeds: Vec::new(),
            channel: None,
            extraction: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Set custom [Cache] policy.
    pub fn cache(mut self, cache: Cache) -> Self {
        self.cache = Arc::new(RwLock::new(cache));
        self
    }

    /// Sets the list of RSS feeds to be crawled.
    pub fn feeds(&mut self, feeds: Vec<String>) -> &Self {
        self.feeds = feeds;
        self
    }

    /// Set [`tokio::sync::mpsc::Sender`] to receive news after being crawled.
    pub fn channel(&mut self, channel: Sender<RssNews>) -> &Self {
        self.channel = Some(channel);
        self
    }

    /// Starts the crawling process using the provided RSS feeds
    /// and retrieves the full content of each news article.
    pub fn crawl(&self) -> Result<(), Infallible> {
        let interval = interval(self.delay);
        let feeds = self.feeds.clone();
        let client = self.client.clone();
        let crawler = Arc::clone(&self.crawler);
        let extraction = Arc::clone(&self.extraction);
        let cache = Arc::clone(&self.cache);
        let channel = self.channel.clone();

        spawn(async move {
            let mut interval = interval;
            loop {
                interval.tick().await;
                info!("RSS feed reading...");

                let fetches = feeds.iter().map(|url| {
                    let client = client.clone();
                    let crawler = Arc::clone(&crawler);
                    let extraction = Arc::clone(&extraction);
                    let cache = Arc::clone(&cache);
                    let tokio_channel = channel.clone();

                    async move {
                        match client.get(url).send().await {
                            Ok(req) => match req.bytes().await {
                                Ok(content) => match Channel::read_from(&content[..]) {
                                    Ok(channel) => {
                                        let articles_count = Arc::new(RwLock::new(0u64));

                                        let tasks = channel.items().iter().map(|item| {
                                            let mut news = RssNews {
                                                content: String::default(),
                                                title: item.title.as_deref().unwrap_or_default().to_owned(),
                                                description: item.clone().description,
                                                url: item.link.as_deref().unwrap_or_default().to_owned(),
                                                authors: item.author.as_ref().map(|author| {
                                                    author.split(',').map(|v| v.trim().to_string()).collect()
                                                }),
                                                date: item.pub_date.as_ref().and_then(|date| {
                                                    DateTime::parse_from_rfc3339(date).ok()
                                                }),
                                                image: item.enclosure.as_ref().map(|enclosure| enclosure.url.clone()),
                                            };

                                            let crawler = Arc::clone(&crawler);
                                            let extraction = Arc::clone(&extraction);
                                            let cache = Arc::clone(&cache);
                                            let articles_count = Arc::clone(&articles_count);
                                            let tokio_channel = tokio_channel.clone();

                                            async move {
                                                {
                                                    let mut count = articles_count.write().await;
                                                    *count += 1;
                                                }

                                                sleep(Duration::from_secs(*articles_count.read().await * 10)).await;

                                                if let Ok(url) = Url::parse(&news.url) {
                                                    let host = match url.host_str() {
                                                        Some(host) => host.to_owned(),
                                                        None => {
                                                            error!("Invalid URL without host: {}", news.url);
                                                            return;
                                                        }
                                                    };

                                                    let is_cached = cache.write().await.get(url.clone()).unwrap_or(false);

                                                    if !is_cached {
                                                        cache.write().await.set(url.clone()).unwrap();

                                                        match crawler.lock().await.just_fetch(news.url.clone(), false, false) {
                                                            Ok(html) => {
                                                                let extractor = Extractor::new(Arc::clone(&extraction), &host, &html);
                                                                news.content = extractor.extract_content().await;
                                                                if news.image.is_none() {
                                                                    news.image = extractor.extract_image().await;
                                                                }

                                                                if let Some(channel) = tokio_channel {
                                                                    channel.send(news).await.unwrap();
                                                                }
                                                            }
                                                            Err(err) => error!("Failed to fetch article content for {}: {}", news.url, err),
                                                        }
                                                    }
                                                }
                                            }
                                        });

                                        join_all(tasks).await;
                                    }
                                    Err(e) => error!("Failed to parse channel from feed {}: {:?}", url, e),
                                },
                                Err(e) => error!("Failed to read content from feed {}: {:?}", url, e),
                            },
                            Err(e) => error!("Failed to send request to feed {}: {:?}", url, e),
                        }
                    }
                });

                join_all(fetches).await;
            }
        });

        Ok(())
    }
}
