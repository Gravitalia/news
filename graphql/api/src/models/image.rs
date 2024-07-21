use juniper::{GraphQLEnum, GraphQLObject};
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Default, GraphQLEnum, Serialize, Deserialize)]
pub enum Scheme {
    #[default]
    Http,
    Https,
}

/// A structure representing an image.
#[derive(Clone, Debug, Default, GraphQLObject, Serialize, Deserialize)]
pub struct Image {
    pub host: String,
    pub path: String,
    pub full_url: String,
    pub scheme: Scheme,
}
