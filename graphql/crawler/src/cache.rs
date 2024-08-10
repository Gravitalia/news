//! Have I already crawled this page?

use error::{Database, Error, ErrorType};
use polymath_cache::lru::LRUCache;
use r2d2::Pool;
use r2d2_memcache::MemcacheConnectionManager;
use std::fmt;
use url::Url;

/// Cache manager.
/// Implement memcached cache and LRU cache.
pub struct Cache {
    memcached: Option<Pool<MemcacheConnectionManager>>,
    lru: LRUCache<String, bool>,
}

impl fmt::Debug for Cache {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point")
            .field("memcached", &self.memcached.is_some())
            .field("lru", &self.lru)
            .finish()
    }
}

impl Cache {
    /// Create a new [`Cache`] manager.
    /// LRU is used by default.
    pub fn new(capacity: usize) -> Self {
        Cache {
            memcached: None,
            lru: LRUCache::with_capacity(capacity),
        }
    }

    /// Set a [`r2d2::Pool`] of [`memcache::ConnectionManager`].
    pub fn memcached(mut self, pool: Pool<MemcacheConnectionManager>) -> Self {
        self.memcached = Some(pool);
        self
    }

    fn transform_url(&self, url: Url) -> String {
        format!(
            "{}/{}",
            url.host_str().unwrap_or(""),
            url.path().trim_start_matches('/')
        )
    }

    /// Set a [`url::Host`] as crawled.
    pub fn set(&mut self, url: Url) -> Result<(), Error> {
        let url = self.transform_url(url);

        match &self.memcached {
            Some(pool) => {
                pool.get()
                    .map_err(|err| {
                        Error::new(
                            ErrorType::Database(Database::Pool),
                            Some(Box::new(err)),
                            None,
                        )
                    })?
                    .set(&url, true, 86400)
                    .map_err(|err| {
                        Error::new(
                            ErrorType::Unspecified,
                            Some(Box::new(err)),
                            None,
                        )
                    })?;
            },
            None => {
                self.lru.put(url, true);
            },
        }

        Ok(())
    }

    /// Checks if the URL exists in the cache.
    pub fn get(&mut self, url: Url) -> Result<bool, Error> {
        let url = self.transform_url(url);

        match &self.memcached {
            Some(pool) => Ok(pool
                .get()
                .map_err(|err| {
                    Error::new(
                        ErrorType::Database(Database::Pool),
                        Some(Box::new(err)),
                        None,
                    )
                })?
                .get::<bool>(&url)
                .map_err(|err| {
                    Error::new(
                        ErrorType::Unspecified,
                        Some(Box::new(err)),
                        None,
                    )
                })?
                .is_some()),
            None => Ok(self.lru.get(&url).is_some()),
        }
    }
}
