use crate::models::image::Image;
use juniper::GraphQLObject;
use serde::{Serialize, Deserialize};

/// A structure representing where the article has been taken.
#[derive(Clone, Debug, Default, GraphQLObject, Serialize, Deserialize)]
pub struct Media {
    pub name: String,
    pub url: String,
    pub media_url: String,
    pub media_image: Image,
}
