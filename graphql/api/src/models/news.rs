use crate::models::image::Image;
use crate::models::source::Media;

/// A structure representing a news article.
#[derive(Clone, Debug, Default)]
#[allow(dead_code)]
pub struct News {
    pub title: String,
    pub description: String,
    pub(crate) content: String,
    pub published_at: i32,
    pub image: Image,
    pub source: Media,
    pub similar: Vec<News>,
}
