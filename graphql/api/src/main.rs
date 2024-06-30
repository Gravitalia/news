#![forbid(unsafe_code)]
#![deny(dead_code, unused_imports, unused_mut, missing_docs)]
//! GraphQL API.

mod media;
mod models;
mod news;

use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};
use warp::Filter;

const DEFAULT_PORT: u16 = 5400;

#[tokio::main]
async fn main() {
    let fmt_layer = fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true);

    tracing_subscriber::registry().with(fmt_layer).init();

    // Create a filter for the main GraphQL endpoint.
    let context = warp::any().map(move || news::Context {});
    let graphql_filter = juniper_warp::make_graphql_filter(news::schema(), context);

    warp::serve(
        warp::any()
            .and(warp::options())
            .map(|| "OK")
            .or(warp::post()
                .and(warp::path("graphql").and(graphql_filter))
                .with(warp::log("warp_server")))
            .or(warp::get()
                .and(warp::path("graphiql"))
                .and(juniper_warp::graphiql_filter(
                    "/graphql",
                    Some("/subscriptions"),
                ))),
    )
    .run((
        [127, 0, 0, 1],
        std::env::var("PORT")
            .unwrap_or_else(|_| DEFAULT_PORT.to_string())
            .parse()
            .unwrap(),
    ))
    .await
}
