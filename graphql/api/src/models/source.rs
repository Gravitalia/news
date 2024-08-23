use crate::models::image::Image;
use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};

/// A structure representing where the article has been taken.
#[derive(Clone, Debug, Default, GraphQLObject, Serialize, Deserialize)]
pub struct Media {
    /// The country where the media is based.
    pub country: String,
    /// Website homepage of media.
    pub media_url: String,
    /// The media favicon.
    pub media_image: Image,
    /// The name of the media.
    pub name: String,
    /// URL of the article.
    pub url: String,
}
