//! # Visual Regression Testing
//!
//! Captures screenshots of component stories and compares them against baseline images.
//!
//! ## Usage
//!
//! ```bash
//! just kit-docs test-visual
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
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

use eframe::egui;
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

        // In CI, show geckodriver output for debugging
        let is_ci = std::env::var("CI").is_ok();
        let mut cmd = Command::new("geckodriver");
        cmd.args(["--port", "4444"]);

        if !is_ci {
            cmd.stdout(Stdio::null()).stderr(Stdio::null());
        }

        let process = cmd.spawn()?;

        // Wait for geckodriver to be ready
        thread::sleep(Duration::from_secs(2));

        Ok(GeckoDriver { process })
    }
}

impl Drop for GeckoDriver {
    fn drop(&mut self) {
        println!("Shutting down geckodriver...");
        self.process.kill().expect("couldn't kill geckodriver");
    }
}

/// Manages the trunk serve process
struct TrunkServer {
    process: Child,
}

impl TrunkServer {
    fn start() -> Result<Self, Box<dyn std::error::Error>> {
        println!("Pre-building WASM app...");

        // Build the app first to avoid timeout issues with trunk serve
        // Use debug build for faster compilation
        let build_status = Command::new("trunk").args(["build"]).status()?;

        if !build_status.success() {
            return Err("Failed to build WASM app".into());
        }

        println!("Starting trunk server...");

        // In CI, show trunk output for debugging
        let is_ci = std::env::var("CI").is_ok();
        let mut cmd = Command::new("trunk");
        // Disable auto-reload to prevent rebuilds when baselines change during test
        cmd.args(["serve", "--port", "8080", "--no-autoreload", "true"]);

        if !is_ci {
            cmd.stdout(Stdio::null()).stderr(Stdio::null());
        }

        let mut process = cmd.spawn()?;

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

        // Kill the process before returning error
        println!("Timeout reached, killing trunk server...");
        process.kill().expect("couldn't kill trunk server");

        Err("Server failed to start within 30 seconds".into())
    }
}

impl Drop for TrunkServer {
    fn drop(&mut self) {
        println!("Shutting down trunk server...");
        self.process.kill().expect("couldn't kill trunk server");
    }
}

/// Discovers all stories and variants by scraping the storybook navigation
async fn discover_stories(driver: &WebDriver) -> WebDriverResult<Vec<StoryVariant>> {
    println!("Discovering stories...");

    // Navigate to the storybook home page
    driver.goto(&format!("{}/", SERVER_URL)).await?;

    // Wait for stories to load
    tokio::time::sleep(Duration::from_millis(1000)).await;

    // Get all story links from the navigation
    let story_links = driver.find_all(By::Css("nav a[href^='/story/']")).await?;

    let mut story_ids = Vec::new();
    for link in story_links {
        if let Ok(href) = link.attr("href").await
            && let Some(href_str) = href
        {
            // Extract story ID from href like "/story/button"
            if let Some(id) = href_str.strip_prefix("/story/") {
                story_ids.push(id.to_string());
            }
        }
    }

    println!("Found {} stories", story_ids.len());

    // For each story, navigate to it and count variants
    let mut variants = Vec::new();
    for story_id in story_ids {
        driver
            .goto(&format!("{}/story/{}", SERVER_URL, story_id))
            .await?;
        tokio::time::sleep(Duration::from_millis(500)).await;

        // Find the select element for variants
        if let Ok(select) = driver.find(By::Css("select")).await
            && let Ok(options) = select.find_all(By::Tag("option")).await
        {
            for (idx, option) in options.iter().enumerate() {
                if let Ok(name) = option.text().await {
                    variants.push(StoryVariant {
                        story_id: story_id.clone(),
                        variant_index: idx,
                        name: name.to_lowercase().replace(' ', "_"),
                    });
                }
            }
        }
    }

    println!("Found {} total story variants", variants.len());
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
    tokio::time::sleep(Duration::from_millis(500)).await;

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

/// egui app for comparing images
struct ImageCompareApp {
    title: String,
    baseline_image: egui::ColorImage,
    screenshot_image: egui::ColorImage,
    baseline_texture: Option<egui::TextureHandle>,
    screenshot_texture: Option<egui::TextureHandle>,
    show_baseline: bool,
    result_tx: mpsc::Sender<bool>,
    sent_result: bool,
}

impl ImageCompareApp {
    fn new(
        _cc: &eframe::CreationContext<'_>,
        variant: &StoryVariant,
        baseline: &[u8],
        screenshot: &[u8],
        result_tx: mpsc::Sender<bool>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        use image::ImageReader;
        use std::io::Cursor;

        let title = format!("Visual Diff: {}/{}", variant.story_id, variant.name);

        // Load images
        let baseline_img = ImageReader::new(Cursor::new(baseline))
            .with_guessed_format()?
            .decode()?;
        let screenshot_img = ImageReader::new(Cursor::new(screenshot))
            .with_guessed_format()?
            .decode()?;

        // Convert to egui ColorImage (no texture size limits)
        let baseline_rgba = baseline_img.to_rgba8();
        let screenshot_rgba = screenshot_img.to_rgba8();

        let baseline_image = egui::ColorImage::from_rgba_unmultiplied(
            [
                baseline_rgba.width() as usize,
                baseline_rgba.height() as usize,
            ],
            &baseline_rgba,
        );

        let screenshot_image = egui::ColorImage::from_rgba_unmultiplied(
            [
                screenshot_rgba.width() as usize,
                screenshot_rgba.height() as usize,
            ],
            &screenshot_rgba,
        );

        Ok(Self {
            title,
            baseline_image,
            screenshot_image,
            baseline_texture: None,
            screenshot_texture: None,
            show_baseline: true,
            result_tx,
            sent_result: false,
        })
    }
}

impl eframe::App for ImageCompareApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Header with controls
            ui.horizontal(|ui| {
                ui.heading(&self.title);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("✗ Reject").clicked() && !self.sent_result {
                        let _ = self.result_tx.send(false);
                        self.sent_result = true;
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                    if ui.button("✓ Accept New").clicked() && !self.sent_result {
                        let _ = self.result_tx.send(true);
                        self.sent_result = true;
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                    ui.separator();
                    if ui
                        .selectable_label(!self.show_baseline, "New Screenshot")
                        .clicked()
                    {
                        self.show_baseline = false;
                    }
                    if ui
                        .selectable_label(self.show_baseline, "Baseline")
                        .clicked()
                    {
                        self.show_baseline = true;
                    }
                    ui.label("View:");
                });
            });

            ui.separator();

            // Display the selected image, scaled to fit
            let available_size = ui.available_size();

            egui::ScrollArea::both()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    // Load texture on demand (lazy loading)
                    let (image, texture_slot, name) = if self.show_baseline {
                        (&self.baseline_image, &mut self.baseline_texture, "baseline")
                    } else {
                        (
                            &self.screenshot_image,
                            &mut self.screenshot_texture,
                            "screenshot",
                        )
                    };

                    // Create texture if not already loaded
                    let texture = texture_slot.get_or_insert_with(|| {
                        ctx.load_texture(name, image.clone(), egui::TextureOptions::LINEAR)
                    });

                    // Calculate size to fit in window while maintaining aspect ratio
                    let original_size = texture.size_vec2();
                    let scale = (available_size.x / original_size.x)
                        .min(available_size.y / original_size.y)
                        .min(1.0); // Don't upscale
                    let display_size = original_size * scale;

                    ui.image(egui::load::SizedTexture::new(texture.id(), display_size));
                });
        });
    }
}

/// Terminal-based fallback for comparing images
fn prompt_user_approval_terminal(
    variant: &StoryVariant,
    _baseline: &[u8],
    screenshot: &[u8],
    baseline_path: &Path,
) -> io::Result<bool> {
    println!("\n  GUI not available, using terminal mode");

    // Save new screenshot to temp file
    let temp_dir = std::env::temp_dir();
    let temp_path = temp_dir.join(format!(
        "visual-diff-{}-{}-new.png",
        variant.story_id, variant.name
    ));
    fs::write(&temp_path, screenshot)?;

    // Get absolute paths
    let baseline_abs = baseline_path
        .canonicalize()
        .unwrap_or_else(|_| baseline_path.to_path_buf());
    let temp_abs = temp_path
        .canonicalize()
        .unwrap_or_else(|_| temp_path.clone());

    println!("\n  Compare images:");
    println!("    Baseline:       {}", baseline_abs.display());
    println!("    New screenshot: {}", temp_abs.display());

    // Try to open both in default image viewer
    #[cfg(target_os = "macos")]
    {
        let _ = Command::new("open")
            .args([baseline_abs.as_os_str(), temp_abs.as_os_str()])
            .spawn();
    }

    #[cfg(target_os = "linux")]
    {
        let _ = Command::new("xdg-open").arg(&baseline_abs).spawn();
        let _ = Command::new("xdg-open").arg(&temp_abs).spawn();
    }

    #[cfg(target_os = "windows")]
    {
        let _ = Command::new("cmd")
            .args(["/C", "start", ""])
            .arg(&baseline_abs)
            .spawn();
        let _ = Command::new("cmd")
            .args(["/C", "start", ""])
            .arg(&temp_abs)
            .spawn();
    }

    print!(
        "\n  Screenshot differs for {}/{}. Accept new baseline? [y/N]: ",
        variant.story_id, variant.name
    );
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().eq_ignore_ascii_case("y"))
}

/// Shows image comparison UI and prompts user for approval
fn prompt_user_approval(
    variant: &StoryVariant,
    baseline: &[u8],
    screenshot: &[u8],
    baseline_path: &Path,
) -> io::Result<bool> {
    // Check if we can use GUI (not in CI, has display on Linux or always try on macOS/Windows)
    let use_gui = std::env::var("CI").is_err()
        && (cfg!(not(target_os = "linux")) || std::env::var("DISPLAY").is_ok());

    if !use_gui {
        return prompt_user_approval_terminal(variant, baseline, screenshot, baseline_path);
    }

    println!("\n  Opening comparison window...");

    let (tx, rx) = mpsc::channel();
    let variant_clone = variant.clone();
    let baseline_clone = baseline.to_vec();
    let screenshot_clone = screenshot.to_vec();

    // Run on main thread (required for macOS)
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 800.0])
            .with_title(format!(
                "Visual Diff: {}/{}",
                variant_clone.story_id, variant_clone.name
            )),
        ..Default::default()
    };

    // Use Arc<Mutex> to share the result between threads
    let result_shared = Arc::new(Mutex::new(None));
    let result_clone = Arc::clone(&result_shared);

    let result = std::thread::scope(|s| {
        // Spawn a thread to wait for the result
        let handle = s.spawn(move || {
            if let Ok(decision) = rx.recv() {
                *result_clone.lock().unwrap() = Some(decision);
            }
        });

        // Run GUI on this thread
        let gui_result = eframe::run_native(
            "Image Compare",
            options,
            Box::new(move |cc| {
                match ImageCompareApp::new(
                    cc,
                    &variant_clone,
                    &baseline_clone,
                    &screenshot_clone,
                    tx,
                ) {
                    Ok(app) => Ok(Box::new(app)),
                    Err(e) => {
                        let err_msg = format!("Failed to create comparison window: {}", e);
                        eprintln!("{}", err_msg);
                        Err(err_msg.into())
                    }
                }
            }),
        );

        // If GUI failed, return None to trigger fallback
        if gui_result.is_err() {
            return None;
        }

        // Wait for the thread to finish
        let _ = handle.join();

        // Get the result
        result_shared.lock().unwrap().take()
    });

    match result {
        Some(decision) => {
            println!("  → Comparison window closed");
            Ok(decision)
        }
        None => {
            eprintln!("  GUI failed, falling back to terminal mode");
            prompt_user_approval_terminal(variant, baseline, screenshot, baseline_path)
        }
    }
}

/// Processes a single story variant
async fn process_variant(
    driver: &WebDriver,
    variant: &StoryVariant,
) -> Result<bool, Box<dyn std::error::Error>> {
    let screenshot = capture_screenshot(driver, variant).await?;
    let baseline_path = get_baseline_path(variant);
    let is_ci = std::env::var("CI").is_ok();

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

            if is_ci {
                // In CI mode, fail the test and save diff for review
                let diff_dir = Path::new("tests/visual-diffs");
                fs::create_dir_all(diff_dir.join(&variant.story_id))?;
                let diff_path = diff_dir
                    .join(&variant.story_id)
                    .join(format!("{}.png", variant.name));
                fs::write(&diff_path, screenshot)?;
                println!("  → Diff saved to {}", diff_path.display());
                Ok(false)
            } else {
                // Interactive mode
                if prompt_user_approval(variant, &baseline, &screenshot, &baseline_path)? {
                    fs::write(&baseline_path, screenshot)?;
                    println!("  → Baseline updated");
                    Ok(true)
                } else {
                    println!("  → Baseline not updated");
                    Ok(false)
                }
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
    let mut caps = DesiredCapabilities::firefox();

    // Run headless in CI (no display server available)
    let is_ci = std::env::var("CI").is_ok();
    if is_ci {
        caps.set_headless()?;
        println!("Running Firefox in headless mode");
    }

    let driver = WebDriver::new("http://localhost:4444", caps).await?;

    // Set viewport size for consistent screenshots
    driver.set_window_rect(0, 0, 1280, 720).await?;

    // Discover stories
    let variants = discover_stories(&driver).await?;

    println!("\nProcessing {} story variants...\n", variants.len());

    // Process each variant
    let mut passed = 0;
    let mut failed = 0;

    for variant in &variants {
        match process_variant(&driver, variant).await {
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

    if !is_ci {
        // Clean up orphaned baseline images
        cleanup_orphaned_baselines(&variants)?;
    }

    if failed > 0 {
        std::process::exit(1);
    }

    Ok(())
}

/// Removes baseline images that no longer have corresponding stories/variants
fn cleanup_orphaned_baselines(variants: &[StoryVariant]) -> io::Result<()> {
    let baseline_dir = Path::new(BASELINE_DIR);
    if !baseline_dir.exists() {
        return Ok(());
    }

    // Build a set of expected baseline paths
    let mut expected_paths = std::collections::HashSet::new();
    for variant in variants {
        let path = get_baseline_path(variant);
        expected_paths.insert(path);
    }

    // Walk through all baseline files
    let mut orphaned = Vec::new();
    for entry in fs::read_dir(baseline_dir)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            // Check files within story directories
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

    // Delete orphaned files
    if !orphaned.is_empty() {
        println!("\nCleaning up {} orphaned baseline(s):", orphaned.len());
        for path in orphaned {
            println!("  Removing: {}", path.display());
            fs::remove_file(&path)?;

            // Remove parent directory if empty
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
