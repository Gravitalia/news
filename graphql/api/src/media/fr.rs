//! France (FR) medias.

enum French {
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
    fn name(&self) -> &'static str {
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

    fn url(&self) -> &'static str {
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

    fn rss(&self) -> Option<&'static str> {
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
}
