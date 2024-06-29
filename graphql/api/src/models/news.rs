use crate::models::image::Image;
use crate::models::source::Media;

/// A structure representing a news article.
#[derive(Clone, Debug)]
pub struct News {
    pub title: String,
    pub description: String,
    content: String,
    pub published_at: i32,
    pub image: Image,
    pub source: Media,
    pub similar: Vec<News>,
}
