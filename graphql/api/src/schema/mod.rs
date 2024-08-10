mod mcq;
mod news;

use juniper::{EmptyMutation, EmptySubscription, RootNode};
use mcq::QuestionQuery;
use news::NewsQuery;
use search::Search;
use std::sync::Arc;

/// Define the context for the GraphQL schema.
#[derive(Clone, Debug)]
pub struct Context {
    pub meilisearch: Arc<Search>,
}
impl juniper::Context for Context {}

/// Define the root query object.
#[derive(Clone, Copy, Debug)]
pub struct Query;

#[juniper::graphql_object(context = Context)]
impl Query {
    /// Everything related to raw news.
    fn news() -> NewsQuery {
        NewsQuery
    }

    /// Everything related multiple-choice question.
    fn question() -> QuestionQuery {
        QuestionQuery
    }
}

/// Define the schema using RootNode.
type Schema = RootNode<
    'static,
    Query,
    EmptyMutation<Context>,
    EmptySubscription<Context>,
>;

/// Create the schema instance.
pub fn schema() -> Schema {
    Schema::new(
        Query,
        EmptyMutation::<Context>::new(),
        EmptySubscription::<Context>::new(),
    )
}
