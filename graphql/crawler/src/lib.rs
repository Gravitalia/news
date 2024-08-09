//#![forbid(unsafe_code)]
#![deny(dead_code, unused_imports, unused_mut, missing_docs)]
//! Fetches news from RSS feeds provided by various media outlets.
//! It then uses a [Polymath](https://github.com/Lubmminy/Polymath) extension (crawler) to retrieve the full news content.

pub mod scraper;

use chrono::{DateTime, FixedOffset};
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
use tracing::{debug, error, info};
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
            client: Client::new(),
            crawler: Arc::new(Mutex::new(crawler)),
            delay,
            _content_crawl_delay: Duration::from_secs(60),
            feeds: Vec::new(),
            channel: None,
            extraction: Arc::new(RwLock::new(HashMap::new())),
        }
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
        let client = self.client.clone();
        let mut interval = interval(self.delay);
        let feeds = self.feeds.clone();
        let crawler = Arc::clone(&self.crawler);
        let extraction = Arc::clone(&self.extraction);

        spawn(async move {
            loop {
                interval.tick().await;
                info!("RSS feed reading...");

                for url in &feeds {
                    debug!("Getting {} feed.", url);

                    if let Ok(req) = client.get(url).send().await {
                        match req.bytes().await {
                            Ok(content) => {
                                match Channel::read_from(&content[..]) {
                                    Ok(channel) => {
                                        let articles_count =
                                            Arc::new(RwLock::new(0u64));

                                        for item in channel.items() {
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
                                            let extraction =
                                                Arc::clone(&extraction);
                                            let articles_count =
                                                Arc::clone(&articles_count);

                                            spawn(async move {
                                                {
                                                    let mut count =
                                                        articles_count
                                                            .write()
                                                            .await;
                                                    *count += 1;
                                                }

                                                sleep(Duration::from_secs(
                                                    *articles_count
                                                        .read()
                                                        .await
                                                        * 10,
                                                ))
                                                .await;

                                                if let Ok(host) = Url::parse(&news.url)
                                                .map_err(|e| format!("Invalid URL: {}", e))
                                                .and_then(|url| {
                                                    url.host_str()
                                                        .map(|host| host.to_owned())
                                                        .ok_or_else(|| "No host found in URL".to_owned())
                                                })
                                            {
                                                match crawler
                                                .lock()
                                                .await
                                                .just_fetch(
                                                    news.url.clone(), false, false,
                                                ) {
                                                    Ok(html) => {
                                                        let extractor = Extractor::new(Arc::clone(&extraction), &host, &html);
                                                        news.content = extractor.extract_content().await;
                                                        if news.image.is_none() {
                                                            news.image = extractor.extract_image().await;
                                                        }
                                                        println!("{:?}", news);
                                                    },
                                                    Err(err) => error!("Cannot get article content of {} ({}): {}", news.title, news.url, err),
                                                }
                                            }
                                            });
                                        }
                                    },
                                    Err(e) => {
                                        error!("Failed to parse channel from {} feed: {:?}", url, e);
                                    },
                                }
                            },
                            Err(e) => {
                                error!(
                                    "Failed to get content from {} feed: {:?}",
                                    url, e
                                );
                            },
                        }
                    } else {
                        error!("Failed to send request to {} feed.", url);
                    }
                }
            }
        });

        Ok(())
    }
}
