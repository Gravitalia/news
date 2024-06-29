mod models;
mod news;
mod media;

use warp::Filter;

pub const EPOCH: i32 = 1704067200;

#[tokio::main]
async fn main() {
    let context = warp::any().map(move || news::Context {
    });

    // Create a filter for the main GraphQL endpoint.
    let graphql_filter = juniper_warp::make_graphql_filter(news::schema(), context);

    warp::serve(
        warp::any()
            .and(warp::options())
            .map(|| "OK")
            .or(warp::post()
                .and(warp::path("graphql").and(graphql_filter))
                .with(warp::log("warp_server"))),
    )
    .run(([127, 0, 0, 1], 80))
    .await
}
