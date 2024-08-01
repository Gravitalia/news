#![forbid(unsafe_code)]
#![deny(dead_code, unused_imports, unused_mut, missing_docs)]
//! GraphQL API.

mod media;
mod models;
mod schema;

use crate::schema::*;
use crawler::Crawler;
use std::time::Duration;
use strum::IntoEnumIterator;
use tracing::debug;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};
use url::Url;
use warp::Filter;

const DEFAULT_PORT: u16 = 5400;

#[tokio::main]
async fn main() {
    let fmt_layer = fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true);

    tracing_subscriber::registry().with(fmt_layer).init();

    let mut feeds = Vec::new();
    let mut crawler = Crawler::new(Duration::from_secs(300)); // 5 minutes.

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
                "Add {:?} method extractor for {}",
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
    crawler.crawl().unwrap();

    // Create a filter for the main GraphQL endpoint.
    let context = warp::any().map(move || Context {});
    let graphql_filter = juniper_warp::make_graphql_filter(schema(), context);

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
            .parse()
            .unwrap(),
    ))
    .await
}
