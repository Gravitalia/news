use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};
use crate::models::news::News;

/// A structure representing a question of a multiple-choice question (MCQ).
#[derive(Clone, Debug, Default, GraphQLObject, Serialize, Deserialize)]
pub struct Question {
    question: String,
    choices: Vec<String>,
    answer: String,
    article: News,
}
