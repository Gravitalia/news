//! polymath based web scraper.

use scraper::{Html, Selector};
use std::collections::HashMap;
use std::{fmt::Debug, sync::Arc};
use tokio::sync::RwLock;

/// Attribute to extract content from HTML content.
#[derive(Debug, Default)]
pub struct Attribute {
    /// Extraction by [class](https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/class) attribute.
    pub class: Option<String>,
    /// Extraction by [id](https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/id) attribute.
    pub id: Option<String>,
}

/// Attribute to extract content from HTML content.
#[derive(Debug, Default)]
pub struct Extract {
    /// Extract article content.
    pub content: Attribute,
    /// Extract article image.
    /// Do not bypass RSS-provided image.
    pub image: Attribute,
}

/// Extract elements from selected attributes on HTML.
pub struct Extractor {
    extraction: Arc<RwLock<HashMap<String, Extract>>>,
    url: String,
    html: String,
}

impl Extractor {
    /// Create a new [`Extractor`].
    pub fn new(
        extraction: Arc<RwLock<HashMap<String, Extract>>>,
        url: &str,
        html_content: &str,
    ) -> Self {
        Extractor {
            extraction,
            url: url.to_owned(),
            html: html_content.to_owned(),
        }
    }

    /// Extract the written content of the article.
    pub async fn extract_content(&self) -> String {
        if let Some(what_to_extract) =
            self.extraction.read().await.get(&self.url)
        {
            let document = Html::parse_document(&self.html);

            let text = if let Some(class) = &what_to_extract.content.class {
                match Selector::parse(&format!(".{}", class)) {
                    Ok(selector) => document
                        .select(&selector)
                        .flat_map(|e| e.text())
                        .collect::<Vec<_>>()
                        .concat(),
                    Err(_) => String::default(),
                }
            } else if let Some(id) = &what_to_extract.content.id {
                if let Ok(selector) = Selector::parse(&format!("#{}", id)) {
                    document
                        .select(&selector)
                        .next()
                        .map(|e| e.text().collect::<Vec<_>>().concat())
                        .unwrap_or_default()
                } else {
                    String::default()
                }
            } else {
                String::default()
            };

            text
        } else {
            String::default()
        }
    }

    /// Extract the image URL of the article.
    pub async fn extract_image(&self) -> Option<String> {
        if let Some(what_to_extract) =
            self.extraction.read().await.get(&self.url)
        {
            let document = Html::parse_document(&self.html);

            let class = what_to_extract.image.class.as_ref()?;

            // Create the CSS selector.
            let selector =
                Selector::parse(&format!("img[class*=\"{}\"]", class)).ok()?;
            let src =
                document.select(&selector).next().and_then(|element| {
                    element.value().attr("src").map(String::from)
                })?;

            if src.starts_with('/') {
                Some(format!("https://{}{}", self.url, src))
            } else {
                Some(src)
            }
        } else {
            None
        }
    }
}
