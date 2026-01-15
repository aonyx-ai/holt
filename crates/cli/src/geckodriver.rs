//! Geckodriver process management

use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::Duration;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GeckoDriverError {
    #[error("Failed to start geckodriver: {0}")]
    StartFailed(#[from] std::io::Error),
}

/// Manages the geckodriver process
pub struct GeckoDriver {
    process: Child,
}

impl GeckoDriver {
    /// Start geckodriver on port 4444
    pub fn start() -> Result<Self, GeckoDriverError> {
        println!("Starting geckodriver...");

        // In CI, show geckodriver output for debugging
        let is_ci = std::env::var("CI").is_ok();
        let mut cmd = Command::new("geckodriver");
        cmd.args(["--port", "4444"]);

        if !is_ci {
            cmd.stdout(Stdio::null()).stderr(Stdio::null());
        }

        let process = cmd.spawn()?;

        // Wait for geckodriver to be ready
        thread::sleep(Duration::from_secs(2));

        Ok(GeckoDriver { process })
    }
}

impl Drop for GeckoDriver {
    fn drop(&mut self) {
        println!("Shutting down geckodriver...");
        let _ = self.process.kill();
        let _ = self.process.wait();
    }
}
