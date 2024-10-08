mod mcq;
mod news;

use crate::helpers::ranking::Ranker;
use chrono::{DateTime, Utc};
use juniper::{EmptyMutation, EmptySubscription, RootNode};
use mcq::QuestionQuery;
use news::NewsQuery;
use search::Search;
use std::sync::Arc;
use tokio::sync::RwLock;

pub type Date = DateTime<Utc>;

/// Define the context for the GraphQL schema.
#[derive(Clone, Debug)]
pub struct Context {
    /// [`search::Search`] meilisearch client.
    pub meilisearch: Arc<RwLock<Search>>,
    /// Custom [`Ranker`] supporting multiple sources.
    pub ranker: Ranker,
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
