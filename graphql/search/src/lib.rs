#![forbid(unsafe_code)]
#![deny(dead_code, unused_imports, unused_mut, missing_docs)]
//! Searcher runtime.

use error::{Database, Error, ErrorType};
use meilisearch_sdk::{client::Client, errors::Error as MeiliError};
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
    index: String,
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
            index: String::default(),
            ttl_row: None,
        })
    }

    /// Update Meilisearch index.
    /// Warning: DO NOT UPDATE IT.
    pub fn index(&mut self, index: String) -> &Self {
        self.index = index;
        self
    }

    /// Support document time to live (TTL).
    pub fn ttl(&mut self, row: String) -> &Self {
        self.ttl_row = Some(row);

        if let Some(row) = &self.ttl_row {
            ttl::cron_job(
                Arc::new(self.client.index(&self.index)),
                Arc::new(row.to_owned()),
            );
        }

        self
    }

    /// Add entry on Meilisearch database.
    pub async fn add_entry<T>(&self, data: &T) -> Result<(), Error>
    where
        T: serde::Serialize
            + serde::de::DeserializeOwned
            + Attributes
            + Send
            + Sync,
    {
        if self.index.is_empty() {
            return Err(Error::new(
                ErrorType::Database(Database::MissingIndex),
                None,
                Some("Index has not been selected.".to_string()),
            ));
        }

        self.client
            .index(&self.index)
            .add_documents(&[data], data.primary_key())
            .await
            .unwrap();

        Ok(())
    }
}
