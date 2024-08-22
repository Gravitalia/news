#![forbid(unsafe_code)]
#![deny(
    dead_code,
    unused_imports,
    unused_mut,
    missing_docs,
    missing_debug_implementations
)]
//! Internal library to provide structures for errors in Gravitalia News.
//!
//! # Examples
//!
//! Basic usage of [`Error`]:
//! ```rust
//! use error::{Error, ErrorType};
//!
//! let error = Error::new(
//!     ErrorType::Unspecified,
//!     None,
//!     Some("An unspecified error occurred.".to_string()),
//! );
//! eprintln!("{}", error);
//! ```

use std::error::Error as StdError;
use std::fmt;

/// Boxed error to bypass specific [Error](StdError).
pub type BError = Box<dyn StdError + Send + Sync>;

/// Represents an error in Polymath.
#[derive(Debug)]
pub struct Error {
    /// The type of the error.
    pub error_type: ErrorType,
    /// The cause of this error.
    pub cause: Option<BError>,
    /// Contextual information about where the error occurred.
    pub context: Option<String>,
}

impl Error {
    /// Creates a new [`Error`].
    pub fn new(
        error_type: ErrorType,
        cause: Option<BError>,
        context: Option<String>,
    ) -> Self {
        Self {
            error_type,
            cause,
            context,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.error_type)
    }
}

impl StdError for Error {}

/// Defines the types of errors in Polymath.
#[derive(Debug)]
pub enum ErrorType {
    /// A generic error with no additional information.
    Unspecified,
    /// Errors related to databases (Meilisearch).
    Database(Database),
}

impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorType::Unspecified => {
                write!(f, "An unspecified error occurred.")
            },
            ErrorType::Database(ref error) => {
                write!(f, "{}", error)
            },
        }
    }
}

impl StdError for ErrorType {}

/// Errors related to databases or message brokers.
#[derive(Debug)]
pub enum Database {
    /// Meilisearch index has not been selected.
    MissingIndex,
    /// Failed to get pool.
    Pool,
}

impl fmt::Display for Database {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Database::MissingIndex => {
                write!(f, "Index has not been selected.")
            },
            Database::Pool => {
                write!(f, "Failed to get pool.")
            },
        }
    }
}

impl StdError for Database {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let error = Error::new(
            ErrorType::Unspecified,
            None,
            Some("An unspecified error occurred.".to_string()),
        );
        assert_eq!(error.to_string(), "An unspecified error occurred.");
    }

    #[test]
    fn test_database_error_display() {
        let db_error = Error::new(
            ErrorType::Database(Database::MissingIndex),
            None,
            Some("Index has not been selected.".to_string()),
        );
        assert_eq!(db_error.to_string(), "Index has not been selected.");
    }

    #[test]
    fn test_error_with_cause() {
        let cause: BError = Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Root cause",
        ));
        let error = Error::new(
            ErrorType::Unspecified,
            Some(cause),
            Some("An unspecified error occurred.".to_string()),
        );
        assert_eq!(error.to_string(), "An unspecified error occurred.");
    }
}
