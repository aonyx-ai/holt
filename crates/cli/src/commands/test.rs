//! `holt test` - Run visual regression tests

use clawless::prelude::*;
use libtest_mimic::{Arguments, Trial};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::runtime::Handle;

use crate::comparison::{ProcessResult, cleanup_orphaned_baselines, process_variant};
use crate::config::HoltConfig;
use crate::discovery::{StoryVariant, discover_stories};
use crate::geckodriver::GeckoDriver;
use crate::storybook::Storybook;
use crate::webdriver::setup_webdriver;

#[derive(Debug, Args)]
pub struct TestArgs {
    /// Baseline directory (overrides holt.toml)
    #[arg(short, long)]
    baseline_dir: Option<PathBuf>,

    /// Arguments passed to the test runner (e.g., -- --list, -- button)
    #[arg(last = true)]
    test_args: Vec<String>,
}

/// Run visual regression tests
#[command]
pub async fn test(args: TestArgs, _ctx: Context) -> CommandResult {
    println!("Holt Visual Regression Testing");
    println!("================================\n");

    let (config, root) = HoltConfig::find_and_load()?;
    let crate_path = config.storybook_path(&root);
    let baseline_dir = args
        .baseline_dir
        .unwrap_or_else(|| config.baseline_path(&root));

    // Parse libtest-mimic args (supports --list, filtering, etc.)
    let test_args =
        Arguments::from_iter(std::iter::once("holt-test".to_string()).chain(args.test_args));

    // Start services
    let _geckodriver = GeckoDriver::start()?;
    let storybook = Storybook::start(&crate_path, config.storybook.port)?;
    let url = storybook.url();

    // Set up WebDriver
    let driver = Arc::new(setup_webdriver().await?);

    // Discover all story variants
    let variants = discover_stories(&driver, &url).await?;

    println!("\nProcessing {} story variants...\n", variants.len());

    // Get the current runtime handle to use in test closures
    let handle = Handle::current();

    // Create test cases for libtest-mimic
    let tests: Vec<Trial> = variants
        .iter()
        .map(|variant| {
            let url = url.clone();
            let baseline_dir = baseline_dir.clone();
            let variant = variant.clone();
            let driver = Arc::clone(&driver);
            let handle = handle.clone();

            Trial::test(
                format!("{}/{}", variant.story_id, variant.name),
                move || {
                    // Use the existing runtime via handle.block_on
                    handle.block_on(async {
                        match process_variant(&driver, &url, &baseline_dir, &variant).await {
                            ProcessResult::Passed => Ok(()),
                            ProcessResult::Failed => {
                                Err(format!("{}/{} failed", variant.story_id, variant.name).into())
                            }
                            ProcessResult::Error(e) => Err(e.into()),
                        }
                    })
                },
            )
        })
        .collect();

    // Store variants for cleanup later
    let variants_for_cleanup: Vec<StoryVariant> = variants;

    // Run tests with libtest-mimic
    // This provides: filtering, --list, parallel execution, cargo test-like output
    let conclusion = libtest_mimic::run(&test_args, tests);

    // Clean up WebDriver
    if let Err(e) = Arc::try_unwrap(driver)
        .map_err(|_| "Driver still in use")
        .and_then(|d| {
            Handle::current()
                .block_on(d.quit())
                .map_err(|_| "Failed to quit driver")
        })
    {
        eprintln!("Warning: {}", e);
    }

    // Clean up orphaned baselines (only in interactive mode)
    let is_ci = std::env::var("CI").is_ok();
    if !is_ci && let Err(e) = cleanup_orphaned_baselines(&baseline_dir, &variants_for_cleanup) {
        eprintln!("Warning: Failed to clean up orphaned baselines: {}", e);
    }

    // Cleanup happens via Drop (geckodriver, storybook)
    conclusion.exit();
}
