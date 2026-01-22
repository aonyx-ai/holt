use crate::config::Config;
use clawless::prelude::*;
use tokio::process::Command;

#[derive(Debug, Args)]
pub struct ServeArgs {
    /// Port to serve on (default from config or 8080)
    #[arg(short, long)]
    port: Option<u16>,

    /// Open browser automatically (default from config)
    #[arg(short, long)]
    open: bool,
}

/// Start the development server
#[command]
pub async fn serve(args: ServeArgs) -> CommandResult {
    let config = Config::load().map_err(|e| Error::msg(format!("Failed to load config: {e}")))?;

    let port = args.port.unwrap_or(config.serve.port);
    let open = args.open || config.serve.open;

    let mut cmd = Command::new("trunk");
    cmd.arg("serve");
    cmd.arg("--port").arg(port.to_string());
    cmd.current_dir(&config.book.path);

    if open {
        cmd.arg("--open");
    }

    let status = cmd.status().await?;

    if !status.success() {
        return Err(Error::msg("trunk serve failed"));
    }

    Ok(())
}
