use std::error::Error;
use std::path::PathBuf;
use std::process::{Command, Stdio};

/// Trait for document generators that can produce documentation in various formats
pub trait DocumentationGenerator {
    /// Generate documentation and return the path to the generated output
    fn generate(&self) -> Result<PathBuf, Box<dyn Error>>;

    /// Check if the necessary tools are available for this generator
    fn check_prerequisites(&self) -> Result<(), Box<dyn Error>>;
}

/// Generator for rustdoc JSON documentation
pub struct RustdocGenerator {
    output_format: String,
    // target_crates: Vec<String>,
}

impl RustdocGenerator {
    /// Create a new RustdocGenerator
    pub fn new(output_format: String /* target_crates: Vec<String> */) -> Self {
        Self {
            output_format,
            // target_crates,
        }
    }
}

impl DocumentationGenerator for RustdocGenerator {
    fn generate(&self) -> Result<PathBuf, Box<dyn Error>> {
        println!(
            "Running rustdoc to generate {} documentation...",
            self.output_format
        );

        let rustdoc_output = Command::new("rustup")
            .env("RUSTDOCFLAGS", "-Z unstable-options --output-format=json")
            .env("HOLT_BOOK_RUN_BUILD_SCRIPT", "false")
            .args(["run", "nightly", "cargo", "doc"])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()?;

        if !rustdoc_output.status.success() {
            let stderr = String::from_utf8_lossy(&rustdoc_output.stderr);
            eprintln!("rustdoc command failed: {}", stderr);
            // Continue with the process even if rustdoc fails
        } else {
            println!("rustdoc command completed successfully");
        }

        // Return the path to the generated JSON file
        Ok(PathBuf::from("../../target/doc/holt_ui_book.json"))
    }

    fn check_prerequisites(&self) -> Result<(), Box<dyn Error>> {
        // Check if rustup is available
        let rustup_output = Command::new("rustup").arg("--version").output()?;

        if !rustup_output.status.success() {
            return Err("rustup is not available".into());
        }

        // Check if nightly toolchain is installed
        let nightly_output = Command::new("rustup")
            .args(["toolchain", "list"])
            .output()?;

        if !nightly_output.status.success() {
            return Err("Failed to check for nightly toolchain".into());
        }

        let output = String::from_utf8_lossy(&nightly_output.stdout);
        if !output.contains("nightly") {
            return Err("nightly toolchain is not installed. Install it with 'rustup toolchain install nightly'".into());
        }

        Ok(())
    }
}
