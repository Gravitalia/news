//! polymath based web scraper.

use polymath_crawler::extractor::meta::Meta;
use polymath_crawler::Event;
use scraper::{Html, Selector};
use std::collections::HashMap;
use std::{fmt::Debug, sync::Arc};
use tokio::sync::RwLock;

/// Custom event manager.
#[derive(Debug)]
pub struct News;

impl Event for News {
    fn before_request(&self, _: &str) -> Result<(), polymath_error::Error> {
        Ok(())
    }

    fn after_request(
        &self,
        _title: &str,
        _: Vec<Meta>,
        _html: &str,
    ) -> Result<(), polymath_error::Error> {
        // Process or analyze the HTML content here.
        // You can also save result on a database.
        Ok(())
    }
}

/// Attribute to extract content from HTML content.
#[derive(Debug, Default)]
pub struct Extractor {
    /// Content extraction by [class](https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/class) attribute.
    pub class: Option<String>,
    /// Content extraction by [id](https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/id) attribute.
    pub id: Option<String>,
}

/// Extract the written content of the article.
pub async fn extract_article_content(
    extraction: Arc<RwLock<HashMap<String, Extractor>>>,
    url: &str,
    html_content: &str,
) -> String {
    if let Some(what_to_extract) = extraction.read().await.get(url) {
        let document = Html::parse_document(html_content);

        let text = if let Some(class) = &what_to_extract.class {
            if let Ok(selector) = Selector::parse(&format!(".{}", class)) {
                document
                    .select(&selector)
                    .flat_map(|e| e.text())
                    .collect::<Vec<_>>()
                    .concat()
            } else {
                String::default()
            }
        } else if let Some(id) = &what_to_extract.id {
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
