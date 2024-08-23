use crate::helpers::ranking::Ranker;
use crate::helpers::summary;
use crate::media::fr::French;
use crate::media::us::UnitedStates;
use crate::models::{
    image::{Image, Scheme},
    news::News,
    source::Media as Source,
};
use chrono::Utc;
use crawler::RssNews;
use search::Search;
use std::sync::Arc;
use url::Url;

#[derive(Debug, Default)]
struct Media {
    name: String,
    url: String,
    country: String,
}

/// Find a media based on its article URL.
fn find_media(from_url: &str) -> Media {
    if let Some(media) = French::from_url(from_url) {
        Media {
            name: media.name().to_owned(),
            url: media.url().to_owned(),
            country: "fr".to_owned(),
        }
    } else if let Some(media) = UnitedStates::from_url(from_url) {
        Media {
            name: media.name().to_owned(),
            url: media.url().to_owned(),
            country: "us".to_owned(),
        }
    } else {
        Media::default()
    }
}

/// Handling incoming messages from MPSC channel.
pub async fn process_article(
    article: RssNews,
    searcher: &Arc<Search>,
    ranker: &mut Ranker,
) -> Result<(), Box<dyn std::error::Error>> {
    let summary = summary::get_summary(&article.content).await?;

    let image = if let Some(img) = article.image {
        let url = Url::parse(&img)?;
        Image {
            host: url.host_str().unwrap_or_default().to_owned(),
            path: url.path().to_owned(),
            full_url: url.into(),
            scheme: Scheme::Https,
        }
    } else {
        Image::default()
    };

    let media = find_media(&article.url);
    if !media.name.is_empty() {
        let source = Source {
            country: media.country,
            media_url: media.url,
            media_image: Image {
                host: "news.gravitalia.com".to_owned(),
                path: format!("/media/{}.png", media.name),
                full_url: format!(
                    "https://news.gravitalia.com/media/{}.png",
                    media.name
                ),
                scheme: Scheme::Https,
            },
            name: media.name,
            url: article.url,
        };

        let news = News {
            id: uuid::Uuid::new_v4().into(),
            title: article.title,
            description: article.description.unwrap_or_default(),
            content: article.content,
            published_at: article
                .date
                .unwrap_or(Utc::now().into())
                .with_timezone(&Utc),
            image,
            similar: Vec::new(),
            source,
            summary,
        };

        ranker.add_entry(&news.title).await?;

        searcher.add_entry(news).await.map_err(|e| e.into())
    } else {
        Err("No media found with this URL".into())
    }
}
