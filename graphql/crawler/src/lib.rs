#![forbid(unsafe_code)]
#![deny(dead_code, unused_imports, unused_mut, missing_docs)]
//! Fetches news from RSS feeds provided by various media outlets.
//! It then uses a [Polymath](https://github.com/Lubmminy/Polymath) extension (crawler) to retrieve the full news content.

use chrono::{DateTime, FixedOffset};
use reqwest::Client;
use rss::Channel;
use std::{convert::Infallible, time::Duration};
use tokio::{sync::mpsc::Sender, task::spawn, time::interval};
use tracing::{debug, error, info};

/// Represents a news article in an RSS feed.
pub struct RssNews {
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
    delay: Duration,
    _content_crawl_delay: Duration,
    feeds: Vec<String>,
    channel: Option<Sender<RssNews>>,
}

impl Crawler {
    /// Create a new [`Crawler`] with a specified delay between each RSS feed crawl.
    pub fn new(delay: Duration) -> Self {
        Crawler {
            client: Client::new(),
            delay,
            _content_crawl_delay: Duration::from_secs(60),
            feeds: Vec::new(),
            channel: None,
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
                                        for item in channel.items() {
                                            let _news = RssNews {
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
