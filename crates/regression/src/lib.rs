//! Visual regression testing library for Holt storybook components.
//!
//! Captures screenshots of story variants and compares them against baseline images on disk.
//! Returns structured results without printing or prompting — the CLI layer handles presentation.
//!
//! Story discovery is the caller's responsibility. Construct [`StoryVariant`]s from your story
//! registry (e.g. `inventory::iter`) and pass them to [`run`].

mod baseline;
mod compare;
mod error;
mod result;
mod run;
mod story;

pub use baseline::{cleanup_orphaned, save};
pub use compare::ImageComparator;
pub use error::{Error, Result};
pub use result::{Comparison, RunResult, VariantOutcome};
pub use run::{Config, run};
pub use story::StoryVariant;
