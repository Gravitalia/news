//! France (FR) medias.

use crawler::scraper::{Attribute, Extract};
use strum_macros::EnumIter;

#[derive(Debug, EnumIter)]
pub enum French {
    LHumanité,
    Libération,
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

    pub fn extractor(&self) -> Extract {
        match self {
            French::LHumanité => Extract {
                content: Attribute {
                    class: Some("rich-text".to_owned()),
                    id: None,
                },
                image: Attribute {
                    class: Some("wp-image-".to_owned()),
                    id: None,
                },
            },
            French::Libération => Extract {
                content: Attribute {
                    class: Some("article-body-wrapper".to_owned()),
                    id: None,
                },
                image: Attribute {
                    class: Some("sc-".to_owned()),
                    id: None,
                },
            },
            French::LeMonde => Extract {
                content: Attribute {
                    class: Some("article__content".to_owned()),
                    id: None,
                },
                image: Attribute {
                    class: Some("lzld--loading".to_owned()),
                    id: None,
                },
            },
            French::LeParisien => Extract {
                content: Attribute {
                    class: Some("article-section".to_owned()),
                    id: None,
                },
                image: Attribute {
                    class: Some("image".to_owned()),
                    id: None,
                },
            },
            French::LesEchos => Extract {
                content: Attribute {
                    class: Some("post-paywall".to_owned()), // `post-paywall` seems to be the unchanged class; however we have `kbgxbh`.
                    id: None,
                },
                image: Attribute {
                    class: Some("sc-".to_owned()),
                    id: None,
                },
            },
            French::OuestFrance => Extract {
                content: Attribute {
                    class: None, // `contenu-principal` may work.
                    id: Some("article-detail".to_owned()),
                },
                image: Attribute {
                    class: Some("su-media".to_owned()),
                    id: None,
                },
            },
            French::LePoint => Extract {
                content: Attribute {
                    class: None, // `article-styles` may work.
                    id: Some("contenu".to_owned()),
                },
                image: Attribute {
                    class: None,
                    id: None,
                },
            },
            French::LExpress => Extract {
                content: Attribute {
                    class: Some("qiota_reserve".to_owned()),
                    id: None,
                },
                image: Attribute {
                    class: Some("illustration__image".to_owned()), // `img_resp_full` should also work.
                    id: None,
                },
            },
            French::Marianne => Extract {
                content: Attribute {
                    class: Some("article__content".to_owned()), // `article__wrapper` should also work.
                    id: None,
                },
                image: Attribute {
                    class: Some("responsive-image".to_owned()),
                    id: None,
                },
            },
            French::ValeursActuelles => Extract {
                content: Attribute {
                    class: Some("post__content".to_owned()),
                    id: None,
                },
                image: Attribute {
                    class: Some("attachment-post-thumbnail".to_owned()),
                    id: None,
                },
            },
        }
    }
}
