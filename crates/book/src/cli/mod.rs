mod commands;

pub use commands::{Cli, Commands};

use clap::Parser;
use std::error::Error;
use std::path::PathBuf;

use crate::docs::codegen::PhfMapGenerator;
use crate::docs::extractor::DefaultStoryExtractor;
use crate::docs::parser::DefaultRustdocParser;
use crate::docs::pipeline::DocumentationPipeline;
use crate::docs::rustdoc::RustdocGenerator;

/// Run the CLI application
pub fn run() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Run => {
            let output_path = PathBuf::from("src/stories_docs.rs");

            let pipeline = DocumentationPipeline::new(
                RustdocGenerator::new("json".to_string()),
                DefaultRustdocParser,
                DefaultStoryExtractor,
                PhfMapGenerator,
                output_path,
            );

            pipeline.run()?;
        }
    }

    Ok(())
}
