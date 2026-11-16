//! Visual regression testing for Holt storybook components
//!
//! Captures screenshots of component stories and compares them against baseline images.

mod compare;

use std::fmt::Display;
use std::io::Write;
use std::path::Path;
use std::process::Command;

use self::compare::prompt_user_approval;
use clawless::clap;
use doco::Mount;
use holt_regression::{Comparison, VariantOutcome};

static CADDYFILE: &str = include_str!("Caddyfile");

const BASELINE_DIR: &str = "tests/visual-baselines";

/// Whether to run in check mode (CI) or interactive mode (local dev)
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, clap::ValueEnum)]
pub enum SnapshotMode {
    #[default]
    Interactive,
    Check,
}

/// Whether to run the browser headless
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, clap::ValueEnum)]
pub enum HeadlessMode {
    #[default]
    Auto,
    Headless,
    Visible,
}

/// Whether to save screenshots to the baseline directory
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, clap::ValueEnum)]
pub enum SaveMode {
    #[default]
    Save,
    NoSave,
}

/// Configuration for running visual regression tests
#[derive(bon::Builder, getset::CopyGetters)]
pub struct SnapshotConfig<'a> {
    #[getset(get_copy = "pub")]
    book_path: &'a Path,
    #[getset(get_copy = "pub")]
    stories_path: &'a Path,
    #[getset(get_copy = "pub")]
    headless: HeadlessMode,
    #[getset(get_copy = "pub")]
    save: SaveMode,
    #[getset(get_copy = "pub")]
    mode: SnapshotMode,
}

/// Writes one line to the output handle
fn say(out: &mut impl Write, message: impl Display) -> Result<(), String> {
    writeln!(out, "{message}").map_err(|e| format!("Failed to write output: {e}"))
}

/// Runs `trunk build` in the book path to produce `dist/`
fn trunk_build(book_path: &Path, out: &mut impl Write) -> Result<std::path::PathBuf, String> {
    say(out, "Running trunk build...")?;

    let status = Command::new("trunk")
        .args(["build", "--release"])
        .current_dir(book_path)
        .status()
        .map_err(|e| format!("Failed to run trunk: {}. Is trunk installed?", e))?;

    if !status.success() {
        return Err(format!(
            "trunk build failed with exit code {:?}",
            status.code()
        ));
    }

    let dist_path = book_path.join("dist");
    if !dist_path.exists() {
        return Err(format!(
            "trunk build did not produce {}",
            dist_path.display()
        ));
    }

    let dist_path = dist_path
        .canonicalize()
        .map_err(|e| format!("Failed to resolve dist path: {}", e))?;

    say(
        out,
        format_args!("Build complete: {}\n", dist_path.display()),
    )?;
    Ok(dist_path)
}

/// Runs visual regression tests
///
/// Writes the Caddyfile to a temp path, because the Docker bind mount needs a file on disk.
pub async fn run(
    config: SnapshotConfig<'_>,
    out: &mut impl Write,
    err: &mut impl Write,
) -> Result<(), String> {
    let baseline_dir = config.book_path().join(BASELINE_DIR);

    say(out, "Holt Visual Regression Testing")?;
    say(out, "================================\n")?;

    let dist_path = trunk_build(config.book_path(), out)?;

    let dist_str = dist_path
        .to_str()
        .ok_or_else(|| "dist path is not valid UTF-8".to_string())?;

    let caddyfile_path = std::env::temp_dir().join("holt-Caddyfile");
    std::fs::write(&caddyfile_path, CADDYFILE)
        .map_err(|e| format!("Failed to write Caddyfile: {e}"))?;
    let caddyfile_str = caddyfile_path
        .to_str()
        .ok_or_else(|| "Caddyfile path is not valid UTF-8".to_string())?;

    let doco = doco::Doco::builder()
        .server(
            doco::Server::builder()
                .image("caddy")
                .tag("alpine")
                .port(80)
                .wait(doco::WaitFor::message_on_stderr("server running"))
                .mount(Mount::bind_mount(dist_str, "/srv"))
                .mount(Mount::bind_mount(caddyfile_str, "/etc/caddy/Caddyfile"))
                .build(),
        )
        .headless(config.headless() == HeadlessMode::Headless)
        .viewport(doco::Viewport::new(1280, 720))
        .build();

    say(out, "Starting doco session...")?;
    let session = doco.connect().await.map_err(|e| e.to_string())?;

    let variants = holt_regression::discover_variants(config.stories_path())
        .map_err(|e| format!("Failed to discover stories: {}", e))?;
    say(
        out,
        format_args!("Found {} story variants\n", variants.len()),
    )?;

    let regression_config = holt_regression::Config::builder()
        .baseline_dir(baseline_dir.clone())
        .build();

    let result = holt_regression::run(session.client(), &variants, &regression_config).await;

    let mut passed = 0;
    let mut failed = 0;

    for outcome in result.outcomes() {
        match handle_outcome(outcome, &baseline_dir, &config, out, err) {
            Ok(true) => passed += 1,
            Ok(false) => failed += 1,
            Err(e) => {
                say(
                    out,
                    format_args!(
                        "  [ERROR] {}/{}: {}",
                        outcome.variant().story_id(),
                        outcome.variant().name(),
                        e
                    ),
                )?;
                failed += 1;
            }
        }
    }

    session.close().await.map_err(|e| e.to_string())?;

    say(out, "\n================================")?;
    say(
        out,
        format_args!("Results: {} passed, {} failed", passed, failed),
    )?;

    if config.mode() == SnapshotMode::Interactive && config.headless() != HeadlessMode::Headless {
        let orphaned = holt_regression::cleanup_orphaned(&baseline_dir, &variants)
            .map_err(|e| e.to_string())?;
        if !orphaned.is_empty() {
            say(
                out,
                format_args!("\nCleaned up {} orphaned baseline(s):", orphaned.len()),
            )?;
            for path in &orphaned {
                say(out, format_args!("  Removed: {}", path.display()))?;
            }
        }
    }

    if failed > 0 {
        return Err(format!("{} visual regression test(s) failed", failed));
    }

    Ok(())
}

/// Handles a single variant outcome: prints the status and prompts for approval if needed
///
/// A mismatch prompts for approval in interactive mode.
/// Returns Ok(true) for passed, Ok(false) for failed
fn handle_outcome(
    outcome: &VariantOutcome,
    baseline_dir: &Path,
    config: &SnapshotConfig<'_>,
    out: &mut impl Write,
    err: &mut impl Write,
) -> Result<bool, String> {
    let variant = outcome.variant();
    let label = format!("{}/{}", variant.story_id(), variant.name());

    match outcome.result() {
        Err(e) => {
            say(out, format_args!("  [ERROR] {} {}", label, e))?;
            Ok(false)
        }
        Ok(Comparison::Passed) => {
            say(out, format_args!("  [ok] {} matches baseline", label))?;
            Ok(true)
        }
        Ok(Comparison::New { screenshot }) => {
            match config.save() {
                SaveMode::Save => {
                    say(out, format_args!("  [new] {} (new baseline)", label))?;
                    holt_regression::save(baseline_dir, variant, screenshot)
                        .map_err(|e| e.to_string())?;
                    say(out, "  -> Baseline created")?;
                }
                SaveMode::NoSave => {
                    say(out, format_args!("  [FAIL] {} no baseline", label))?;
                }
            }
            Ok(false)
        }
        Ok(Comparison::Mismatch {
            baseline,
            screenshot,
        }) => {
            say(
                out,
                format_args!("  [FAIL] {} differs from baseline", label),
            )?;

            match config.save() {
                SaveMode::NoSave => return Ok(false),
                SaveMode::Save => {}
            }

            match config.headless() {
                HeadlessMode::Headless => {
                    holt_regression::save(baseline_dir, variant, screenshot)
                        .map_err(|e| e.to_string())?;
                    say(out, "  -> New screenshot saved")?;
                    return Ok(false);
                }
                HeadlessMode::Auto | HeadlessMode::Visible => {}
            }

            let baseline_path = baseline_dir
                .join(variant.story_id())
                .join(format!("{}.png", variant.name()));
            let approved =
                prompt_user_approval(variant, baseline, screenshot, &baseline_path, out, err)
                    .map_err(|e| e.to_string())?;
            if approved {
                holt_regression::save(baseline_dir, variant, screenshot)
                    .map_err(|e| e.to_string())?;
                say(out, "  -> Baseline updated")?;
                Ok(true)
            } else {
                say(out, "  -> Baseline not updated")?;
                Ok(false)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn headless_mode_is_send_sync_and_unpin() {
        fn assert_send_sync_and_unpin<T: Send + Sync + Unpin>() {}

        assert_send_sync_and_unpin::<HeadlessMode>();
    }

    #[test]
    fn save_mode_is_send_sync_and_unpin() {
        fn assert_send_sync_and_unpin<T: Send + Sync + Unpin>() {}

        assert_send_sync_and_unpin::<SaveMode>();
    }

    #[test]
    fn snapshot_config_is_send_sync_and_unpin() {
        fn assert_send_sync_and_unpin<T: Send + Sync + Unpin>() {}

        assert_send_sync_and_unpin::<SnapshotConfig<'static>>();
    }

    #[test]
    fn snapshot_mode_is_send_sync_and_unpin() {
        fn assert_send_sync_and_unpin<T: Send + Sync + Unpin>() {}

        assert_send_sync_and_unpin::<SnapshotMode>();
    }
}
