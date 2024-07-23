use crate::models::{mcq::Question, news::News};
use crate::Context;
use juniper::graphql_object;

/// Implement GraphQL on Question structure.
#[graphql_object(context = Context, description = "A question, its choices and answers.")]
impl Question {
    fn question(&self) -> &str {
        &self.question
    }

    fn choices(&self) -> &Vec<String> {
        &self.choices
    }

    fn answer(&self) -> &str {
        &self.answer
    }

    fn article(&self) -> &News {
        &self.article
    }
}

/// Define the question query object.
#[derive(Clone, Copy, Debug)]
pub struct QuestionQuery;

/// Implement the GraphQL object for the question query.
#[graphql_object(context = Context)]
impl QuestionQuery {
    /// Get the 3 questions of the day.
    async fn get_mcq(
        _ctx: &Context,
        #[graphql(description = "ISO 3166-1 alpha-2 country code.")]
        _country: String,
    ) -> Vec<Question> {
        vec![Question {
            ..Default::default()
        }]
    }
}
