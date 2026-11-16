//! Visual regression testing library for Holt storybook components
//!
//! Captures screenshots of story variants and compares them against baseline images on disk.
//! Returns structured results without printing or prompting. The CLI layer handles presentation.
//!
//! Story variants are discovered by scanning Rust source files for `#[story]` and `#[variant]`
//! macro annotations via [`discover_variants`].

mod baseline;
mod compare;
mod discover;
mod error;
mod result;
mod run;
mod story;

pub use self::baseline::{cleanup_orphaned, save};
pub use self::compare::ImageComparator;
pub use self::discover::discover_variants;
pub use self::error::{Error, Result};
pub use self::result::{Comparison, RunResult, VariantOutcome};
pub use self::run::{Config, run};
pub use self::story::StoryVariant;
