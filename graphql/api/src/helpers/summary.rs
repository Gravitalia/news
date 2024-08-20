use std::convert::Infallible;

use reqwest::{Client, Error};

/// Turns a long article text into a short summary.
pub async fn get_summary(text: &str) -> Result<String, Error> {
    match Ok::<String, Infallible>("http://0.0.0.0:8000".to_string()) /*std::env::var("SUMMARY_URL")*/ {
        Ok(url) => {
            let response = Client::new().get(format!("{}/summary/", url))
            .query(&[("text", text)])
            .send()
            .await?;

            response.json().await
        },
        Err(_) => Ok(String::default()),
    }
}
