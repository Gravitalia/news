//! United States (US) of America medias.

use crawler::scraper::{Attribute, Extract};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter)]
pub enum UnitedStates {
    NationalPublicRadio,
    CableNewsNetwork,
    NewYorkTimes,
    Forbes,
    WashingtonPost,
    FoxNews,
}

impl UnitedStates {
    pub fn name(&self) -> &'static str {
        match self {
            UnitedStates::NationalPublicRadio => "National Public Radio (NPR)",
            UnitedStates::CableNewsNetwork => "Cable News Network (CNN)",
            UnitedStates::NewYorkTimes => "The New York Times",
            UnitedStates::Forbes => "Forbes",
            UnitedStates::WashingtonPost => "The Washington Post",
            UnitedStates::FoxNews => "Fox News",
        }
    }

    pub fn url(&self) -> &'static str {
        match self {
            UnitedStates::NationalPublicRadio => "https://www.npr.org/",
            UnitedStates::CableNewsNetwork => "https://www.cnn.com",
            UnitedStates::NewYorkTimes => "https://www.nytimes.com",
            UnitedStates::Forbes => "https://www.forbes.com",
            UnitedStates::WashingtonPost => "https://www.washingtonpost.com",
            UnitedStates::FoxNews => "https://www.foxnews.com",
        }
    }

    pub fn rss(&self) -> Option<&'static str> {
        match self {
            UnitedStates::NationalPublicRadio => None,
            UnitedStates::CableNewsNetwork => {
                Some("http://rss.cnn.com/rss/cnn_topstories.rss")
            }, // Not secure.
            UnitedStates::NewYorkTimes => Some(
                "https://rss.nytimes.com/services/xml/rss/nyt/HomePage.xml",
            ),
            UnitedStates::Forbes => None,
            UnitedStates::WashingtonPost => {
                Some("https://feeds.washingtonpost.com/rss/world")
            },
            UnitedStates::FoxNews => {
                Some("https://moxie.foxnews.com/google-publisher/latest.xml")
            },
        }
    }

    pub fn extractor(&self) -> Extract {
        match self {
            UnitedStates::NationalPublicRadio => Extract {
                content: Attribute {
                    class: Some("storytext".to_owned()),
                    id: None, // `storytext` also work.
                },
                image: Attribute {
                    class: Some("img".to_owned()),
                    id: None,
                },
            },
            UnitedStates::CableNewsNetwork => Extract {
                content: Attribute {
                    class: Some("article__content".to_owned()),
                    id: None,
                },
                image: Attribute {
                    class: Some("sc-".to_owned()),
                    id: None,
                },
            },
            UnitedStates::NewYorkTimes => Extract {
                content: Attribute {
                    class: Some("meteredContent".to_owned()),
                    id: None,
                },
                image: Attribute {
                    class: Some("css-".to_owned()),
                    id: None,
                },
            },
            UnitedStates::Forbes => Extract {
                content: Attribute {
                    class: Some("article-body".to_owned()),
                    id: None,
                },
                image: Attribute {
                    class: Some("image-embed".to_owned()),
                    id: None,
                },
            },
            UnitedStates::WashingtonPost => Extract {
                content: Attribute {
                    class: Some("meteredContent".to_owned()),
                    id: None,
                },
                image: Attribute {
                    class: Some("w-100".to_owned()), // Look, they use tailwind.
                    id: None,
                },
            },
            UnitedStates::FoxNews => Extract {
                content: Attribute {
                    class: Some("article-body".to_owned()),
                    id: None,
                },
                image: Attribute {
                    class: Some("".to_owned()),
                    id: None,
                },
            },
        }
    }

    pub fn from_url(url: &str) -> Option<UnitedStates> {
        UnitedStates::iter().find(|media| url.starts_with(media.url()))
    }
}
