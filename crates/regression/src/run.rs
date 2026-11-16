//! Test run orchestration

use std::fs;
use std::path::{Path, PathBuf};

use doco::Client;

use crate::compare::ImageComparator;
use crate::error::Error;
use crate::result::{Comparison, RunResult, VariantOutcome};
use crate::story::{self, StoryVariant};

/// Configuration for a regression test run
///
/// # Examples
///
/// ```
/// use holt_regression::Config;
///
/// let config = Config::builder()
///     .baseline_dir("tests/visual-baselines")
///     .build();
///
/// assert!(config.baseline_dir().ends_with("visual-baselines"));
/// ```
#[derive(bon::Builder, getset::Getters)]
pub struct Config {
    /// Directory containing baseline images (e.g. `tests/visual-baselines`)
    #[builder(into)]
    #[getset(get = "pub")]
    baseline_dir: PathBuf,

    /// Image comparator to use (defaults to exact byte comparison)
    #[builder(default)]
    #[getset(get = "pub")]
    comparator: Box<dyn ImageComparator>,
}

/// Runs visual regression tests against discovered variants
///
/// Captures a screenshot of each variant and compares it against the baseline in `config.baseline_dir`.
/// Returns structured results and does not write files or print output.
///
/// # Examples
///
/// ```no_run
/// use doco::Client;
/// use holt_regression::{Config, StoryVariant, run};
///
/// async fn check(client: &Client, variants: &[StoryVariant]) {
///     let config = Config::builder()
///         .baseline_dir("tests/visual-baselines")
///         .build();
///
///     let result = run(client, variants, &config).await;
///
///     assert_eq!(result.failed(), 0);
/// }
/// ```
pub async fn run(client: &Client, variants: &[StoryVariant], config: &Config) -> RunResult {
    let mut outcomes = Vec::with_capacity(variants.len());

    for variant in variants {
        let result = capture_and_compare(client, config, variant).await;
        outcomes.push(
            VariantOutcome::builder()
                .variant(variant.clone())
                .result(result)
                .build(),
        );
    }

    RunResult::builder().outcomes(outcomes).build()
}

async fn capture_and_compare(
    client: &Client,
    config: &Config,
    variant: &StoryVariant,
) -> Result<Comparison, Error> {
    let screenshot = story::capture_screenshot(client, variant).await?;

    compare_screenshot(
        config.baseline_dir(),
        config.comparator().as_ref(),
        variant,
        screenshot,
    )
}

/// Compares a screenshot against the baseline on disk
fn compare_screenshot(
    baseline_dir: &Path,
    comparator: &dyn ImageComparator,
    variant: &StoryVariant,
    screenshot: Vec<u8>,
) -> Result<Comparison, Error> {
    let baseline_path = story::baseline_path(baseline_dir, variant);

    if !baseline_path.exists() {
        return Ok(Comparison::New { screenshot });
    }

    let baseline = fs::read(&baseline_path).map_err(|e| Error::BaselineRead {
        path: baseline_path,
        source: e,
    })?;

    if comparator.images_match(&baseline, &screenshot) {
        Ok(Comparison::Passed)
    } else {
        Ok(Comparison::Mismatch {
            baseline,
            screenshot,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::baseline;
    use crate::compare::ExactComparator;

    #[bon::builder]
    fn variant(story_id: &str, name: &str) -> StoryVariant {
        StoryVariant::builder()
            .story_id(story_id)
            .variant_index(0)
            .name(name)
            .build()
    }

    fn mismatched_screenshots() -> (Vec<u8>, Vec<u8>) {
        let dir = tempfile::tempdir().unwrap();
        let v = variant().story_id("button").name("default").call();
        baseline::save(dir.path(), &v, b"old").unwrap();

        let result = compare_screenshot(dir.path(), &ExactComparator, &v, b"new".to_vec()).unwrap();

        match result {
            Comparison::Mismatch {
                baseline,
                screenshot,
            } => (baseline, screenshot),
            other => panic!("expected Mismatch, got {}", comparison_name(&other)),
        }
    }

    #[test]
    fn compare_returns_mismatch_when_different_keeps_baseline() {
        let (baseline, _) = mismatched_screenshots();

        assert_eq!(baseline, b"old");
    }

    #[test]
    fn compare_returns_mismatch_when_different_keeps_screenshot() {
        let (_, screenshot) = mismatched_screenshots();

        assert_eq!(screenshot, b"new");
    }

    #[test]
    fn compare_returns_new_when_no_baseline() {
        let dir = tempfile::tempdir().unwrap();
        let v = variant().story_id("button").name("default").call();

        let result =
            compare_screenshot(dir.path(), &ExactComparator, &v, b"screenshot".to_vec()).unwrap();

        match result {
            Comparison::New { screenshot } => assert_eq!(screenshot, b"screenshot"),
            other => panic!("expected New, got {}", comparison_name(&other)),
        }
    }

    #[test]
    fn compare_returns_passed_when_identical() {
        let dir = tempfile::tempdir().unwrap();
        let v = variant().story_id("button").name("default").call();
        baseline::save(dir.path(), &v, b"identical").unwrap();

        let result =
            compare_screenshot(dir.path(), &ExactComparator, &v, b"identical".to_vec()).unwrap();

        assert!(result.is_passed(), "expected Passed");
    }

    #[test]
    fn compare_uses_custom_comparator() {
        struct AlwaysMatch;
        impl ImageComparator for AlwaysMatch {
            fn images_match(&self, _: &[u8], _: &[u8]) -> bool {
                true
            }
        }

        let dir = tempfile::tempdir().unwrap();
        let v = variant().story_id("button").name("default").call();
        baseline::save(dir.path(), &v, b"old").unwrap();

        let result =
            compare_screenshot(dir.path(), &AlwaysMatch, &v, b"totally different".to_vec())
                .unwrap();

        assert!(result.is_passed(), "custom comparator should have matched");
    }

    #[test]
    fn config_is_send_sync_and_unpin() {
        fn assert_send_sync_and_unpin<T: Send + Sync + Unpin>() {}

        assert_send_sync_and_unpin::<Config>();
    }

    fn comparison_name(c: &Comparison) -> &'static str {
        match c {
            Comparison::Passed => "Passed",
            Comparison::Mismatch { .. } => "Mismatch",
            Comparison::New { .. } => "New",
        }
    }
}
