use clap::{Parser, Subcommand};
use leptos::prelude::*;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate documentation by running rustdoc command
    Run,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Run => {
            println!("Running rustdoc to generate JSON documentation...");

            let rustdoc_output = Command::new("rustup")
                .env("RUSTDOCFLAGS", "-Z unstable-options --output-format=json")
                .env("HOLT_BOOK_RUN_BUILD_SCRIPT", "false")
                .args(&["run", "nightly", "cargo", "doc"])
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .output()?;

            if !rustdoc_output.status.success() {
                let stderr = String::from_utf8_lossy(&rustdoc_output.stderr);
                eprintln!("rustdoc command failed: {}", stderr);
                // Continue with the process even if rustdoc fails
            } else {
                println!("rustdoc command completed successfully");
            }

            // Path to the JSON file
            let json_path = Path::new("../../target/doc/holt_ui_book.json");

            // Skip processing if the file doesn't exist yet
            if !json_path.exists() {
                println!("target/doc/ui-book.json not found, skipping processing");
                return Ok(());
            }

            // Read the JSON file
            let json_content = fs::read_to_string(json_path)?;
            let json: serde_json::Value = serde_json::from_str(&json_content)?;

            let external_crates = json["external_crates"].as_object().ok_or("External crates not found or not an object")?;

            // find crate with .name = "holt_book"
            let holt_book_crate = external_crates.iter().find(|(_, val)| val["name"].as_str().map(|s| s == "holt_book").unwrap_or(false)).ok_or("holt_book crate not found")?;
            let holt_book_crate_id = holt_book_crate.0.parse::<u64>()?;

            // Access the index property
            let index = json["index"].as_object().ok_or("Index not found or not an object")?;

            // Find the "Story" object with crate_id 0
            let mut story_id = None;
            let mut story_obj = None;

            for (id, obj) in index {
                if let Some(obj) = obj.as_object() {
                    if let (Some(crate_id), Some(name)) = (
                        obj.get("crate_id").and_then(|v| v.as_u64()),
                        obj.get("name").and_then(|v| v.as_str()),
                    ) {
                        if crate_id == holt_book_crate_id && name == "Story" {
                            story_id = Some(id.clone());
                            story_obj = Some(obj);
                            break;
                        }
                    }
                }
            }

            let story_id = story_id.ok_or_else(|| format!("Story object with crate_id {} not found", holt_book_crate_id))?;
            let story_obj = story_obj.ok_or("Story object data not found")?;

            println!("Found Story object with ID: {}", story_id);

            // Extract trait implementations
            let trait_implementations = story_obj
                .get("inner")
                .and_then(|inner| inner.get("trait"))
                .and_then(|trait_obj| trait_obj.get("implementations"))
                .and_then(|implementations| implementations.as_array())
                .ok_or("Trait implementations not found")?;

            println!("Found {} trait implementations", trait_implementations.len());

            // Find objects with those IDs
            for impl_id in trait_implementations {
                if let Some(id_str) = impl_id.as_str() {
                    if let Some(impl_obj) = index.get(id_str) {
                        println!("Implementation ID: {}", id_str);

                        // You can extract further information from each implementation object here
                        if let Some(impl_name) = impl_obj.get("name").and_then(|n| n.as_str()) {
                            println!("  Implementation name: {}", impl_name);
                        }
                    } else {
                        println!("Implementation with ID {} not found in index", id_str);
                    }
                }
            }

            println!("Starting trunk serve...");

            // Run trunk serve
            let status = Command::new("trunk")
                .arg("serve")
                .status()?;

            if !status.success() {
                eprintln!("trunk serve failed with status: {}", status);
                return Err("trunk serve failed".into());
            }
        }
    }

    Ok(())
}
