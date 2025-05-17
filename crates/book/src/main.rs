use clap::{Parser, Subcommand};
use proc_macro2::TokenStream;
use quote::quote;
use std::error::Error;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
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
                .args([
                    "run", "nightly", "cargo",
                    "doc", /*, "--no-deps", "-p holt-ui-book", "-p holt-book" */
                ])
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
                println!("target/doc/holt_ui_book.json not found, skipping processing");
                return Ok(());
            }

            // Read the JSON file
            let json_content = fs::read_to_string(json_path)?;
            let json: serde_json::Value = serde_json::from_str(&json_content)?;

            let external_crates = json["external_crates"]
                .as_object()
                .ok_or("External crates not found or not an object")?;

            // find crate with .name = "holt_book"
            let holt_book_crate = external_crates
                .iter()
                .find(|(_, val)| {
                    val["name"]
                        .as_str()
                        .map(|s| s == "holt_book")
                        .unwrap_or(false)
                })
                .ok_or("holt_book crate not found")?;
            let holt_book_crate_id = holt_book_crate.0.parse::<u64>()?;

            // Access the index property
            let paths = json["paths"]
                .as_object()
                .ok_or("paths not found or not an object")?;

            // Find the "Story" object with crate_id
            let mut story_id = None;

            for (id, obj) in paths {
                if let Some(obj) = obj.as_object() {
                    if let (Some(crate_id), Some(path)) = (
                        obj.get("crate_id").and_then(|v| v.as_u64()),
                        obj.get("path").and_then(|v| v.as_array()).and_then(|a| {
                            a.iter()
                                .map(|item| item.as_str())
                                .collect::<Option<Vec<&str>>>()
                        }),
                    ) {
                        if crate_id == holt_book_crate_id
                            && path == ["holt_book", "ui", "story", "Story"]
                        {
                            story_id = Some(id.clone()).map(|s| s.parse::<u64>()).transpose()?;
                            break;
                        }
                    }
                }
            }

            let story_id = story_id.ok_or_else(|| {
                format!(
                    "Story object with crate_id {} not found",
                    holt_book_crate_id
                )
            })?;

            println!("Found Story object with ID: {}", story_id);

            let index = json["index"]
                .as_object()
                .ok_or("index not found or not an object")?;
            let mut stories_ids: Vec<u64> = vec![];

            for (_, obj) in index {
                if let Some(obj) = obj.get("inner") {
                    if let Some(obj) = obj.get("impl") {
                        if let Some(obj) = obj.get("trait") {
                            if obj.get("path").and_then(|p| p.as_str()) != Some("Story")
                                || obj.get("id").and_then(|i| i.as_u64()) != Some(story_id)
                            {
                                continue;
                            }
                        }

                        if let Some(obj) = obj.get("for") {
                            if let Some(obj) = obj.get("resolved_path") {
                                stories_ids.push(
                                    obj.get("id")
                                        .and_then(|id| id.as_u64())
                                        .ok_or("id not found or not a number")?,
                                );
                            }
                        }
                    }
                }
            }

            let f = File::create("src/stories_docs.rs").unwrap();
            {
                let mut codegen = phf_codegen::Map::new();
                codegen.phf_path("holt_book");

                for id in stories_ids {
                    let obj = index
                        .get(&id.to_string())
                        .and_then(|o| o.as_object())
                        .ok_or("no item found or not an object")?;
                    let name = obj.get("name").and_then(|name| name.as_str()).unwrap_or("");
                    let docs = obj.get("docs").and_then(|docs| docs.as_str()).unwrap_or("");

                    codegen.entry(name.to_string(), &quote! { #docs }.to_string());
                }

                let map_tokens: TokenStream = dbg!(codegen.build().to_string()).parse().unwrap();

                let mut f = BufWriter::new(f);
                let parse_file = syn::parse_file(&dbg!(quote! {
                    pub static STORY_DOCS: holt_book::Map<&'static str, &'static str> = #map_tokens;
                }.to_string()));
                let buf = prettyplease::unparse(&parse_file?);
                f.write_all(buf.as_bytes()).unwrap();
            }

            // println!("Starting trunk serve...");

            // // Run trunk serve
            // let status = Command::new("trunk")
            //     .arg("serve")
            //     .status()?;

            // if !status.success() {
            //     eprintln!("trunk serve failed with status: {}", status);
            //     return Err("trunk serve failed".into());
            // }
        }
    }

    Ok(())
}
