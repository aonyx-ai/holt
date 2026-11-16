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

/// Builds for production
#[command]
pub async fn build(args: BuildArgs, _ctx: Context) -> CommandResult {
    let config = Config::load().context("Failed to load config")?;

    let mut cmd = Command::new("trunk");
    cmd.arg("build");
    cmd.current_dir(config.book().path());

    match args.profile {
        BuildProfile::Debug => {}
        BuildProfile::Release => {
            cmd.arg("--release");
        }
    }

    let status = cmd.status().await.context("failed to run trunk")?;

    if !status.success() {
        return Err(Error::msg("trunk build failed"));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_args_is_send_sync_and_unpin() {
        fn assert_send_sync_and_unpin<T: Send + Sync + Unpin>() {}

        assert_send_sync_and_unpin::<BuildArgs>();
    }

    #[test]
    fn build_profile_is_send_sync_and_unpin() {
        fn assert_send_sync_and_unpin<T: Send + Sync + Unpin>() {}

        assert_send_sync_and_unpin::<BuildProfile>();
    }
}
