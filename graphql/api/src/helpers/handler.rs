use crate::helpers::summary;
use crate::media::fr::French;
use crate::models::{
    image::{Image, Scheme},
    news::News,
    source::Media,
};
use chrono::Utc;
use crawler::RssNews;
use search::Search;
use std::sync::Arc;
use url::Url;

/// Handling incoming messages from MPSC channel.
pub async fn process_article(
    article: RssNews,
    searcher: &Arc<Search>,
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

    if let Some(media) = French::from_url(&article.url) {
        let source = Media {
            name: media.name().to_owned(),
            url: article.url.clone(),
            media_url: media.url().to_owned(),
            media_image: Image {
                host: "news.gravitalia.com".to_owned(),
                path: format!("/media/{}.png", media.name()),
                full_url: format!(
                    "https://news.gravitalia.com/media/{}.png",
                    media.name()
                ),
                scheme: Scheme::Https,
            },
        };

        let news = News {
            id: uuid::Uuid::new_v4().into(),
            title: article.title.clone(),
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

        searcher.add_entry(news).await.map_err(|e| e.into())
    } else {
        Err("dd".into())
    }
}
