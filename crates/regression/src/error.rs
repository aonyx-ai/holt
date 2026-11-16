//! Error types for the regression crate

use std::path::PathBuf;

/// Errors that can occur during visual regression testing
///
/// # Examples
///
/// ```
/// use holt_regression::Error;
///
/// let error = Error::Cleanup {
///     source: std::io::Error::other("disk full"),
/// };
///
/// assert_eq!(error.to_string(), "failed to clean up baselines");
/// ```
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Failed to navigate the browser to a story variant
    #[error("failed to navigate to {path}")]
    Navigate {
        path: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    /// Failed to capture a screenshot via the browser
    #[error("failed to capture screenshot")]
    Capture {
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    /// Failed to read a baseline image from disk
    #[error("failed to read baseline at {}", path.display())]
    BaselineRead {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    /// Failed to write a baseline image to disk
    #[error("failed to write baseline at {}", path.display())]
    BaselineWrite {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    /// Failed to scan a path for story variants
    #[error("failed to scan {} for story variants", path.display())]
    Discover {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    /// Filesystem error during baseline cleanup
    #[error("failed to clean up baselines")]
    Cleanup {
        #[source]
        source: std::io::Error,
    },
}

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_is_send_sync_and_unpin() {
        fn assert_send_sync_and_unpin<T: Send + Sync + Unpin>() {}

        assert_send_sync_and_unpin::<Error>();
    }
}
