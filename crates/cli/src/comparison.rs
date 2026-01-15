//! Image comparison and baseline management

use crate::discovery::StoryVariant;
use crate::screenshot::capture_screenshot;
use eframe::egui;
use std::collections::HashSet;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Arc, Mutex, mpsc};
use thirtyfour::WebDriver;
use walkdir::WalkDir;

/// Gets the baseline path for a story variant
pub fn get_baseline_path(baseline_dir: &Path, variant: &StoryVariant) -> PathBuf {
    baseline_dir
        .join(&variant.story_id)
        .join(format!("{}.png", variant.name))
}

/// Compares two images (simple byte comparison)
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

        // Convert to egui ColorImage
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

/// Result of processing a variant
pub enum ProcessResult {
    /// Variant matches baseline
    Passed,
    /// Variant differs from baseline or is new
    Failed,
    /// Error occurred during processing
    Error(String),
}

/// Processes a single story variant - captures screenshot and compares to baseline
pub async fn process_variant(
    driver: &WebDriver,
    base_url: &str,
    baseline_dir: &Path,
    variant: &StoryVariant,
) -> ProcessResult {
    // Capture screenshot
    let screenshot = match capture_screenshot(driver, base_url, variant).await {
        Ok(s) => s,
        Err(e) => return ProcessResult::Error(e.to_string()),
    };

    let baseline_path = get_baseline_path(baseline_dir, variant);
    let is_ci = std::env::var("CI").is_ok();

    // Check if baseline exists
    if baseline_path.exists() {
        let baseline = match fs::read(&baseline_path) {
            Ok(b) => b,
            Err(e) => return ProcessResult::Error(e.to_string()),
        };

        if images_match(&screenshot, &baseline) {
            println!("  ✓ {}/{} matches baseline", variant.story_id, variant.name);
            ProcessResult::Passed
        } else {
            println!(
                "  ✗ {}/{} differs from baseline",
                variant.story_id, variant.name
            );

            if is_ci {
                if let Err(e) = fs::write(&baseline_path, screenshot) {
                    return ProcessResult::Error(e.to_string());
                }
                println!("  → New screenshot saved for artifact upload");
                ProcessResult::Failed
            } else {
                // Interactive mode
                match prompt_user_approval(variant, &baseline, &screenshot, &baseline_path) {
                    Ok(true) => {
                        if let Err(e) = fs::write(&baseline_path, screenshot) {
                            return ProcessResult::Error(e.to_string());
                        }
                        println!("  → Baseline updated");
                        ProcessResult::Passed
                    }
                    Ok(false) => {
                        println!("  → Baseline not updated");
                        ProcessResult::Failed
                    }
                    Err(e) => ProcessResult::Error(e.to_string()),
                }
            }
        }
    } else {
        println!("  + {}/{} (new baseline)", variant.story_id, variant.name);

        // Create parent directory if needed
        if let Some(parent) = baseline_path.parent()
            && let Err(e) = fs::create_dir_all(parent)
        {
            return ProcessResult::Error(e.to_string());
        }

        if let Err(e) = fs::write(&baseline_path, screenshot) {
            return ProcessResult::Error(e.to_string());
        }
        println!("  → Baseline created (test will fail until committed)");

        // Fail the test - new baselines must be reviewed and committed
        ProcessResult::Failed
    }
}

/// Removes baseline images that no longer have corresponding stories/variants
pub fn cleanup_orphaned_baselines(
    baseline_dir: &Path,
    variants: &[StoryVariant],
) -> io::Result<()> {
    if !baseline_dir.exists() {
        return Ok(());
    }

    // Build a set of expected baseline paths
    let expected_paths: HashSet<PathBuf> = variants
        .iter()
        .map(|v| get_baseline_path(baseline_dir, v))
        .collect();

    // Walk through all baseline files
    let mut orphaned = Vec::new();
    for entry in WalkDir::new(baseline_dir)
        .min_depth(2)
        .max_depth(2)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            let path = entry.path().to_path_buf();
            if !expected_paths.contains(&path) {
                orphaned.push(path);
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
