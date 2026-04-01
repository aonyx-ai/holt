//! Visual regression testing library for Holt storybook components.
//!
//! Captures screenshots of story variants and compares them against baseline images on disk.
//! Returns structured results without printing or prompting — the CLI layer handles presentation.
//!
//! Story variants are discovered by scanning Rust source files for `#[story]` and `#[variant]`
//! macro annotations via [`discover_variants`].

mod baseline;
mod compare;
mod discover;
mod error;
mod report;
mod result;
mod run;
mod story;

pub use baseline::{cleanup_orphaned, save};
pub use compare::ImageComparator;
pub use discover::discover_variants;
pub use error::{Error, Result};
pub use report::generate_html_report;
pub use result::{Comparison, RunResult, VariantOutcome};
pub use run::{Config, run};
pub use story::StoryVariant;
