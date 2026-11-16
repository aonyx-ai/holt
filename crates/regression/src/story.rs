//! Story variant types and screenshot capture

use std::path::{Path, PathBuf};
use std::time::Duration;

use doco::Client;

use crate::error::Error;

/// Story variant metadata
///
/// Callers construct these from their own story registry (e.g. `inventory::iter`).
/// The regression crate uses them to capture screenshots and locate baselines.
///
/// # Examples
///
/// ```
/// use holt_regression::StoryVariant;
///
/// let variant = StoryVariant::builder()
///     .story_id("button")
///     .variant_index(0)
///     .name("default")
///     .build();
///
/// assert_eq!(variant.story_id(), "button");
/// ```
#[derive(Clone, Debug, bon::Builder, getset::CopyGetters, getset::Getters)]
pub struct StoryVariant {
    /// The story identifier (e.g. "button", "checkbox")
    #[builder(into)]
    #[getset(get = "pub")]
    story_id: String,

    /// The zero-based index of this variant within the story
    #[getset(get_copy = "pub")]
    variant_index: usize,

    /// A slug-style name for the variant (e.g. "default", "destructive")
    #[builder(into)]
    #[getset(get = "pub")]
    name: String,
}

/// Captures a screenshot of a story variant
pub async fn capture_screenshot(client: &Client, variant: &StoryVariant) -> Result<Vec<u8>, Error> {
    let path = format!(
        "/visual-test/{}/{}",
        variant.story_id(),
        variant.variant_index()
    );
    client.goto(&path).await.map_err(|e| Error::Navigate {
        path,
        source: e.into(),
    })?;
    tokio::time::sleep(Duration::from_millis(500)).await;
    client
        .screenshot_as_png()
        .await
        .map_err(|e| Error::Capture { source: e.into() })
}

/// Returns the baseline file path for a story variant
pub fn baseline_path(baseline_dir: &Path, variant: &StoryVariant) -> PathBuf {
    baseline_dir
        .join(variant.story_id())
        .join(format!("{}.png", variant.name()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn story_variant_is_send_sync_and_unpin() {
        fn assert_send_sync_and_unpin<T: Send + Sync + Unpin>() {}

        assert_send_sync_and_unpin::<StoryVariant>();
    }
}
