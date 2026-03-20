//! HTML report generation for visual regression results.

use std::fmt::Write;

use base64::{Engine, engine::general_purpose::STANDARD as BASE64};

use crate::result::{Comparison, RunResult, VariantOutcome};

static TEMPLATE: &str = include_str!("report_template.html");

/// Generate a self-contained HTML report from regression test results.
///
/// The report embeds all images as base64 data URIs, so it can be opened
/// directly in a browser with no external dependencies.
pub fn generate_html_report(result: &RunResult) -> String {
    let mut passed = 0usize;
    let mut mismatch = 0usize;
    let mut new = 0usize;
    let mut errored = 0usize;
    let mut entries = String::new();

    for outcome in &result.outcomes {
        match render_entry(outcome) {
            EntryKind::Passed => passed += 1,
            EntryKind::Rendered { kind, html } => {
                match kind {
                    FailKind::Mismatch => mismatch += 1,
                    FailKind::New => new += 1,
                    FailKind::Error => errored += 1,
                }
                entries.push_str(&html);
            }
        }
    }

    TEMPLATE
        .replace("{{PASSED_COUNT}}", &passed.to_string())
        .replace("{{MISMATCH_COUNT}}", &mismatch.to_string())
        .replace("{{NEW_COUNT}}", &new.to_string())
        .replace("{{ERROR_COUNT}}", &errored.to_string())
        .replace("{{ENTRIES}}", &entries)
}

enum FailKind {
    Mismatch,
    New,
    Error,
}

enum EntryKind {
    Passed,
    Rendered { kind: FailKind, html: String },
}

fn render_entry(outcome: &VariantOutcome) -> EntryKind {
    let label = format!("{}/{}", outcome.variant.story_id, outcome.variant.name);

    match &outcome.result {
        Ok(Comparison::Passed) => EntryKind::Passed,
        Ok(Comparison::Mismatch {
            baseline,
            screenshot,
        }) => {
            let baseline_b64 = BASE64.encode(baseline);
            let screenshot_b64 = BASE64.encode(screenshot);
            let baseline_uri = format!("data:image/png;base64,{baseline_b64}");
            let screenshot_uri = format!("data:image/png;base64,{screenshot_b64}");

            let html = format!(
                r#"<div class="entry">
  <div class="entry-header"><span class="arrow">&#9654;</span><span class="label">{label}</span><span class="badge badge-mismatch">changed</span></div>
  <div class="entry-body">
    <div class="mode-tabs">
      <button class="mode-tab active" data-mode="slider">Slider</button>
      <button class="mode-tab" data-mode="side-by-side">Side by Side</button>
      <button class="mode-tab" data-mode="toggle">Toggle</button>
    </div>
    <div class="comparison-mode" data-mode="slider">
      <div class="comparison slider-wrap">
        <img src="{baseline_uri}" alt="baseline">
        <div class="slider-new"><img src="{screenshot_uri}" alt="new"></div>
        <input type="range" min="0" max="100" value="50" class="slider-input">
        <div class="slider-line"></div>
      </div>
    </div>
    <div class="comparison-mode" data-mode="side-by-side" style="display:none">
      <div class="side-by-side">
        <div class="panel"><div class="panel-label">Baseline</div><div class="comparison"><img src="{baseline_uri}" alt="baseline"></div></div>
        <div class="panel"><div class="panel-label">New</div><div class="comparison"><img src="{screenshot_uri}" alt="new"></div></div>
      </div>
    </div>
    <div class="comparison-mode" data-mode="toggle" style="display:none">
      <div class="comparison toggle-wrap">
        <img class="toggle-baseline" src="{baseline_uri}" alt="baseline">
        <img class="toggle-screenshot" src="{screenshot_uri}" alt="new">
      </div>
      <div class="toggle-hint">Click to toggle between baseline and new</div>
    </div>
  </div>
</div>
"#
            );
            EntryKind::Rendered {
                kind: FailKind::Mismatch,
                html,
            }
        }
        Ok(Comparison::New { screenshot }) => {
            let screenshot_b64 = BASE64.encode(screenshot);
            let screenshot_uri = format!("data:image/png;base64,{screenshot_b64}");

            let html = format!(
                r#"<div class="entry">
  <div class="entry-header"><span class="arrow">&#9654;</span><span class="label">{label}</span><span class="badge badge-new">new</span></div>
  <div class="entry-body">
    <div class="new-screenshot comparison"><img src="{screenshot_uri}" alt="new screenshot"></div>
  </div>
</div>
"#
            );
            EntryKind::Rendered {
                kind: FailKind::New,
                html,
            }
        }
        Err(e) => {
            let msg = html_escape(&error_chain(e));
            let html = format!(
                r#"<div class="entry">
  <div class="entry-header"><span class="arrow">&#9654;</span><span class="label">{label}</span><span class="badge badge-error">error</span></div>
  <div class="entry-body">
    <div class="error-message">{msg}</div>
  </div>
</div>
"#
            );
            EntryKind::Rendered {
                kind: FailKind::Error,
                html,
            }
        }
    }
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// Format an error and its full source chain.
fn error_chain(err: &crate::error::Error) -> String {
    use std::error::Error;
    let mut msg = err.to_string();
    let mut source = err.source();
    while let Some(cause) = source {
        let _ = write!(msg, ": {cause}");
        source = cause.source();
    }
    msg
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Error;
    use crate::story::StoryVariant;

    fn variant(story_id: &str, name: &str) -> StoryVariant {
        StoryVariant {
            story_id: story_id.to_string(),
            variant_index: 0,
            name: name.to_string(),
        }
    }

    #[test]
    fn report_contains_summary_counts() {
        let result = RunResult {
            outcomes: vec![
                VariantOutcome {
                    variant: variant("button", "default"),
                    result: Ok(Comparison::Passed),
                },
                VariantOutcome {
                    variant: variant("button", "destructive"),
                    result: Ok(Comparison::Mismatch {
                        baseline: vec![0x89, 0x50, 0x4E, 0x47],
                        screenshot: vec![0x89, 0x50, 0x4E, 0x47, 0xFF],
                    }),
                },
                VariantOutcome {
                    variant: variant("card", "default"),
                    result: Ok(Comparison::New {
                        screenshot: vec![0x89, 0x50],
                    }),
                },
                VariantOutcome {
                    variant: variant("input", "default"),
                    result: Err(Error::Capture("timeout".into())),
                },
            ],
        };

        let html = generate_html_report(&result);

        assert!(html.contains("1 passed"));
        assert!(html.contains("1 changed"));
        assert!(html.contains("1 new"));
        assert!(html.contains("1 errors"));
    }

    #[test]
    fn report_embeds_base64_images() {
        let result = RunResult {
            outcomes: vec![VariantOutcome {
                variant: variant("button", "primary"),
                result: Ok(Comparison::Mismatch {
                    baseline: vec![1, 2, 3],
                    screenshot: vec![4, 5, 6],
                }),
            }],
        };

        let html = generate_html_report(&result);

        assert!(html.contains("data:image/png;base64,"));
        assert!(html.contains("button/primary"));
        assert!(html.contains("slider"));
        assert!(html.contains("side-by-side"));
        assert!(html.contains("toggle"));
    }

    #[test]
    fn report_shows_new_variant() {
        let result = RunResult {
            outcomes: vec![VariantOutcome {
                variant: variant("dialog", "open"),
                result: Ok(Comparison::New {
                    screenshot: vec![10, 20],
                }),
            }],
        };

        let html = generate_html_report(&result);

        assert!(html.contains("dialog/open"));
        assert!(html.contains("badge-new"));
        assert!(html.contains("new-screenshot"));
    }

    #[test]
    fn report_shows_error() {
        let result = RunResult {
            outcomes: vec![VariantOutcome {
                variant: variant("tooltip", "hover"),
                result: Err(Error::Capture("browser crashed".into())),
            }],
        };

        let html = generate_html_report(&result);

        assert!(html.contains("tooltip/hover"));
        assert!(html.contains("badge-error"));
        // Error::Capture displays as "failed to capture screenshot: browser crashed"
        assert!(html.contains("failed to capture screenshot"));
        assert!(html.contains("browser crashed"));
    }

    #[test]
    fn report_escapes_html_in_errors() {
        let result = RunResult {
            outcomes: vec![VariantOutcome {
                variant: variant("xss", "test"),
                result: Err(Error::Capture("<script>alert('xss')</script>".into())),
            }],
        };

        let html = generate_html_report(&result);

        assert!(!html.contains("<script>alert"));
        assert!(html.contains("&lt;script&gt;alert"));
    }

    #[test]
    fn all_passed_produces_no_entries() {
        let result = RunResult {
            outcomes: vec![
                VariantOutcome {
                    variant: variant("a", "x"),
                    result: Ok(Comparison::Passed),
                },
                VariantOutcome {
                    variant: variant("b", "y"),
                    result: Ok(Comparison::Passed),
                },
            ],
        };

        let html = generate_html_report(&result);

        assert!(html.contains("2 passed"));
        assert!(html.contains("0 changed"));
        // No comparison entries should be rendered
        assert!(!html.contains(r#"class="entry">"#));
    }
}
