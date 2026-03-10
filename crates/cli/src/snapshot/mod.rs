//! Visual regression testing for Holt storybook components.
//!
//! Captures screenshots of component stories and compares them against baseline images.

mod compare;

use std::path::Path;
use std::process::Command;

use compare::prompt_user_approval;
use doco::Mount;
use holt_regression::{Comparison, StoryVariant, VariantOutcome};

const BASELINE_DIR: &str = "tests/visual-baselines";

/// Configuration for running visual regression tests.
pub struct SnapshotConfig<'a> {
    pub book_path: &'a Path,
    pub stories_path: &'a Path,
}

/// Run `trunk build` in the book path to produce `dist/`.
fn trunk_build(book_path: &Path) -> Result<std::path::PathBuf, String> {
    println!("Running trunk build...");

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

    println!("Build complete: {}\n", dist_path.display());
    Ok(dist_path)
}

/// Run visual regression tests.
pub async fn run(config: SnapshotConfig<'_>) -> Result<(), String> {
    let baseline_dir = config.book_path.join(BASELINE_DIR);
    let is_ci = std::env::var("CI").is_ok();

    println!("Holt Visual Regression Testing");
    println!("================================\n");

    let dist_path = trunk_build(config.book_path)?;

    let dist_str = dist_path
        .to_str()
        .ok_or_else(|| "dist path is not valid UTF-8".to_string())?;

    let doco = doco::Doco::builder()
        .server(
            doco::Server::builder()
                .image("caddy")
                .tag("alpine")
                .port(80)
                .mount(Mount::bind_mount(dist_str, "/srv"))
                .cmd_arg("caddy")
                .cmd_arg("file-server")
                .cmd_arg("--root")
                .cmd_arg("/srv")
                .cmd_arg("--listen")
                .cmd_arg(":80")
                .cmd_arg("--try-files")
                .cmd_arg("{path}")
                .cmd_arg("/index.html")
                .build(),
        )
        .headless(is_ci)
        .viewport(doco::Viewport::new(1280, 720))
        .build();

    println!("Starting doco session...");
    let session = doco.connect().await.map_err(|e| e.to_string())?;

    let variants = holt_regression::discover_variants(config.stories_path)
        .map_err(|e| format!("Failed to discover stories: {}", e))?;
    println!("Found {} story variants\n", variants.len());

    let regression_config = holt_regression::Config {
        baseline_dir: baseline_dir.clone(),
        comparator: Default::default(),
    };

    let result = holt_regression::run(session.client(), &variants, &regression_config).await;

    let mut passed = 0;
    let mut failed = 0;

    for outcome in &result.outcomes {
        match handle_outcome(outcome, &baseline_dir, is_ci) {
            Ok(true) => passed += 1,
            Ok(false) => failed += 1,
            Err(e) => {
                println!(
                    "  [ERROR] {}/{}: {}",
                    outcome.variant.story_id, outcome.variant.name, e
                );
                failed += 1;
            }
        }
    }

    session.close().await.map_err(|e| e.to_string())?;

    println!("\n================================");
    println!("Results: {} passed, {} failed", passed, failed);

    if !is_ci {
        let orphaned = holt_regression::cleanup_orphaned(&baseline_dir, &variants)
            .map_err(|e| e.to_string())?;
        if !orphaned.is_empty() {
            println!("\nCleaned up {} orphaned baseline(s):", orphaned.len());
            for path in &orphaned {
                println!("  Removed: {}", path.display());
            }
        }
    }

    if failed > 0 {
        return Err(format!("{} visual regression test(s) failed", failed));
    }

    Ok(())
}

/// Handle a single variant outcome — print status, prompt for approval if needed.
/// Returns Ok(true) for passed, Ok(false) for failed.
fn handle_outcome(
    outcome: &VariantOutcome,
    baseline_dir: &Path,
    is_ci: bool,
) -> Result<bool, String> {
    let variant = &outcome.variant;
    let label = format!("{}/{}", variant.story_id, variant.name);

    match &outcome.result {
        Err(e) => {
            println!("  [ERROR] {} {}", label, e);
            Ok(false)
        }
        Ok(Comparison::Passed) => {
            println!("  [ok] {} matches baseline", label);
            Ok(true)
        }
        Ok(Comparison::New { screenshot }) => {
            println!("  [new] {} (new baseline)", label);
            holt_regression::save(baseline_dir, variant, screenshot).map_err(|e| e.to_string())?;
            println!("  -> Baseline created (test will fail until committed)");
            Ok(false)
        }
        Ok(Comparison::Mismatch {
            baseline,
            screenshot,
        }) => {
            println!("  [FAIL] {} differs from baseline", label);
            handle_mismatch(variant, baseline, screenshot, baseline_dir, is_ci)
        }
    }
}

fn handle_mismatch(
    variant: &StoryVariant,
    baseline: &[u8],
    screenshot: &[u8],
    baseline_dir: &Path,
    is_ci: bool,
) -> Result<bool, String> {
    if is_ci {
        holt_regression::save(baseline_dir, variant, screenshot).map_err(|e| e.to_string())?;
        println!("  -> New screenshot saved for artifact upload");
        return Ok(false);
    }

    let baseline_path = baseline_dir
        .join(&variant.story_id)
        .join(format!("{}.png", variant.name));

    let approved = prompt_user_approval(variant, baseline, screenshot, &baseline_path)
        .map_err(|e| e.to_string())?;

    if approved {
        holt_regression::save(baseline_dir, variant, screenshot).map_err(|e| e.to_string())?;
        println!("  -> Baseline updated");
        Ok(true)
    } else {
        println!("  -> Baseline not updated");
        Ok(false)
    }
}
