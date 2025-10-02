//! # Visual Regression Testing
//!
//! Captures screenshots of component stories and compares them against baseline images.
//!
//! ## Usage
//!
//! ```bash
//! just ui_book test-visual
//! ```
//!
//! The tool automatically manages geckodriver and the storybook server.
//!
//! ## Workflow
//!
//! First run creates baselines in `tests/visual-baselines/`. Subsequent runs compare screenshots
//! and prompt for approval on differences:
//!
//! ```text
//! ✓ button/default matches baseline
//! ✗ button/destructive differs from baseline
//!   Screenshot differs for button/destructive. Accept new baseline? [y/N]:
//! ```
//!
//! Baseline images are committed to git and should be included in PRs when visuals change.

use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::Duration;

use thirtyfour::prelude::*;

const SERVER_URL: &str = "http://localhost:8080";
const BASELINE_DIR: &str = "tests/visual-baselines";

/// Story metadata extracted from the storybook
#[derive(Debug, Clone)]
struct StoryVariant {
    story_id: String,
    variant_index: usize,
    name: String,
}

/// Manages the geckodriver process
struct GeckoDriver {
    process: Child,
}

impl GeckoDriver {
    fn start() -> Result<Self, Box<dyn std::error::Error>> {
        println!("Starting geckodriver...");

        let process = Command::new("geckodriver")
            .args(["--port", "4444"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;

        // Wait for geckodriver to be ready
        thread::sleep(Duration::from_secs(2));

        Ok(GeckoDriver { process })
    }
}

impl Drop for GeckoDriver {
    fn drop(&mut self) {
        println!("Shutting down geckodriver...");
        let _ = self.process.kill();
        let _ = self.process.wait();
    }
}

/// Manages the trunk serve process
struct TrunkServer {
    process: Child,
}

impl TrunkServer {
    fn start() -> Result<Self, Box<dyn std::error::Error>> {
        println!("Starting trunk server...");

        let process = Command::new("trunk")
            .args(["serve", "--port", "8080"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;

        // Wait for server to be ready
        for i in 0..30 {
            thread::sleep(Duration::from_secs(1));
            if let Ok(response) = ureq::get(SERVER_URL).call()
                && response.status() == 200
            {
                println!("Server is ready!");
                return Ok(TrunkServer { process });
            }
            if i % 5 == 0 {
                println!("Waiting for server to start... ({}/30)", i);
            }
        }

        Err("Server failed to start within 30 seconds".into())
    }
}

impl Drop for TrunkServer {
    fn drop(&mut self) {
        println!("Shutting down trunk server...");
        let _ = self.process.kill();
        let _ = self.process.wait();
    }
}

/// Discovers all stories and variants by scraping the storybook
async fn discover_stories() -> WebDriverResult<Vec<StoryVariant>> {
    println!("Discovering stories...");

    // For MVP, we'll hardcode the stories we know exist
    // In the future, we could scrape the storybook or use a build script
    let stories = vec![
        (
            "badge",
            vec!["default", "secondary", "destructive", "outline"],
        ),
        (
            "button",
            vec![
                "default",
                "destructive",
                "outline",
                "secondary",
                "ghost",
                "link",
                "icon",
                "loading",
            ],
        ),
        ("breadcrumb", vec!["default"]),
        ("card", vec!["default"]),
        ("input", vec!["default", "disabled", "with_label"]),
        ("checkbox", vec!["default", "checked", "disabled"]),
        ("collapsible", vec!["default"]),
        ("label", vec!["default"]),
        ("select", vec!["default"]),
    ];

    let mut variants = Vec::new();
    for (story_id, variant_names) in stories {
        for (idx, name) in variant_names.iter().enumerate() {
            variants.push(StoryVariant {
                story_id: story_id.to_string(),
                variant_index: idx,
                name: name.to_string(),
            });
        }
    }

    println!("Found {} story variants", variants.len());
    Ok(variants)
}

/// Captures a screenshot of a story variant
async fn capture_screenshot(
    driver: &WebDriver,
    variant: &StoryVariant,
) -> WebDriverResult<Vec<u8>> {
    let url = format!(
        "{}/visual-test/{}/{}",
        SERVER_URL, variant.story_id, variant.variant_index
    );

    println!("  Capturing: {}/{}", variant.story_id, variant.name);
    driver.goto(&url).await?;

    // Wait a bit for rendering to complete
    thread::sleep(Duration::from_millis(500));

    driver.screenshot_as_png().await
}

/// Gets the baseline path for a story variant
fn get_baseline_path(variant: &StoryVariant) -> PathBuf {
    Path::new(BASELINE_DIR)
        .join(&variant.story_id)
        .join(format!("{}.png", variant.name))
}

/// Compares two images (simple byte comparison for MVP)
fn images_match(img1: &[u8], img2: &[u8]) -> bool {
    img1 == img2
}

/// Prompts user for approval of a diff
fn prompt_user_approval(variant: &StoryVariant) -> io::Result<bool> {
    print!(
        "  Screenshot differs for {}/{}. Accept new baseline? [y/N]: ",
        variant.story_id, variant.name
    );
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().eq_ignore_ascii_case("y"))
}

/// Processes a single story variant
async fn process_variant(
    driver: &WebDriver,
    variant: &StoryVariant,
) -> Result<bool, Box<dyn std::error::Error>> {
    let screenshot = capture_screenshot(driver, variant).await?;
    let baseline_path = get_baseline_path(variant);

    // Check if baseline exists
    if baseline_path.exists() {
        let baseline = fs::read(&baseline_path)?;

        if images_match(&screenshot, &baseline) {
            println!("  ✓ {}/{} matches baseline", variant.story_id, variant.name);
            Ok(true)
        } else {
            println!(
                "  ✗ {}/{} differs from baseline",
                variant.story_id, variant.name
            );

            if prompt_user_approval(variant)? {
                fs::write(&baseline_path, screenshot)?;
                println!("  → Baseline updated");
                Ok(true)
            } else {
                println!("  → Baseline not updated");
                Ok(false)
            }
        }
    } else {
        println!("  + {}/{} (new baseline)", variant.story_id, variant.name);

        // Create parent directory if needed
        if let Some(parent) = baseline_path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&baseline_path, screenshot)?;
        println!("  → Baseline created");
        Ok(true)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Holt Visual Regression Testing");
    println!("================================\n");

    // Start geckodriver
    let _geckodriver = GeckoDriver::start()?;

    // Start the trunk server
    let _server = TrunkServer::start()?;

    // Set up WebDriver
    println!("Connecting to WebDriver...");
    let caps = DesiredCapabilities::firefox();
    let driver = WebDriver::new("http://localhost:4444", caps).await?;

    // Set viewport size for consistent screenshots
    driver.set_window_rect(0, 0, 1280, 720).await?;

    // Discover stories
    let variants = discover_stories().await?;

    println!("\nProcessing {} story variants...\n", variants.len());

    // Process each variant
    let mut passed = 0;
    let mut failed = 0;

    for variant in variants {
        match process_variant(&driver, &variant).await {
            Ok(true) => passed += 1,
            Ok(false) => failed += 1,
            Err(e) => {
                println!(
                    "  ✗ Error processing {}/{}: {}",
                    variant.story_id, variant.name, e
                );
                failed += 1;
            }
        }
    }

    // Clean up
    driver.quit().await?;

    println!("\n================================");
    println!("Results: {} passed, {} failed", passed, failed);

    if failed > 0 {
        std::process::exit(1);
    }

    Ok(())
}
