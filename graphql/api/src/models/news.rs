use crate::models::image::Image;
use crate::models::source::Media;
use serde::{Deserialize, Serialize};

/// A structure representing a news article.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct News {
    pub title: String,
    pub description: String,
    pub(crate) content: String,
    pub published_at: i32,
    pub image: Image,
    pub similar: Vec<News>,
    pub source: Media,
    pub summary: String,
}
