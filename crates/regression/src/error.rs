//! Error types for the regression crate.

use std::path::PathBuf;

/// Errors that can occur during visual regression testing.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Failed to capture a screenshot via the browser.
    #[error("failed to capture screenshot")]
    Capture(#[source] Box<dyn std::error::Error + Send + Sync>),

    /// Failed to read a baseline image from disk.
    #[error("failed to read baseline at {}", path.display())]
    BaselineRead {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    /// Failed to write a baseline image to disk.
    #[error("failed to write baseline at {}", path.display())]
    BaselineWrite {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    /// Filesystem error during baseline cleanup.
    #[error("failed to clean up baselines")]
    Cleanup(#[source] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
