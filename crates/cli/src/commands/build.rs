use crate::config::Config;
use clawless::clap;
use clawless::prelude::*;
use tokio::process::Command;

/// Build profile selection
#[derive(Copy, Clone, Debug, Default, clap::ValueEnum)]
pub enum BuildProfile {
    #[default]
    Debug,
    Release,
}

#[derive(Debug, Args)]
pub struct BuildArgs {
    /// Build profile
    #[arg(short, long, default_value = "debug")]
    profile: BuildProfile,
}

/// Build for production
#[command]
pub async fn build(args: BuildArgs, _ctx: Context) -> CommandResult {
    let config = Config::load().map_err(|e| Error::msg(format!("Failed to load config: {e}")))?;

    let mut cmd = Command::new("trunk");
    cmd.arg("build");
    cmd.current_dir(&config.book.path);

    match args.profile {
        BuildProfile::Debug => {}
        BuildProfile::Release => {
            cmd.arg("--release");
        }
    }

    let status = cmd.status().await?;

    if !status.success() {
        return Err(Error::msg("trunk build failed"));
    }

    Ok(())
}
