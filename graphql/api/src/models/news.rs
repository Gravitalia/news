use crate::models::image::Image;
use crate::models::source::Media;
use crate::schema::Date;
use serde::{Deserialize, Serialize};

/// A structure representing a news article.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct News {
    /// UUID for identifying the news article.
    pub id: String,
    /// Title of the news article.
    pub title: String,
    /// A brief description of the news article.
    pub description: String,
    /// Written content by authors of the news article.
    pub content: String,
    /// The date when the news article was published.
    pub published_at: Date,
    /// An associated image with the news article.
    pub image: Image,
    /// A list of similar news articles for recommendations or related content.
    pub similar: Vec<News>,
    /// The source of the news article.
    pub source: Media,
    /// A ML-genereated summary of the news article.
    pub summary: String,
}
