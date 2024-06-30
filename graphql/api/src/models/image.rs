use juniper::{GraphQLEnum, GraphQLObject};

#[derive(Clone, Debug, Default, GraphQLEnum)]
pub enum Scheme {
    #[default]
    Http,
    Https,
}

/// A structure representing an image.
#[derive(Clone, Debug, Default, GraphQLObject)]
pub struct Image {
    pub host: String,
    pub path: String,
    pub full_url: String,
    pub scheme: Scheme,
}
