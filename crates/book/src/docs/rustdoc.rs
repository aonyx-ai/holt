use std::path::PathBuf;
use std::process::Command;
use thiserror::Error;

/// Custom error type for rustdoc operations
#[derive(Error, Debug)]
pub enum RustdocError {
    #[error("rustup is not available: {source}")]
    RustupNotAvailable { source: std::io::Error },

    #[error(
        "nightly toolchain is not installed. Install it with 'rustup toolchain install nightly'"
    )]
    NightlyToolchainMissing,

    #[error("Failed to check prerequisites: {reason}")]
    PrerequisiteCheckFailed { reason: String },

    #[error("Command execution failed: {source}")]
    CommandExecutionFailed { source: std::io::Error },

    #[error("Command '{command}' failed with exit code {exit_code}: {stderr}")]
    CommandFailed {
        command: String,
        exit_code: i32,
        stderr: String,
    },
}

/// Command output structure
#[derive(Debug, Clone)]
pub struct CommandOutput {
    pub status_code: i32,
    pub stdout: String,
    pub stderr: String,
}

/// Trait for executing commands (allows dependency injection for testing)
pub trait CommandExecutor {
    fn execute(
        &self,
        program: &str,
        args: &[&str],
        env: &[(&str, &str)],
    ) -> Result<CommandOutput, std::io::Error>;
}

/// Real command executor that uses std::process::Command
pub struct RealCommandExecutor;

impl CommandExecutor for RealCommandExecutor {
    fn execute(
        &self,
        program: &str,
        args: &[&str],
        env: &[(&str, &str)],
    ) -> Result<CommandOutput, std::io::Error> {
        let mut cmd = Command::new(program);
        cmd.args(args);

        for (key, value) in env {
            cmd.env(key, value);
        }

        let output = cmd.output()?;

        Ok(CommandOutput {
            status_code: output.status.code().unwrap_or(-1),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        })
    }
}

/// Trait for document generators that can produce documentation in various formats
pub trait DocumentationGenerator {
    /// Generate documentation and return the path to the generated output
    fn generate(&self) -> Result<PathBuf, RustdocError>;

    /// Check if the necessary tools are available for this generator
    fn check_prerequisites(&self) -> Result<(), RustdocError>;
}

/// Generator for rustdoc JSON documentation
pub struct RustdocGenerator<E: CommandExecutor> {
    output_format: String,
    executor: E,
}

impl RustdocGenerator<RealCommandExecutor> {
    /// Create a new RustdocGenerator with real command execution
    pub fn new(output_format: String) -> Self {
        Self {
            output_format,
            executor: RealCommandExecutor,
        }
    }
}

impl<E: CommandExecutor> DocumentationGenerator for RustdocGenerator<E> {
    fn generate(&self) -> Result<PathBuf, RustdocError> {
        println!(
            "Running rustdoc to generate {} documentation...",
            self.output_format
        );

        let env = [
            ("RUSTDOCFLAGS", "-Z unstable-options --output-format=json"),
            ("HOLT_BOOK_RUN_BUILD_SCRIPT", "false"),
        ];

        let output = self
            .executor
            .execute("rustup", &["run", "nightly", "cargo", "doc"], &env)
            .map_err(|source| RustdocError::CommandExecutionFailed { source })?;

        if output.status_code != 0 {
            eprintln!("rustdoc command failed: {}", output.stderr);
            return Err(RustdocError::CommandFailed {
                command: "rustup run nightly cargo doc".to_string(),
                exit_code: output.status_code,
                stderr: output.stderr,
            });
        } else {
            println!("rustdoc command completed successfully");
        }

        // Return the path to the generated JSON file
        let output_path = PathBuf::from("../../target/doc/holt_ui_book.json");
        Ok(output_path)
    }

    fn check_prerequisites(&self) -> Result<(), RustdocError> {
        // Check if rustup is available
        let rustup_output = self
            .executor
            .execute("rustup", &["--version"], &[])
            .map_err(|source| RustdocError::RustupNotAvailable { source })?;

        if rustup_output.status_code != 0 {
            return Err(RustdocError::PrerequisiteCheckFailed {
                reason: format!("rustup version check failed: {}", rustup_output.stderr),
            });
        }

        // Check if nightly toolchain is installed
        let nightly_output = self
            .executor
            .execute("rustup", &["toolchain", "list"], &[])
            .map_err(|source| RustdocError::CommandExecutionFailed { source })?;

        if nightly_output.status_code != 0 {
            return Err(RustdocError::PrerequisiteCheckFailed {
                reason: format!("Failed to list toolchains: {}", nightly_output.stderr),
            });
        }

        if !nightly_output.stdout.contains("nightly") {
            return Err(RustdocError::NightlyToolchainMissing);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Mock command executor for testing
    pub struct MockCommandExecutor {
        responses: std::collections::HashMap<String, CommandOutput>,
    }

    impl<E: CommandExecutor> RustdocGenerator<E> {
        /// Create a new RustdocGenerator with custom command executor (for testing)
        pub fn with_executor(output_format: String, executor: E) -> Self {
            Self {
                output_format,
                executor,
            }
        }
    }

    impl MockCommandExecutor {
        pub fn new() -> Self {
            Self {
                responses: std::collections::HashMap::new(),
            }
        }

        pub fn with_response(mut self, command: &str, output: CommandOutput) -> Self {
            self.responses.insert(command.to_string(), output);
            self
        }

        fn command_key(&self, program: &str, args: &[&str]) -> String {
            format!("{} {}", program, args.join(" "))
        }
    }

    impl CommandExecutor for MockCommandExecutor {
        fn execute(
            &self,
            program: &str,
            args: &[&str],
            _env: &[(&str, &str)],
        ) -> Result<CommandOutput, std::io::Error> {
            let key = self.command_key(program, args);
            self.responses.get(&key).cloned().ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("Command not found: {}", key),
                )
            })
        }
    }

    fn successful_rustup_version() -> CommandOutput {
        CommandOutput {
            status_code: 0,
            stdout: "rustup 1.26.0".to_string(),
            stderr: "".to_string(),
        }
    }

    fn successful_nightly_list() -> CommandOutput {
        CommandOutput {
            status_code: 0,
            stdout: "stable-x86_64-unknown-linux-gnu (default)\nnightly-x86_64-unknown-linux-gnu"
                .to_string(),
            stderr: "".to_string(),
        }
    }

    fn missing_nightly_list() -> CommandOutput {
        CommandOutput {
            status_code: 0,
            stdout: "stable-x86_64-unknown-linux-gnu (default)".to_string(),
            stderr: "".to_string(),
        }
    }

    fn successful_doc_generation() -> CommandOutput {
        CommandOutput {
            status_code: 0,
            stdout: "Documenting holt-ui...".to_string(),
            stderr: "".to_string(),
        }
    }

    fn failed_doc_generation() -> CommandOutput {
        CommandOutput {
            status_code: 1,
            stdout: "".to_string(),
            stderr: "error: could not compile `holt-ui`".to_string(),
        }
    }

    fn failed_command() -> CommandOutput {
        CommandOutput {
            status_code: 1,
            stdout: "".to_string(),
            stderr: "command failed".to_string(),
        }
    }

    #[test]
    fn test_check_prerequisites_success() {
        let executor = MockCommandExecutor::new()
            .with_response("rustup --version", successful_rustup_version())
            .with_response("rustup toolchain list", successful_nightly_list());

        let generator = RustdocGenerator::with_executor("json".to_string(), executor);
        let result = generator.check_prerequisites();

        assert!(result.is_ok());
    }

    #[test]
    fn test_check_prerequisites_rustup_not_available() {
        let executor = MockCommandExecutor::new(); // No responses configured

        let generator = RustdocGenerator::with_executor("json".to_string(), executor);
        let result = generator.check_prerequisites();

        assert!(result.is_err());
        match result.unwrap_err() {
            RustdocError::RustupNotAvailable { source: _ } => {}
            _ => panic!("Expected RustupNotAvailable error"),
        }
    }

    #[test]
    fn test_check_prerequisites_rustup_version_failed() {
        let executor =
            MockCommandExecutor::new().with_response("rustup --version", failed_command());

        let generator = RustdocGenerator::with_executor("json".to_string(), executor);
        let result = generator.check_prerequisites();

        assert!(result.is_err());
        match result.unwrap_err() {
            RustdocError::PrerequisiteCheckFailed { reason } => {
                assert!(reason.contains("rustup version check failed"));
            }
            _ => panic!("Expected PrerequisiteCheckFailed error"),
        }
    }

    #[test]
    fn test_check_prerequisites_nightly_toolchain_missing() {
        let executor = MockCommandExecutor::new()
            .with_response("rustup --version", successful_rustup_version())
            .with_response("rustup toolchain list", missing_nightly_list());

        let generator = RustdocGenerator::with_executor("json".to_string(), executor);
        let result = generator.check_prerequisites();

        assert!(result.is_err());
        match result.unwrap_err() {
            RustdocError::NightlyToolchainMissing => {}
            _ => panic!("Expected NightlyToolchainMissing error"),
        }
    }

    #[test]
    fn test_check_prerequisites_toolchain_list_failed() {
        let executor = MockCommandExecutor::new()
            .with_response("rustup --version", successful_rustup_version())
            .with_response("rustup toolchain list", failed_command());

        let generator = RustdocGenerator::with_executor("json".to_string(), executor);
        let result = generator.check_prerequisites();

        assert!(result.is_err());
        match result.unwrap_err() {
            RustdocError::PrerequisiteCheckFailed { reason } => {
                assert!(reason.contains("Failed to list toolchains"));
            }
            _ => panic!("Expected PrerequisiteCheckFailed error"),
        }
    }

    #[test]
    fn test_generate_success() {
        let executor = MockCommandExecutor::new()
            .with_response("rustup run nightly cargo doc", successful_doc_generation());

        let generator = RustdocGenerator::with_executor("json".to_string(), executor);
        let result = generator.generate();

        assert!(result.is_ok());
        let path = result.unwrap();
        assert_eq!(path, PathBuf::from("../../target/doc/holt_ui_book.json"));
    }

    #[test]
    fn test_generate_command_execution_failed() {
        let executor = MockCommandExecutor::new(); // No responses configured

        let generator = RustdocGenerator::with_executor("json".to_string(), executor);
        let result = generator.generate();

        assert!(result.is_err());
        match result.unwrap_err() {
            RustdocError::CommandExecutionFailed { source: _ } => {}
            _ => panic!("Expected CommandExecutionFailed error"),
        }
    }

    #[test]
    fn test_generate_command_failed() {
        let executor = MockCommandExecutor::new()
            .with_response("rustup run nightly cargo doc", failed_doc_generation());

        let generator = RustdocGenerator::with_executor("json".to_string(), executor);
        let result = generator.generate();

        assert!(result.is_err());
        match result.unwrap_err() {
            RustdocError::CommandFailed {
                command,
                exit_code,
                stderr,
            } => {
                assert_eq!(command, "rustup run nightly cargo doc");
                assert_eq!(exit_code, 1);
                assert!(stderr.contains("could not compile"));
            }
            _ => panic!("Expected CommandFailed error"),
        }
    }

    #[test]
    fn test_error_display() {
        let error = RustdocError::NightlyToolchainMissing;
        assert_eq!(
            error.to_string(),
            "nightly toolchain is not installed. Install it with 'rustup toolchain install nightly'"
        );

        let error = RustdocError::CommandFailed {
            command: "cargo doc".to_string(),
            exit_code: 1,
            stderr: "compilation failed".to_string(),
        };
        assert_eq!(
            error.to_string(),
            "Command 'cargo doc' failed with exit code 1: compilation failed"
        );

        let error = RustdocError::PrerequisiteCheckFailed {
            reason: "test reason".to_string(),
        };
        assert_eq!(
            error.to_string(),
            "Failed to check prerequisites: test reason"
        );
    }

    #[test]
    fn test_real_command_executor_integration() {
        let executor = RealCommandExecutor;

        // Test a simple command that should exist on most systems
        let result = executor.execute("echo", &["hello"], &[]);
        assert!(result.is_ok());

        let output = result.unwrap();
        assert_eq!(output.status_code, 0);
        assert!(output.stdout.contains("hello"));
    }

    #[test]
    fn test_mock_command_executor() {
        let executor = MockCommandExecutor::new().with_response(
            "test command",
            CommandOutput {
                status_code: 42,
                stdout: "test output".to_string(),
                stderr: "test error".to_string(),
            },
        );

        let result = executor.execute("test", &["command"], &[]);
        assert!(result.is_ok());

        let output = result.unwrap();
        assert_eq!(output.status_code, 42);
        assert_eq!(output.stdout, "test output");
        assert_eq!(output.stderr, "test error");
    }
}
