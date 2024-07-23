use crate::models::news::News;
use serde::{Deserialize, Serialize};

/// A structure representing a question of a multiple-choice question (MCQ).
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Question {
    pub question: String,
    pub choices: Vec<String>,
    pub answer: String,
    pub article: News,
}
