//! United States (US) of America medias.

enum UnitedStates {
    Vox,
    NationalPublicRadio,
    CableNewsNetwork,
    NewYorkTimes,
    Forbes,
    WallStreetJournal,
    WashingtonPost,
    FoxNews,
}

impl UnitedStates {
    fn name(&self) -> &'static str {
        match self {
            UnitedStates::Vox => "Vox",
            UnitedStates::NationalPublicRadio => "National Public Radio (NPR)",
            UnitedStates::CableNewsNetwork => "Cable News Network (CNN)",
            UnitedStates::NewYorkTimes => "The New York Times",
            UnitedStates::Forbes => "Forbes",
            UnitedStates::WallStreetJournal => "The Wall Street Journal",
            UnitedStates::WashingtonPost => "The Washington Post",
            UnitedStates::FoxNews => "Fox News",
        }
    }

    fn url(&self) -> &'static str {
        match self {
            UnitedStates::Vox => "https://www.vox.com",
            UnitedStates::NationalPublicRadio => "https://www.npr.org/",
            UnitedStates::CableNewsNetwork => "https://www.cnn.com",
            UnitedStates::NewYorkTimes => "https://www.nytimes.com",
            UnitedStates::Forbes => "https://www.forbes.com",
            UnitedStates::WallStreetJournal => "https://www.wsj.com",
            UnitedStates::WashingtonPost => "https://www.washingtonpost.com",
            UnitedStates::FoxNews => "https://www.foxnews.com",
        }
    }

    fn rss(&self) -> Option<&'static str> {
        match self {
            UnitedStates::Vox => Some("https://www.vox.com/rss/index.xml"),
            UnitedStates::NationalPublicRadio => None,
            UnitedStates::CableNewsNetwork => Some("http://rss.cnn.com/rss/cnn_topstories.rss"), // Not secure.
            UnitedStates::NewYorkTimes => {
                Some("https://rss.nytimes.com/services/xml/rss/nyt/HomePage.xml")
            }
            UnitedStates::Forbes => None,
            UnitedStates::WallStreetJournal => Some("https://feeds.a.dj.com/rss/RSSWorldNews.xml"), // Reader looks broken on Chrome.
            UnitedStates::WashingtonPost => Some("https://feeds.washingtonpost.com/rss/world"),
            UnitedStates::FoxNews => Some("https://moxie.foxnews.com/google-publisher/latest.xml"),
        }
    }
}
