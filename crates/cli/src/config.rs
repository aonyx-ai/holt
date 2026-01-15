//! Configuration loading for holt.toml

use serde::Deserialize;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("holt.toml not found. Create one in your project root.")]
    NotFound,
    #[error("Failed to read holt.toml: {0}")]
    ReadError(#[from] std::io::Error),
    #[error("Failed to parse holt.toml: {0}")]
    ParseError(#[from] toml::de::Error),
}

#[derive(Debug, Deserialize)]
pub struct HoltConfig {
    pub storybook: StorybookConfig,
    #[serde(default)]
    pub visual_test: VisualTestConfig,
}

#[derive(Debug, Deserialize)]
pub struct StorybookConfig {
    /// Path to storybook crate, relative to holt.toml
    #[serde(rename = "crate")]
    pub crate_path: PathBuf,
    /// Port to serve on
    #[serde(default = "default_port")]
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct VisualTestConfig {
    /// Baseline directory, relative to holt.toml root
    #[serde(default = "default_baseline_dir")]
    pub baseline_dir: PathBuf,
}

impl Default for VisualTestConfig {
    fn default() -> Self {
        Self {
            baseline_dir: default_baseline_dir(),
        }
    }
}

fn default_port() -> u16 {
    8080
}

fn default_baseline_dir() -> PathBuf {
    PathBuf::from("tests/visual-baselines")
}

impl HoltConfig {
    /// Find and load holt.toml by walking up from cwd
    pub fn find_and_load() -> Result<(Self, PathBuf), ConfigError> {
        let mut dir = std::env::current_dir().map_err(ConfigError::ReadError)?;
        loop {
            let config_path = dir.join("holt.toml");
            if config_path.exists() {
                let content = std::fs::read_to_string(&config_path)?;
                let config: HoltConfig = toml::from_str(&content)?;
                return Ok((config, dir));
            }
            if !dir.pop() {
                return Err(ConfigError::NotFound);
            }
        }
    }

    /// Get the absolute path to the storybook crate
    pub fn storybook_path(&self, root: &Path) -> PathBuf {
        root.join(&self.storybook.crate_path)
    }

    /// Get the absolute path to the baseline directory
    pub fn baseline_path(&self, root: &Path) -> PathBuf {
        root.join(&self.visual_test.baseline_dir)
    }
}
