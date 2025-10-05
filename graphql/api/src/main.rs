#![forbid(unsafe_code)]
#![deny(unused_imports, unused_mut, missing_docs)]
//! GraphQL API.

mod media;
mod models;
mod schema;
mod services;

use crawler::{cache::Cache, Crawler};
use search::{Attributes, Search};
use services::ranking::Ranker;
use std::{sync::Arc, time::Duration};
use strum::IntoEnumIterator;
use tokio::sync::mpsc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, Level};
use tracing_subscriber::fmt;
use url::Url;
use warp::Filter;

use crate::models::news::News;
use crate::schema::*;
use crate::services::summary::Sum;

const DEFAULT_PORT: u16 = 5400;
const LRU_CAPACITY: usize = 100;

impl Attributes for News {
    fn primary_key(&self) -> Option<&str> {
        Some("id")
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    fmt()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_max_level(Level::INFO)
        .init();

    let cache = if let Ok(url) = std::env::var("MEMCACHED_URL") {
        let manager = r2d2_memcache::MemcacheConnectionManager::new(url);
        let pool = r2d2_memcache::r2d2::Pool::builder()
            .max_size(15)
            .build(manager)?;
        info!("created memcached pool");
        Cache::new(LRU_CAPACITY).memcached(pool)
    } else {
        Cache::new(LRU_CAPACITY)
    };

    let mut feeds = Vec::new();
    let mut crawler = Crawler::new(Duration::from_secs(300)).cache(cache); // 5 minutes.

    // Add method to extract from French medias.
    for variant in media::fr::French::iter() {
        // Add RSS feed on feeds content.
        if let Some(rss) = variant.rss() {
            feeds.push(rss.to_owned());
        }

        // Help crawler (scraper) finding article content by adding class or id attribute of article content.
        if let Ok(host) = Url::parse(variant.url())
            .map_err(|e| format!("Invalid URL: {}", e))
            .and_then(|url| {
                url.host_str()
                    .map(|host| host.to_owned())
                    .ok_or_else(|| "No host found in URL".to_owned())
            })
        {
            debug!(
                "add {:?} method extractor for {}",
                variant.extractor(),
                host
            );
            crawler
                .extraction
                .write()
                .await
                .insert(host, variant.extractor());
        }
    }

    // Add method to extract from US medias.
    for variant in media::us::UnitedStates::iter() {
        // Add RSS feed on feeds content.
        if let Some(rss) = variant.rss() {
            feeds.push(rss.to_owned());
        }

        // Help crawler (scraper) finding article content by adding class or id attribute of article content.
        if let Ok(host) = Url::parse(variant.url())
            .map_err(|e| format!("Invalid URL: {}", e))
            .and_then(|url| {
                url.host_str()
                    .map(|host| host.to_owned())
                    .ok_or_else(|| "No host found in URL".to_owned())
            })
        {
            debug!(
                "add {:?} method extractor for {}",
                variant.extractor(),
                host
            );
            crawler
                .extraction
                .write()
                .await
                .insert(host, variant.extractor());
        }
    }

    crawler.feeds(feeds);

    // Create MPSC channel.
    let (tx, mut rx) = mpsc::channel(100);
    crawler.channel(tx);

    // Start crawling medias.
    crawler.crawl()?;

    // Create meilisearch client.
    let searcher = Arc::new(RwLock::new(
        Search::new(
            std::env::var("MEILISEARCH_URL")
                .unwrap_or("http://localhost:7700".into()),
            std::env::var("MEILISEARCH_URL").ok(),
        )?
        .index("news".into())
        .await,
    ));

    // Add country field as a filterable attribute.
    searcher
        .write()
        .await
        .index
        .as_ref()
        .unwrap()
        .set_filterable_attributes(&["source.country"])
        .await?;

    // Create ranking platform.
    let ranker = Ranker::new().await?;
    // Create summary platform.
    let sum = Sum::new(
        std::env::var("SUMMARY_URL")
            .unwrap_or("http://localhost:8000".to_string()),
    )?;

    // Create a filter for the main GraphQL endpoint.
    let ctx_ranker = ranker.clone();
    let ctx_searcher = Arc::clone(&searcher);
    let context = warp::any().map(move || Context {
        meilisearch: Arc::clone(&ctx_searcher),
        ranker: ctx_ranker.clone(),
    });
    let graphql_filter = juniper_warp::make_graphql_filter(schema(), context);

    // Create receiver of crawled articles.
    tokio::spawn(async move {
        while let Some(i) = rx.recv().await {
            if let Err(err) = crate::services::handler::process_article(
                i,
                &sum,
                &searcher,
                &mut ranker.clone(),
            )
            .await
            {
                error!(%err, "failed to process article");
            }
        }
    });

    warp::serve(
        warp::any()
            .and(warp::options())
            .map(|| "OK")
            .or(warp::post()
                .and(warp::path("graphql").and(graphql_filter))
                .with(warp::log("warp_server")))
            .or(warp::get().and(warp::path("graphiql")).and(
                juniper_warp::graphiql_filter(
                    "/graphql",
                    Some("/subscriptions"),
                ),
            )),
    )
    .run((
        [0, 0, 0, 0],
        std::env::var("PORT")
            .unwrap_or_else(|_| DEFAULT_PORT.to_string())
            .parse()?,
    ))
    .await;

    Ok(())
}
