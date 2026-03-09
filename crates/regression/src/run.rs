//! Test run orchestration.

use std::fs;
use std::path::{Path, PathBuf};

use doco::Client;

use crate::compare::ImageComparator;
use crate::error::Error;
use crate::result::{Comparison, RunResult, VariantOutcome};
use crate::story::{self, StoryVariant};

/// Configuration for a regression test run.
pub struct Config {
    /// Directory containing baseline images (e.g. `tests/visual-baselines`)
    pub baseline_dir: PathBuf,

    /// Image comparator to use (defaults to exact byte comparison)
    pub comparator: Box<dyn ImageComparator>,
}

/// Run visual regression tests against discovered variants.
///
/// Captures a screenshot of each variant and compares it against the baseline in `config.baseline_dir`.
/// Returns structured results — does not write files or print output.
pub async fn run(client: &Client, variants: &[StoryVariant], config: &Config) -> RunResult {
    let mut outcomes = Vec::with_capacity(variants.len());

    for variant in variants {
        let result = capture_and_compare(client, config, variant).await;
        outcomes.push(VariantOutcome {
            variant: variant.clone(),
            result,
        });
    }

    RunResult { outcomes }
}

async fn capture_and_compare(
    client: &Client,
    config: &Config,
    variant: &StoryVariant,
) -> Result<Comparison, Error> {
    let screenshot = story::capture_screenshot(client, variant)
        .await
        .map_err(|e| Error::Capture(e.into()))?;

    compare_screenshot(
        &config.baseline_dir,
        config.comparator.as_ref(),
        variant,
        screenshot,
    )
}

/// Compare a screenshot against the baseline on disk.
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

    fn variant(story_id: &str, name: &str) -> StoryVariant {
        StoryVariant {
            story_id: story_id.to_string(),
            variant_index: 0,
            name: name.to_string(),
        }
    }

    #[test]
    fn compare_returns_new_when_no_baseline() {
        let dir = tempfile::tempdir().unwrap();
        let v = variant("button", "default");

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
        let v = variant("button", "default");
        baseline::save(dir.path(), &v, b"identical").unwrap();

        let result =
            compare_screenshot(dir.path(), &ExactComparator, &v, b"identical".to_vec()).unwrap();

        assert!(result.is_passed(), "expected Passed");
    }

    #[test]
    fn compare_returns_mismatch_when_different() {
        let dir = tempfile::tempdir().unwrap();
        let v = variant("button", "default");
        baseline::save(dir.path(), &v, b"old").unwrap();

        let result = compare_screenshot(dir.path(), &ExactComparator, &v, b"new".to_vec()).unwrap();

        match result {
            Comparison::Mismatch {
                baseline,
                screenshot,
            } => {
                assert_eq!(baseline, b"old");
                assert_eq!(screenshot, b"new");
            }
            other => panic!("expected Mismatch, got {}", comparison_name(&other)),
        }
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
        let v = variant("button", "default");
        baseline::save(dir.path(), &v, b"old").unwrap();

        let result =
            compare_screenshot(dir.path(), &AlwaysMatch, &v, b"totally different".to_vec())
                .unwrap();

        assert!(result.is_passed(), "custom comparator should have matched");
    }

    fn comparison_name(c: &Comparison) -> &'static str {
        match c {
            Comparison::Passed => "Passed",
            Comparison::Mismatch { .. } => "Mismatch",
            Comparison::New { .. } => "New",
        }
    }
}
