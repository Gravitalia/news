use crate::models::{image::Image, news::News, source::Media};
use crate::Context;
use juniper::graphql_object;

/// Implement GraphQL on News structure.
#[graphql_object(context = Context, description = "A media article.")]
impl News {
    fn title(&self) -> &str {
        &self.title
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn published_at(&self) -> i32 {
        self.published_at
    }

    fn image(&self) -> &Image {
        &self.image
    }

    fn similar(&self) -> &Vec<News> {
        &self.similar
    }

    fn source(&self) -> &Media {
        &self.source
    }

    fn summary(&self) -> &String {
        &self.summary
    }
}

/// Define the news query object.
#[derive(Clone, Copy, Debug)]
pub struct NewsQuery;

/// Implement the GraphQL object for the news query.
#[graphql_object(context = Context)]
impl NewsQuery {
    /// Get the most relevant news of the day.
    async fn get_news(
        _ctx: &Context,
        #[graphql(description = "ISO 3166-1 alpha-2 country code.")]
        _country: String,
        #[graphql(description = "Maximum number of articles sent.")]
        _limit: i32,
    ) -> Vec<News> {
        vec![News {
            ..Default::default()
        }]
    }
}
