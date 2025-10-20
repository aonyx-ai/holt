//! Renders a single story variant to a PNG file for LLM agent use.
//!
//! ## Usage
//!
//! ```bash
//! just kit-docs render-variant <story_id> <variant> [output_path]
//! ```
//!
//! The `<variant>` parameter can be either:
//! - A numeric index (e.g., `0`, `1`, `2`)
//! - A variant name (e.g., `default`, `destructive`, `outline`)
//!
//! Examples:
//! ```bash
//! # Render button story, first variant to default location (./variant.png)
//! just kit-docs render-variant button 0
//!
//! # Render button story, "destructive" variant by name
//! just kit-docs render-variant button destructive
//!
//! # Render select story, variant 2 to specific file
//! just kit-docs render-variant select 2 ./select-variant-2.png
//! ```
//!
//! The tool automatically manages geckodriver and the storybook server.

use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::Duration;
use thirtyfour::prelude::*;

const SERVER_URL: &str = "http://localhost:8080";

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

/// Discovers variant names for a story by scraping the select dropdown
async fn discover_variant_names(
    driver: &WebDriver,
    story_id: &str,
) -> WebDriverResult<Vec<String>> {
    println!("Discovering variants for story '{}'...", story_id);

    driver
        .goto(&format!("{}/story/{}", SERVER_URL, story_id))
        .await?;

    tokio::time::sleep(Duration::from_millis(500)).await;

    // Find the select element for variants
    let select = driver.find(By::Css("select")).await?;
    let options = select.find_all(By::Tag("option")).await?;

    let mut variant_names = Vec::new();
    for option in options {
        if let Ok(name) = option.text().await {
            variant_names.push(name);
        }
    }

    Ok(variant_names)
}

/// Resolves a variant identifier (name or index) to an index
async fn resolve_variant_index(
    driver: &WebDriver,
    story_id: &str,
    variant_identifier: &str,
) -> Result<usize, Box<dyn std::error::Error>> {
    // Try to parse as index first
    if let Ok(index) = variant_identifier.parse::<usize>() {
        return Ok(index);
    }

    // Otherwise, treat as variant name and discover all variants
    let variant_names = discover_variant_names(driver, story_id).await?;

    if variant_names.is_empty() {
        return Err(format!("No variants found for story '{}'", story_id).into());
    }

    // Find matching variant name (case-insensitive)
    let normalized_input = variant_identifier.to_lowercase().replace('_', " ");
    for (index, name) in variant_names.iter().enumerate() {
        let normalized_name = name.to_lowercase();
        if normalized_name == normalized_input {
            println!(
                "Resolved variant '{}' to index {} ({})",
                variant_identifier, index, name
            );
            return Ok(index);
        }
    }

    // No match found - show available variants
    let available = variant_names
        .iter()
        .enumerate()
        .map(|(i, name)| format!("  {} - {}", i, name))
        .collect::<Vec<_>>()
        .join("\n");

    Err(format!(
        "Variant '{}' not found for story '{}'.\nAvailable variants:\n{}",
        variant_identifier, story_id, available
    )
    .into())
}

/// Captures a screenshot of a story variant
async fn capture_variant_screenshot(
    driver: &WebDriver,
    story_id: &str,
    variant_index: usize,
) -> WebDriverResult<Vec<u8>> {
    let url = format!("{}/visual-test/{}/{}", SERVER_URL, story_id, variant_index);

    println!(
        "Capturing screenshot for story '{}', variant {}",
        story_id, variant_index
    );

    driver.goto(&url).await?;

    // Wait for rendering to complete
    tokio::time::sleep(Duration::from_millis(500)).await;

    driver.screenshot_as_png().await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <story_id> <variant> [output_path]", args[0]);
        eprintln!();
        eprintln!("Arguments:");
        eprintln!("  <story_id>    Story identifier (e.g., 'button', 'select')");
        eprintln!(
            "  <variant>     Variant index (0, 1, 2...) or name ('default', 'destructive'...)"
        );
        eprintln!("  [output_path] Output file path (default: ./variant.png)");
        eprintln!();
        eprintln!("Examples:");
        eprintln!("  {} button 0", args[0]);
        eprintln!("  {} button destructive", args[0]);
        eprintln!("  {} select 2 ./output.png", args[0]);
        std::process::exit(1);
    }

    let story_id = &args[1];
    let variant_identifier = &args[2];

    let output_path: PathBuf = if args.len() > 3 {
        args[3].clone().into()
    } else {
        PathBuf::from("./variant.png")
    };

    println!("Holt Story Variant Renderer");
    println!("===========================\n");

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

    // Resolve variant identifier to index
    let variant_index = resolve_variant_index(&driver, story_id, variant_identifier).await?;

    // Capture the screenshot
    let screenshot = capture_variant_screenshot(&driver, story_id, variant_index).await?;

    // Save to file
    std::fs::write(&output_path, &screenshot)?;

    // Get absolute path for output
    let absolute_path = output_path
        .canonicalize()
        .unwrap_or_else(|_| output_path.clone());

    println!("\n✓ Screenshot saved to: {}", absolute_path.display());
    println!("  Size: {} bytes", screenshot.len());

    // Clean up
    driver.quit().await?;

    println!("\nDone!");

    Ok(())
}
