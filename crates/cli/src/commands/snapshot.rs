//! CLI command for visual regression testing.

use crate::config::Config;
use crate::snapshot::{self, SnapshotConfig};
use clawless::prelude::*;

#[derive(Debug, Args)]
pub struct SnapshotArgs {}

/// Run visual regression tests
#[command]
pub async fn snapshot(_args: SnapshotArgs, _ctx: Context) -> CommandResult {
    let config = Config::load().map_err(|e| Error::msg(format!("Failed to load config: {e}")))?;

    snapshot::run(SnapshotConfig {
        book_path: &config.book.path,
    })
    .await
    .map_err(Error::msg)
}
