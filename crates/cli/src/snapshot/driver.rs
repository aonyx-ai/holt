//! Process management for geckodriver and trunk server.

use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::Duration;

/// Manages the geckodriver process.
pub struct GeckoDriver {
    process: Child,
}

impl GeckoDriver {
    pub fn start() -> Result<Self, Box<dyn std::error::Error>> {
        println!("Starting geckodriver...");

        let is_ci = std::env::var("CI").is_ok();
        let mut cmd = Command::new("geckodriver");
        cmd.args(["--port", "4444"]);

        if !is_ci {
            cmd.stdout(Stdio::null()).stderr(Stdio::null());
        }

        let process = cmd.spawn()?;
        thread::sleep(Duration::from_secs(2));

        Ok(GeckoDriver { process })
    }
}

impl Drop for GeckoDriver {
    fn drop(&mut self) {
        println!("Shutting down geckodriver...");
        let _ = self.process.kill();
    }
}

/// Manages the trunk serve process.
pub struct TrunkServer {
    process: Child,
    server_url: String,
}

impl TrunkServer {
    pub fn start(book_path: &Path, port: u16) -> Result<Self, Box<dyn std::error::Error>> {
        let server_url = format!("http://localhost:{}", port);

        println!("Pre-building WASM app...");
        let build_status = Command::new("trunk")
            .args(["build"])
            .current_dir(book_path)
            .status()?;

        if !build_status.success() {
            return Err("Failed to build WASM app".into());
        }

        println!("Starting trunk server...");

        let is_ci = std::env::var("CI").is_ok();
        let mut cmd = Command::new("trunk");
        cmd.args([
            "serve",
            "--port",
            &port.to_string(),
            "--no-autoreload",
            "true",
        ]);
        cmd.current_dir(book_path);

        if !is_ci {
            cmd.stdout(Stdio::null()).stderr(Stdio::null());
        }

        let mut process = cmd.spawn()?;

        for i in 0..30 {
            thread::sleep(Duration::from_secs(1));
            if let Ok(response) = ureq::get(&server_url).call()
                && response.status() == 200
            {
                println!("Server is ready!");
                return Ok(TrunkServer {
                    process,
                    server_url,
                });
            }
            if i % 5 == 0 {
                println!("Waiting for server to start... ({}/30)", i);
            }
        }

        println!("Timeout reached, killing trunk server...");
        process.kill().expect("couldn't kill trunk server");

        Err("Server failed to start within 30 seconds".into())
    }

    pub fn url(&self) -> &str {
        &self.server_url
    }
}

impl Drop for TrunkServer {
    fn drop(&mut self) {
        println!("Shutting down trunk server...");
        let _ = self.process.kill();
    }
}
