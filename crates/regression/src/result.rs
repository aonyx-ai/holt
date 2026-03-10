//! Outcome types for regression test runs.

use crate::error::Error;
use crate::story::StoryVariant;

/// The outcome of comparing a single variant against its baseline.
pub enum Comparison {
    /// Screenshot matches the baseline
    Passed,

    /// Screenshot differs from the baseline
    Mismatch {
        baseline: Vec<u8>,
        screenshot: Vec<u8>,
    },

    /// No baseline exists yet — this is a new variant
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

/// A variant paired with its test outcome.
pub struct VariantOutcome {
    pub variant: StoryVariant,
    pub result: Result<Comparison, Error>,
}

/// Results of a full regression test run.
pub struct RunResult {
    pub outcomes: Vec<VariantOutcome>,
}

impl RunResult {
    /// Count of variants that matched their baselines.
    pub fn passed(&self) -> usize {
        self.outcomes
            .iter()
            .filter(|o| o.result.as_ref().is_ok_and(|c| c.is_passed()))
            .count()
    }

    /// Count of variants that failed (mismatch, new, or error).
    pub fn failed(&self) -> usize {
        self.outcomes.len() - self.passed()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn variant(story_id: &str, name: &str) -> StoryVariant {
        StoryVariant {
            story_id: story_id.to_string(),
            variant_index: 0,
            name: name.to_string(),
        }
    }

    #[test]
    fn run_result_counts() {
        let result = RunResult {
            outcomes: vec![
                VariantOutcome {
                    variant: variant("a", "x"),
                    result: Ok(Comparison::Passed),
                },
                VariantOutcome {
                    variant: variant("b", "y"),
                    result: Ok(Comparison::New { screenshot: vec![] }),
                },
                VariantOutcome {
                    variant: variant("c", "z"),
                    result: Ok(Comparison::Mismatch {
                        baseline: vec![],
                        screenshot: vec![],
                    }),
                },
                VariantOutcome {
                    variant: variant("d", "w"),
                    result: Err(Error::Capture("capture failed".into())),
                },
            ],
        };
        assert_eq!(result.passed(), 1);
        assert_eq!(result.failed(), 3);
    }

    #[test]
    fn is_passed_returns_true_for_passed() {
        assert!(Comparison::Passed.is_passed());
    }

    #[test]
    fn is_passed_returns_false_for_others() {
        assert!(!Comparison::New { screenshot: vec![] }.is_passed());
        assert!(
            !Comparison::Mismatch {
                baseline: vec![],
                screenshot: vec![]
            }
            .is_passed()
        );
    }
}
