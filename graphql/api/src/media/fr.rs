//! France (FR) medias.

use crawler::scraper::Extractor;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter)]
pub enum French {
    LHumanité,
    Libération,
    LaCroix,
    LeMonde,
    LeParisien,
    LesEchos,
    OuestFrance,
    LePoint,
    LExpress,
    Marianne,
    ValeursActuelles,
}

impl French {
    pub fn _name(&self) -> &'static str {
        match self {
            French::LHumanité => "L'Humanité",
            French::Libération => "Libération",
            French::LaCroix => "La Croix",
            French::LeMonde => "Le Monde",
            French::LeParisien => "Le Parisien",
            French::LesEchos => "Les Echos",
            French::OuestFrance => "Ouest-France",
            French::LePoint => "Le Point",
            French::LExpress => "L'Express",
            French::Marianne => "Marianne",
            French::ValeursActuelles => "Valeurs Actuelles",
        }
    }

    pub fn url(&self) -> &'static str {
        match self {
            French::LHumanité => "https://www.humanite.fr",
            French::Libération => "https://www.liberation.fr",
            French::LaCroix => "https://www.la-croix.com",
            French::LeMonde => "https://www.lemonde.fr",
            French::LeParisien => "https://www.leparisien.fr",
            French::LesEchos => "https://www.lesechos.fr",
            French::OuestFrance => "https://www.ouest-france.fr",
            French::LePoint => "https://www.lepoint.fr",
            French::LExpress => "https://www.lexpress.fr",
            French::Marianne => "https://www.marianne.net",
            French::ValeursActuelles => "https://www.valeursactuelles.com",
        }
    }

    pub fn rss(&self) -> Option<&'static str> {
        match self {
            French::LHumanité => None,
            French::Libération => {
                Some("https://www.liberation.fr/arc/outboundfeeds/rss-all/collection/accueil-une/")
            }
            French::LaCroix => Some("https://www.la-croix.com/RSS/UNIVERS"),
            French::LeMonde => Some("https://www.lemonde.fr/rss/une.xml"),
            French::LeParisien => Some("https://feeds.leparisien.fr/leparisien/rss"),
            French::LesEchos => Some("https://services.lesechos.fr/rss/les-echos-monde.xml"),
            French::OuestFrance => Some("https://www.ouest-france.fr/rss/une"),
            French::LePoint => None,
            French::LExpress => None,
            French::Marianne => Some("https://www.marianne.net/rss.xml"),
            French::ValeursActuelles => Some("https://www.valeursactuelles.com/feed"), // Reader looks broken on Chrome.
        }
    }

    pub fn extractor(&self) -> Extractor {
        match self {
            French::LHumanité => Extractor {
                class: Some("rich-text".to_owned()),
                id: None,
            },
            French::Libération => Extractor {
                class: Some("article-body-wrapper".to_owned()),
                id: None,
            },
            French::LaCroix => Extractor {
                class: Some("content".to_owned()), // `article-content-wrapper` should also work.
                id: None,
            },
            French::LeMonde => Extractor {
                class: Some("article__content".to_owned()),
                id: None,
            },
            French::LeParisien => Extractor {
                class: Some("article-section".to_owned()),
                id: None,
            },
            French::LesEchos => Extractor {
                class: Some("post-paywall".to_owned()), // `post-paywall` seems to be the unchanged class; however we have `kbgxbh`.
                id: None,
            },
            French::OuestFrance => Extractor {
                class: None, // `contenu-principal` may work.
                id: Some("article-detail".to_owned()),
            },
            French::LePoint => Extractor {
                class: None, // `article-styles` may work.
                id: Some("contenu".to_owned()),
            },
            French::LExpress => Extractor {
                class: Some("qiota_reserve".to_owned()),
                id: None,
            },
            French::Marianne => Extractor {
                class: Some("article__content".to_owned()), // `article__wrapper` should also work.
                id: None,
            },
            French::ValeursActuelles => Extractor {
                class: Some("post__content".to_owned()),
                id: None,
            },
        }
    }
}
