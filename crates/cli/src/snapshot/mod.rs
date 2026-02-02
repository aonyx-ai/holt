//! Visual regression testing for Holt storybook components.
//!
//! Captures screenshots of component stories and compares them against baseline images.

mod compare;
mod driver;
mod story;

use compare::{images_match, prompt_user_approval};
use driver::{GeckoDriver, TrunkServer};
use story::{StoryVariant, capture_screenshot, discover_stories, get_baseline_path};

use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::Path;
use thirtyfour::prelude::*;

const BASELINE_DIR: &str = "tests/visual-baselines";

/// Configuration for running visual regression tests.
pub struct SnapshotConfig<'a> {
    pub book_path: &'a Path,
    pub port: u16,
}

/// Processes a single story variant.
async fn process_variant(
    driver: &WebDriver,
    server_url: &str,
    baseline_dir: &Path,
    variant: &StoryVariant,
) -> Result<bool, Box<dyn std::error::Error>> {
    let screenshot = capture_screenshot(driver, server_url, variant).await?;
    let baseline_path = get_baseline_path(baseline_dir, variant);
    let is_ci = std::env::var("CI").is_ok();

    if baseline_path.exists() {
        let baseline = fs::read(&baseline_path)?;

        if images_match(&screenshot, &baseline) {
            println!(
                "  [ok] {}/{} matches baseline",
                variant.story_id, variant.name
            );
            Ok(true)
        } else {
            println!(
                "  [FAIL] {}/{} differs from baseline",
                variant.story_id, variant.name
            );

            if is_ci {
                fs::write(&baseline_path, screenshot)?;
                println!("  -> New screenshot saved for artifact upload");
                Ok(false)
            } else if prompt_user_approval(variant, &baseline, &screenshot, &baseline_path)? {
                fs::write(&baseline_path, screenshot)?;
                println!("  -> Baseline updated");
                Ok(true)
            } else {
                println!("  -> Baseline not updated");
                Ok(false)
            }
        }
    } else {
        println!(
            "  [new] {}/{} (new baseline)",
            variant.story_id, variant.name
        );

        if let Some(parent) = baseline_path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&baseline_path, screenshot)?;
        println!("  -> Baseline created (test will fail until committed)");

        Ok(false)
    }
}

/// Removes baseline images that no longer have corresponding stories/variants.
fn cleanup_orphaned_baselines(baseline_dir: &Path, variants: &[StoryVariant]) -> io::Result<()> {
    if !baseline_dir.exists() {
        return Ok(());
    }

    let mut expected_paths = HashSet::new();
    for variant in variants {
        let path = get_baseline_path(baseline_dir, variant);
        expected_paths.insert(path);
    }

    let mut orphaned = Vec::new();
    for entry in fs::read_dir(baseline_dir)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            for file_entry in fs::read_dir(entry.path())? {
                let file_entry = file_entry?;
                if file_entry.file_type()?.is_file() {
                    let file_path = file_entry.path();
                    if !expected_paths.contains(&file_path) {
                        orphaned.push(file_path);
                    }
                }
            }
        }
    }

    if !orphaned.is_empty() {
        println!("\nCleaning up {} orphaned baseline(s):", orphaned.len());
        for path in orphaned {
            println!("  Removing: {}", path.display());
            fs::remove_file(&path)?;

            if let Some(parent) = path.parent()
                && let Ok(mut entries) = fs::read_dir(parent)
                && entries.next().is_none()
            {
                let _ = fs::remove_dir(parent);
            }
        }
    }

    Ok(())
}

/// Run visual regression tests.
pub async fn run(config: SnapshotConfig<'_>) -> Result<(), String> {
    let baseline_dir = config.book_path.join(BASELINE_DIR);

    println!("Holt Visual Regression Testing");
    println!("================================\n");

    let _geckodriver = GeckoDriver::start().map_err(|e| e.to_string())?;
    let server = TrunkServer::start(config.book_path, config.port).map_err(|e| e.to_string())?;

    println!("Connecting to WebDriver...");
    let mut caps = DesiredCapabilities::firefox();

    let is_ci = std::env::var("CI").is_ok();
    if is_ci {
        caps.set_headless().map_err(|e| e.to_string())?;
        println!("Running Firefox in headless mode");
    }

    let driver = WebDriver::new("http://localhost:4444", caps)
        .await
        .map_err(|e| e.to_string())?;
    driver
        .set_window_rect(0, 0, 1280, 720)
        .await
        .map_err(|e| e.to_string())?;

    let variants = discover_stories(&driver, server.url())
        .await
        .map_err(|e| e.to_string())?;

    println!("\nProcessing {} story variants...\n", variants.len());

    let mut passed = 0;
    let mut failed = 0;

    for variant in &variants {
        match process_variant(&driver, server.url(), &baseline_dir, variant).await {
            Ok(true) => passed += 1,
            Ok(false) => failed += 1,
            Err(e) => {
                println!(
                    "  [ERROR] Error processing {}/{}: {}",
                    variant.story_id, variant.name, e
                );
                failed += 1;
            }
        }
    }

    driver.quit().await.map_err(|e| e.to_string())?;

    println!("\n================================");
    println!("Results: {} passed, {} failed", passed, failed);

    if !is_ci {
        cleanup_orphaned_baselines(&baseline_dir, &variants).map_err(|e| e.to_string())?;
    }

    if failed > 0 {
        return Err(format!("{} visual regression test(s) failed", failed));
    }

    Ok(())
}
