use crate::models::{image::Image, news::News, source::Media};
use juniper::{graphql_object, EmptyMutation, EmptySubscription, RootNode};

/// Define the context for your GraphQL schema.
#[derive(Clone)]
pub struct Context {}
impl juniper::Context for Context {}

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

/// Define the root query object.
#[derive(Clone, Copy, Debug)]
pub struct Query;

/// Implement the GraphQL object for the root query.
#[graphql_object(context = Context)]
impl Query {
    /// Define an asynchronous method to retrieve a user by vanity
    async fn get_news(
        context: &Context,
        country: String,
        limit: i32,
    ) -> Vec<News> {
        vec![News {
            ..Default::default()
        }]
    }
}

/// Define the schema using RootNode
type Schema = RootNode<
    'static,
    Query,
    EmptyMutation<Context>,
    EmptySubscription<Context>,
>;

/// Create the schema instance
pub fn schema() -> Schema {
    Schema::new(
        Query,
        EmptyMutation::<Context>::new(),
        EmptySubscription::<Context>::new(),
    )
}
