//! Image comparison UI for reviewing visual differences.

use super::story::StoryVariant;
use eframe::egui;
use std::fs;
use std::io::{self, Write as IoWrite};
use std::path::Path;
use std::process::Command;
use std::sync::{Arc, Mutex, mpsc};

/// Compares two images (simple byte comparison).
pub fn images_match(img1: &[u8], img2: &[u8]) -> bool {
    img1 == img2
}

/// egui app for comparing images.
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

        let baseline_img = ImageReader::new(Cursor::new(baseline))
            .with_guessed_format()?
            .decode()?;
        let screenshot_img = ImageReader::new(Cursor::new(screenshot))
            .with_guessed_format()?
            .decode()?;

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
            ui.horizontal(|ui| {
                ui.heading(&self.title);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Reject").clicked() && !self.sent_result {
                        let _ = self.result_tx.send(false);
                        self.sent_result = true;
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                    if ui.button("Accept New").clicked() && !self.sent_result {
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

            let available_size = ui.available_size();

            egui::ScrollArea::both()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    let (image, texture_slot, name) = if self.show_baseline {
                        (&self.baseline_image, &mut self.baseline_texture, "baseline")
                    } else {
                        (
                            &self.screenshot_image,
                            &mut self.screenshot_texture,
                            "screenshot",
                        )
                    };

                    let texture = texture_slot.get_or_insert_with(|| {
                        ctx.load_texture(name, image.clone(), egui::TextureOptions::LINEAR)
                    });

                    let original_size = texture.size_vec2();
                    let scale = (available_size.x / original_size.x)
                        .min(available_size.y / original_size.y)
                        .min(1.0);
                    let display_size = original_size * scale;

                    ui.image(egui::load::SizedTexture::new(texture.id(), display_size));
                });
        });
    }
}

/// Terminal-based fallback for comparing images.
fn prompt_user_approval_terminal(
    variant: &StoryVariant,
    _baseline: &[u8],
    screenshot: &[u8],
    baseline_path: &Path,
) -> io::Result<bool> {
    println!("\n  GUI not available, using terminal mode");

    let temp_dir = std::env::temp_dir();
    let temp_path = temp_dir.join(format!(
        "visual-diff-{}-{}-new.png",
        variant.story_id, variant.name
    ));
    fs::write(&temp_path, screenshot)?;

    let baseline_abs = baseline_path
        .canonicalize()
        .unwrap_or_else(|_| baseline_path.to_path_buf());
    let temp_abs = temp_path
        .canonicalize()
        .unwrap_or_else(|_| temp_path.clone());

    println!("\n  Compare images:");
    println!("    Baseline:       {}", baseline_abs.display());
    println!("    New screenshot: {}", temp_abs.display());

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

/// Shows image comparison UI and prompts user for approval.
pub fn prompt_user_approval(
    variant: &StoryVariant,
    baseline: &[u8],
    screenshot: &[u8],
    baseline_path: &Path,
) -> io::Result<bool> {
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

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 800.0])
            .with_title(format!(
                "Visual Diff: {}/{}",
                variant_clone.story_id, variant_clone.name
            )),
        ..Default::default()
    };

    let result_shared = Arc::new(Mutex::new(None));
    let result_clone = Arc::clone(&result_shared);

    let result = std::thread::scope(|s| {
        let handle = s.spawn(move || {
            if let Ok(decision) = rx.recv() {
                *result_clone.lock().unwrap() = Some(decision);
            }
        });

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

        if gui_result.is_err() {
            return None;
        }

        let _ = handle.join();
        result_shared.lock().unwrap().take()
    });

    match result {
        Some(decision) => {
            println!("  -> Comparison window closed");
            Ok(decision)
        }
        None => {
            eprintln!("  GUI failed, falling back to terminal mode");
            prompt_user_approval_terminal(variant, baseline, screenshot, baseline_path)
        }
    }
}
