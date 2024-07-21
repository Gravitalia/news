//! time-to-live (ttl) manager for meillisearch.
//!
//! > You **must** save date with `yyyymmdd` format.

use chrono::Local;
use meilisearch_sdk::{documents::DocumentDeletionQuery, indexes::Index};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::task;
use tracing::{error, info};

#[derive(Deserialize, Serialize)]
struct MyDocumentType {}

/// Create a CRON task activated every night at midnight.
pub fn cron_job(index: Arc<Index>, row_name: Arc<String>) {
    task::spawn(async move {
        loop {
            let now = SystemTime::now();

            std::thread::sleep(Duration::from_secs(
                86400
                    - (now
                        .duration_since(UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs()
                        % 86400),
            ));

            info!("cron job started to delete expired news article.");

            if DocumentDeletionQuery::new(&index)
                .with_filter(&format!(
                    "{}={}",
                    row_name,
                    Local::now().format("%Y%m%d")
                ))
                .execute::<MyDocumentType>()
                .await
                .is_err()
            {
                error!("Failed to delete expired news article.")
            };
        }
    });
}
