//! CLI command for visual regression testing.

use crate::config::Config;
use crate::snapshot::{self, SnapshotConfig};
use clawless::prelude::*;
use std::io::IsTerminal;

#[derive(Debug, Args)]
pub struct SnapshotArgs {
    /// Run in check mode: purely pass/fail, no saving, no prompts.
    /// Exits non-zero on any failure. Implies --headless --no-save.
    #[arg(long)]
    check: bool,

    /// Run the browser in headless mode (no visible window).
    /// Default: headless when stdout is not a terminal.
    #[arg(long)]
    headless: bool,

    /// Force a visible browser window even in non-interactive shells.
    #[arg(long = "no-headless", conflicts_with = "headless")]
    no_headless: bool,

    /// Save new and mismatched screenshots to the baseline directory [default: true].
    #[arg(long)]
    save: bool,

    /// Do not save screenshots.
    #[arg(long = "no-save", conflicts_with = "save")]
    no_save: bool,
}

impl SnapshotArgs {
    fn headless(&self) -> bool {
        if self.check || self.headless {
            return true;
        }
        if self.no_headless {
            return false;
        }
        !std::io::stdout().is_terminal()
    }

    fn save(&self) -> bool {
        if self.check || self.no_save {
            return false;
        }
        true
    }
}

/// Run visual regression tests
#[command]
pub async fn snapshot(args: SnapshotArgs, _ctx: Context) -> CommandResult {
    let config = Config::load().map_err(|e| Error::msg(format!("Failed to load config: {e}")))?;

    snapshot::run(SnapshotConfig {
        book_path: &config.book.path,
        headless: args.headless(),
        save: args.save(),
        check: args.check,
    })
    .await
    .map_err(Error::msg)
}
