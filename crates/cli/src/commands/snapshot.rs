//! CLI command for visual regression testing.

use crate::config::Config;
use crate::snapshot::{self, SnapshotConfig};
use clawless::prelude::*;

#[derive(Debug, Args)]
pub struct SnapshotArgs {
    /// Port to serve on (default from config or 8080)
    #[arg(short, long)]
    port: Option<u16>,
}

/// Run visual regression tests
#[command]
pub async fn snapshot(args: SnapshotArgs) -> CommandResult {
    let config = Config::load().map_err(|e| Error::msg(format!("Failed to load config: {e}")))?;

    let port = args.port.unwrap_or(config.serve.port);

    snapshot::run(SnapshotConfig {
        book_path: &config.book.path,
        port,
    })
    .await
    .map_err(Error::msg)
}
