#[derive(Clone, Debug)]
enum Scheme {
    Http,
    Https,
}

/// A structure representing an image.
#[derive(Clone, Debug)]
pub struct Image {
    pub host: String,
    pub path: String,
    pub full_url: String,
    pub scheme: Scheme,
}
