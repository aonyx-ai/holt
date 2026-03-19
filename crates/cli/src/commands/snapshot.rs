//! CLI command for visual regression testing.

use crate::config::Config;
use crate::snapshot::{self, HeadlessMode, SaveMode, SnapshotConfig, SnapshotMode};
use clawless::prelude::*;
use std::io::IsTerminal;

#[derive(Debug, Args)]
pub struct SnapshotArgs {
    /// Run in check mode: purely pass/fail, no saving, no prompts.
    /// Exits non-zero on any failure. Implies --headless --no-save.
    #[arg(
        long,
        num_args = 0,
        default_missing_value = "check",
        default_value = "interactive"
    )]
    check: SnapshotMode,

    /// Run the browser in headless mode (no visible window).
    /// Default: headless when stdout is not a terminal.
    #[arg(
        long,
        num_args = 0,
        default_missing_value = "headless",
        default_value = "auto"
    )]
    headless: HeadlessMode,

    /// Do not save screenshots.
    #[arg(
        long = "no-save",
        num_args = 0,
        default_missing_value = "no-save",
        default_value = "save"
    )]
    save: SaveMode,
}

impl SnapshotArgs {
    fn resolve_headless(&self) -> HeadlessMode {
        match self.check {
            SnapshotMode::Check => HeadlessMode::Headless,
            SnapshotMode::Interactive => match self.headless {
                HeadlessMode::Auto => {
                    if std::io::stdout().is_terminal() {
                        HeadlessMode::Visible
                    } else {
                        HeadlessMode::Headless
                    }
                }
                other => other,
            },
        }
    }

    fn resolve_save(&self) -> SaveMode {
        match self.check {
            SnapshotMode::Check => SaveMode::NoSave,
            SnapshotMode::Interactive => self.save,
        }
    }
}

/// Run visual regression tests
#[command]
pub async fn snapshot(args: SnapshotArgs, _ctx: Context) -> CommandResult {
    let config = Config::load().map_err(|e| Error::msg(format!("Failed to load config: {e}")))?;

    let stories_path = config.book.path.join(&config.book.stories);
    snapshot::run(SnapshotConfig {
        book_path: &config.book.path,
        stories_path: &stories_path,
        headless: args.resolve_headless(),
        save: args.resolve_save(),
        mode: args.check,
    })
    .await
    .map_err(Error::msg)
}
