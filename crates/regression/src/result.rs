//! Outcome types for regression test runs

use crate::error::Error;
use crate::story::StoryVariant;

/// The outcome of comparing a single variant against its baseline
///
/// # Examples
///
/// ```
/// use holt_regression::Comparison;
///
/// assert!(Comparison::Passed.is_passed());
/// assert!(!Comparison::New { screenshot: vec![] }.is_passed());
/// ```
pub enum Comparison {
    /// Screenshot matches the baseline
    Passed,

    /// Screenshot differs from the baseline
    Mismatch {
        baseline: Vec<u8>,
        screenshot: Vec<u8>,
    },

    /// No baseline exists yet: this is a new variant
    New { screenshot: Vec<u8> },
}

impl Comparison {
    pub fn is_passed(&self) -> bool {
        match self {
            Comparison::Passed => true,
            Comparison::Mismatch { .. } => false,
            Comparison::New { .. } => false,
        }
    }
}

/// A variant paired with its test outcome
///
/// # Examples
///
/// ```
/// use holt_regression::{Comparison, StoryVariant, VariantOutcome};
///
/// let variant = StoryVariant::builder()
///     .story_id("button")
///     .variant_index(0)
///     .name("default")
///     .build();
/// let outcome = VariantOutcome::builder()
///     .variant(variant)
///     .result(Ok(Comparison::Passed))
///     .build();
///
/// assert_eq!(outcome.variant().name(), "default");
/// ```
#[derive(bon::Builder, getset::Getters)]
pub struct VariantOutcome {
    #[getset(get = "pub")]
    variant: StoryVariant,

    #[getset(get = "pub")]
    result: Result<Comparison, Error>,
}

/// Results of a full regression test run
///
/// # Examples
///
/// ```
/// use holt_regression::RunResult;
///
/// let run = RunResult::builder().outcomes(vec![]).build();
///
/// assert!(run.outcomes().is_empty());
/// ```
#[derive(bon::Builder, getset::Getters)]
pub struct RunResult {
    #[getset(get = "pub")]
    outcomes: Vec<VariantOutcome>,
}

impl RunResult {
    /// Returns the count of variants that matched their baselines
    ///
    /// # Examples
    ///
    /// ```
    /// use holt_regression::{Comparison, RunResult, StoryVariant, VariantOutcome};
    ///
    /// let variant = StoryVariant::builder()
    ///     .story_id("button")
    ///     .variant_index(0)
    ///     .name("default")
    ///     .build();
    /// let outcome = VariantOutcome::builder()
    ///     .variant(variant)
    ///     .result(Ok(Comparison::Passed))
    ///     .build();
    /// let run = RunResult::builder().outcomes(vec![outcome]).build();
    ///
    /// assert_eq!(run.passed(), 1);
    /// ```
    pub fn passed(&self) -> usize {
        self.outcomes
            .iter()
            .filter(|o| o.result().as_ref().is_ok_and(|c| c.is_passed()))
            .count()
    }

    /// Returns the count of variants that failed (mismatch, new, or error)
    ///
    /// # Examples
    ///
    /// ```
    /// use holt_regression::{Comparison, RunResult, StoryVariant, VariantOutcome};
    ///
    /// let variant = StoryVariant::builder()
    ///     .story_id("button")
    ///     .variant_index(0)
    ///     .name("default")
    ///     .build();
    /// let outcome = VariantOutcome::builder()
    ///     .variant(variant)
    ///     .result(Ok(Comparison::New { screenshot: vec![] }))
    ///     .build();
    /// let run = RunResult::builder().outcomes(vec![outcome]).build();
    ///
    /// assert_eq!(run.failed(), 1);
    /// ```
    pub fn failed(&self) -> usize {
        self.outcomes.len() - self.passed()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[bon::builder]
    fn variant(story_id: &str, name: &str) -> StoryVariant {
        StoryVariant::builder()
            .story_id(story_id)
            .variant_index(0)
            .name(name)
            .build()
    }

    fn mixed_run_result() -> RunResult {
        RunResult::builder()
            .outcomes(vec![
                VariantOutcome::builder()
                    .variant(variant().story_id("a").name("x").call())
                    .result(Ok(Comparison::Passed))
                    .build(),
                VariantOutcome::builder()
                    .variant(variant().story_id("b").name("y").call())
                    .result(Ok(Comparison::New { screenshot: vec![] }))
                    .build(),
                VariantOutcome::builder()
                    .variant(variant().story_id("c").name("z").call())
                    .result(Ok(Comparison::Mismatch {
                        baseline: vec![],
                        screenshot: vec![],
                    }))
                    .build(),
                VariantOutcome::builder()
                    .variant(variant().story_id("d").name("w").call())
                    .result(Err(Error::Capture {
                        source: "capture failed".into(),
                    }))
                    .build(),
            ])
            .build()
    }

    #[test]
    fn comparison_is_send_sync_and_unpin() {
        fn assert_send_sync_and_unpin<T: Send + Sync + Unpin>() {}

        assert_send_sync_and_unpin::<Comparison>();
    }

    #[test]
    fn is_passed_returns_false_for_mismatch() {
        let comparison = Comparison::Mismatch {
            baseline: vec![],
            screenshot: vec![],
        };

        let passed = comparison.is_passed();

        assert!(!passed);
    }

    #[test]
    fn is_passed_returns_false_for_new() {
        let comparison = Comparison::New { screenshot: vec![] };

        let passed = comparison.is_passed();

        assert!(!passed);
    }

    #[test]
    fn is_passed_returns_true_for_passed() {
        let comparison = Comparison::Passed;

        let passed = comparison.is_passed();

        assert!(passed);
    }

    #[test]
    fn run_result_counts_failed() {
        let result = mixed_run_result();

        let failed = result.failed();

        assert_eq!(failed, 3);
    }

    #[test]
    fn run_result_counts_passed() {
        let result = mixed_run_result();

        let passed = result.passed();

        assert_eq!(passed, 1);
    }

    #[test]
    fn run_result_is_send_sync_and_unpin() {
        fn assert_send_sync_and_unpin<T: Send + Sync + Unpin>() {}

        assert_send_sync_and_unpin::<RunResult>();
    }

    #[test]
    fn variant_outcome_is_send_sync_and_unpin() {
        fn assert_send_sync_and_unpin<T: Send + Sync + Unpin>() {}

        assert_send_sync_and_unpin::<VariantOutcome>();
    }
}
