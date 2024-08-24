#![forbid(unsafe_code)]
#![deny(dead_code, unused_imports, unused_mut, missing_docs)]
//! Searcher runtime.

use error::{Database, Error, ErrorType};
use meilisearch_sdk::{
    client::Client, errors::Error as MeiliError, indexes::Index,
};
use std::sync::Arc;

mod ttl;

/// Attributes required for TTL management.
pub trait Attributes {
    /// Unique identifier of entry used in Meilisearch.
    fn primary_key(&self) -> Option<&str> {
        None
    }
}

/// Meilisearch instance manager.
#[derive(Clone, Debug)]
pub struct Search {
    client: Client,
    /// Meilisearch index.
    pub index: Option<Index>,
    ttl_row: Option<String>,
}

impl Search {
    /// Create a new [`Search`] instance with a Meilisearch [`Client`].
    pub fn new(
        url: String,
        master_key: Option<String>,
    ) -> Result<Self, MeiliError> {
        Ok(Search {
            client: Client::new(url, master_key)?,
            index: None,
            ttl_row: None,
        })
    }

    /// Update Meilisearch index.
    /// Warning: DO NOT UPDATE IT.
    pub async fn index(mut self, index: String) -> Self {
        if let Ok(indexes) = self.client.list_all_indexes().await {
            if !indexes
                .results
                .iter()
                .map(|x| x.uid.clone())
                .collect::<Vec<String>>()
                .contains(&index)
            {
                self.client.create_index(&index, None).await.unwrap();
            }
        } else {
            self.client.create_index(&index, None).await.unwrap();
        }

        self.index = Some(self.client.index(index));
        self
    }

    /// Support document time to live (TTL).
    pub fn ttl(mut self, row: String) -> Result<Self, Error> {
        if let Some(index) = &self.index {
            self.ttl_row = Some(row);

            if let Some(row) = &self.ttl_row {
                ttl::cron_job(
                    Arc::new(index.clone()),
                    Arc::new(row.to_owned()),
                );
            }
            Ok(self)
        } else {
            Err(Error::new(
                ErrorType::Database(Database::MissingIndex),
                None,
                Some("Index has not been selected.".to_string()),
            ))
        }
    }

    /// Add entry on Meilisearch database.
    pub async fn add_entry<T>(&self, data: T) -> Result<(), Error>
    where
        T: serde::Serialize
            + serde::de::DeserializeOwned
            + Attributes
            + Send
            + Sync,
    {
        if let Some(index) = &self.index {
            let task =
                index.add_or_replace(&[data], None).await.map_err(|err| {
                    Error::new(
                        ErrorType::Unspecified,
                        Some(Box::new(err)),
                        None,
                    )
                })?;

            self.client.wait_for_task(task, None, None).await.map_err(
                |err| {
                    Error::new(
                        ErrorType::Unspecified,
                        Some(Box::new(err)),
                        Some("Waiting for task".to_string()),
                    )
                },
            )?;

            Ok(())
        } else {
            Err(Error::new(
                ErrorType::Database(Database::MissingIndex),
                None,
                Some("Index has not been selected.".to_string()),
            ))
        }
    }
}
