//! Create a ranking of most important news.

use error::{Error, ErrorType::*};
use rank::squid::Squid;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::error;

/// Multiple source ranker.
#[derive(Clone, Debug)]
pub struct Ranker {
    squid: Option<Arc<RwLock<Squid>>>,
}

impl Ranker {
    /// Create a new [`Ranker`].
    pub async fn new() -> Result<Self, Error> {
        let squid = match Squid::init(
            std::env::var("SQUID_URL").unwrap_or("http://[::1]:50051".into()),
        )
        .await
        {
            Ok(squid) => Some(Arc::new(RwLock::new(squid))),
            Err(error) => {
                error!("Failed to create Squid connection: {}", error);
                None
            },
        };

        Ok(Ranker { squid })
    }

    /// Get a single ranking from multiple sources.
    pub async fn get_rank(&self, length: u32) -> Result<Vec<String>, Error> {
        let mut result = Vec::new();

        if let Some(squid) = &self.squid {
            result.append(
                &mut squid.write().await.leaderboard(length).await.map_err(
                    |error| {
                        Error::new(
                            Unspecified,
                            Some(Box::new(error)),
                            Some("getting Squid leaderboard".into()),
                        )
                    },
                )?,
            );
        }

        Ok(result)
    }

    /// Add a single entry to multiple rankers.
    pub async fn add_entry(&mut self, text: &str) -> Result<(), Error> {
        if let Some(squid) = &self.squid {
            squid
                .write()
                .await
                .add_entry(text.to_owned())
                .await
                .map_err(|error| {
                    Error::new(Unspecified, Some(Box::new(error)), None)
                })?;
        }

        Ok(())
    }
}
