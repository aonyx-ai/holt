mod commands;

pub use commands::{Cli, Commands};

use clap::Parser;
use std::error::Error;

/// Run the CLI application
pub fn run() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Run => {}
    }

    Ok(())
}
