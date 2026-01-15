//! Trunk server management for the storybook

use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::time::Duration;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorybookError {
    #[error("Failed to build storybook: trunk build failed")]
    BuildFailed,
    #[error("Failed to start storybook server: {0}")]
    StartFailed(#[from] std::io::Error),
    #[error("Storybook server failed to become ready within 30 seconds")]
    Timeout,
}

/// Manages the trunk serve process for the storybook
pub struct Storybook {
    process: Child,
    port: u16,
}

impl Storybook {
    /// Start the storybook server
    ///
    /// This will:
    /// 1. Build the WASM app first (to avoid timeout during serve)
    /// 2. Start trunk serve with auto-reload disabled
    /// 3. Wait for the server to be ready
    pub fn start(crate_path: &Path, port: u16) -> Result<Self, StorybookError> {
        // Build first (avoids timeout during serve)
        println!("Building storybook...");
        let status = Command::new("trunk")
            .args(["build"])
            .current_dir(crate_path)
            .status()?;

        if !status.success() {
            return Err(StorybookError::BuildFailed);
        }

        println!("Starting trunk server on port {}...", port);

        // Start trunk serve with auto-reload disabled to prevent rebuilds during tests
        let process = Command::new("trunk")
            .args([
                "serve",
                "--port",
                &port.to_string(),
                "--no-autoreload",
                "true",
            ])
            .current_dir(crate_path)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;

        let storybook = Storybook { process, port };
        storybook.wait_ready()?;

        println!("Storybook ready at {}", storybook.url());
        Ok(storybook)
    }

    /// Get the URL of the running storybook
    pub fn url(&self) -> String {
        format!("http://localhost:{}", self.port)
    }

    /// Wait for the server to be ready (respond to HTTP requests)
    fn wait_ready(&self) -> Result<(), StorybookError> {
        for i in 0..30 {
            std::thread::sleep(Duration::from_secs(1));
            if ureq::get(&self.url()).call().is_ok() {
                return Ok(());
            }
            if i % 5 == 0 && i > 0 {
                println!("Waiting for server... ({}/30)", i);
            }
        }
        Err(StorybookError::Timeout)
    }
}

impl Drop for Storybook {
    fn drop(&mut self) {
        println!("Shutting down storybook server...");
        let _ = self.process.kill();
        let _ = self.process.wait();
    }
}
