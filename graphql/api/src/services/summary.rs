use error::BError;
use reqwest::Client;

use std::str::FromStr;

const SUMMARY_PATH: &str = "summary";

/// Summary service manager.
#[derive(Debug, Clone)]
pub struct Sum {
    client: Client,
    endpoint: url::Url,
}

impl Sum {
    /// Create a new [`Sum`] manager.
    pub fn new<T: ToString>(endpoint: T) -> Result<Self, url::ParseError> {
        Ok(Self {
            client: Client::new(),
            endpoint: url::Url::from_str(&endpoint.to_string())?,
        })
    }

    /// Sum a news article.
    pub async fn sum(&self, text: &str) -> Result<String, BError> {
        let url = self.endpoint.join(SUMMARY_PATH)?;
        let response =
            self.client.get(url).query(&[("text", text)]).send().await?;

        Ok(response.json().await?)
    }
}
