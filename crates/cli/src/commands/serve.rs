use crate::config::{Config, OpenBrowser};
use clawless::prelude::*;
use tokio::process::Command;

#[derive(Debug, Args)]
pub struct ServeArgs {
    /// Port to serve on (default from config or 8080)
    #[arg(short, long)]
    port: Option<u16>,

    /// Open browser automatically (default from config)
    #[arg(
        short,
        long,
        num_args = 0,
        default_missing_value = "yes",
        default_value = "no"
    )]
    open: OpenBrowser,
}

/// Starts the development server
#[command]
pub async fn serve(args: ServeArgs, _ctx: Context) -> CommandResult {
    let config = Config::load().context("Failed to load config")?;

    let port = args.port.unwrap_or(config.serve().port());
    let open = args.open == OpenBrowser::Yes || config.serve().open() == OpenBrowser::Yes;

    let mut cmd = Command::new("trunk");
    cmd.arg("serve");
    cmd.arg("--port").arg(port.to_string());
    cmd.current_dir(config.book().path());

    if open {
        cmd.arg("--open");
    }

    let status = cmd.status().await.context("failed to run trunk")?;

    if !status.success() {
        return Err(Error::msg("trunk serve failed"));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serve_args_is_send_sync_and_unpin() {
        fn assert_send_sync_and_unpin<T: Send + Sync + Unpin>() {}

        assert_send_sync_and_unpin::<ServeArgs>();
    }
}
