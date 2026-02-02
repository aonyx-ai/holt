use crate::config::Config;
use clawless::prelude::*;
use tokio::process::Command;

#[derive(Debug, Args)]
pub struct BuildArgs {
    /// Build in release mode
    #[arg(short, long)]
    release: bool,
}

/// Build for production
#[command]
pub async fn build(args: BuildArgs, _ctx: Context) -> CommandResult {
    let config = Config::load().map_err(|e| Error::msg(format!("Failed to load config: {e}")))?;

    let mut cmd = Command::new("trunk");
    cmd.arg("build");
    cmd.current_dir(&config.book.path);

    if args.release {
        cmd.arg("--release");
    }

    let status = cmd.status().await?;

    if !status.success() {
        return Err(Error::msg("trunk build failed"));
    }

    Ok(())
}
