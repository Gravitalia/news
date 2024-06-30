use crate::models::image::Image;
use juniper::GraphQLObject;

/// A structure representing where the article has been taken.
#[derive(Clone, Debug, Default, GraphQLObject)]
pub struct Media {
    pub name: String,
    pub url: String,
    pub media_url: String,
    pub media_image: Image,
}
