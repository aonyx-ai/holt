use serde::Deserialize;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub book: BookConfig,
    #[serde(default)]
    pub serve: ServeConfig,
}

#[derive(Debug, Deserialize)]
pub struct BookConfig {
    #[serde(default = "default_book_path")]
    pub path: PathBuf,
}

#[derive(Debug, Deserialize)]
pub struct ServeConfig {
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default)]
    pub open: bool,
}

fn default_book_path() -> PathBuf {
    PathBuf::from(".")
}

fn default_port() -> u16 {
    8080
}

impl Default for Config {
    fn default() -> Self {
        Self {
            book: BookConfig::default(),
            serve: ServeConfig::default(),
        }
    }
}

impl Default for BookConfig {
    fn default() -> Self {
        Self {
            path: default_book_path(),
        }
    }
}

impl Default for ServeConfig {
    fn default() -> Self {
        Self {
            port: default_port(),
            open: false,
        }
    }
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let path = Path::new("holt.toml");
        if path.exists() {
            let content = std::fs::read_to_string(path)?;
            Ok(toml::from_str(&content)?)
        } else {
            Ok(Config::default())
        }
    }
}
