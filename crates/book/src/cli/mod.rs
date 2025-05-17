mod commands;

pub use commands::{Cli, Commands};

use clap::Parser;
use std::error::Error;
use std::path::PathBuf;

use crate::docs::codegen::PhfMapGenerator;
use crate::docs::extractor::StoryExtractor;
use crate::docs::parser::DefaultRustdocParser;
use crate::docs::pipeline::{DocumentationPipeline, RustdocProcessor};
use crate::docs::rustdoc::RustdocGenerator;

/// Run the CLI application
pub fn run() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Run => {
            let generator = RustdocGenerator::new(
                "json".to_string(),
                // vec!["holt-ui-book".to_string(), "holt-book".to_string()]
            );

            let parser = DefaultRustdocParser;
            let extractor = StoryExtractor;

            let processor = RustdocProcessor::new(parser, extractor);

            let code_generator = PhfMapGenerator;
            let output_path = PathBuf::from("src/stories_docs.rs");

            let pipeline =
                DocumentationPipeline::new(generator, processor, code_generator, output_path);

            pipeline.run()?;
        }
    }

    Ok(())
}
