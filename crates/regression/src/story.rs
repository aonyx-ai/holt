//! Story variant types and screenshot capture.

use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::{Context, Result};
use doco::Client;

/// Story variant metadata.
///
/// Callers construct these from their own story registry (e.g. `inventory::iter`).
/// The regression crate uses them to capture screenshots and locate baselines.
#[derive(Clone, Debug)]
pub struct StoryVariant {
    /// The story identifier (e.g. "button", "checkbox")
    pub story_id: String,

    /// The zero-based index of this variant within the story
    pub variant_index: usize,

    /// A slug-style name for the variant (e.g. "default", "destructive")
    pub name: String,
}

/// Capture a screenshot of a story variant.
pub async fn capture_screenshot(client: &Client, variant: &StoryVariant) -> Result<Vec<u8>> {
    let path = format!(
        "/visual-test/{}/{}",
        variant.story_id, variant.variant_index
    );
    client
        .goto(&path)
        .await
        .context("failed to navigate to variant")?;
    tokio::time::sleep(Duration::from_millis(500)).await;
    client
        .screenshot_as_png()
        .await
        .context("failed to capture screenshot")
}

/// Get the baseline file path for a story variant.
pub fn baseline_path(baseline_dir: &Path, variant: &StoryVariant) -> PathBuf {
    baseline_dir
        .join(&variant.story_id)
        .join(format!("{}.png", variant.name))
}
