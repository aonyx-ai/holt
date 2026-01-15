//! `holt run` - Run the storybook development server

use clawless::prelude::*;
use std::process::Command;

use crate::config::HoltConfig;

#[derive(Debug, Args)]
pub struct RunArgs {
    /// Port to serve on (overrides holt.toml)
    #[arg(short, long)]
    port: Option<u16>,
}

/// Run the storybook development server
#[command]
pub async fn run(args: RunArgs, _ctx: Context) -> CommandResult {
    let (config, root) = HoltConfig::find_and_load()?;
    let crate_path = config.storybook_path(&root);
    let port = args.port.unwrap_or(config.storybook.port);

    println!("Starting storybook from {}...", crate_path.display());
    println!("Storybook will be available at http://localhost:{}", port);

    // For `holt run`, we want interactive mode - don't capture output
    let status = Command::new("trunk")
        .args(["serve", "--port", &port.to_string()])
        .current_dir(&crate_path)
        .status()?;

    std::process::exit(status.code().unwrap_or(1));
}
