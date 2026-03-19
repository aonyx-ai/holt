use clawless::clap;
use serde::Deserialize;
use std::path::{Path, PathBuf};

/// Whether to automatically open the browser
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, clap::ValueEnum)]
pub enum OpenBrowser {
    #[default]
    No,
    Yes,
}

impl<'de> Deserialize<'de> for OpenBrowser {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <bool as Deserialize>::deserialize(deserializer)?;
        match value {
            true => Ok(Self::Yes),
            false => Ok(Self::No),
        }
    }
}

#[derive(Debug, Default, Deserialize)]
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
    #[serde(default = "default_stories_path")]
    pub stories: PathBuf,
}

#[derive(Debug, Deserialize)]
pub struct ServeConfig {
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default)]
    pub open: OpenBrowser,
}

fn default_book_path() -> PathBuf {
    PathBuf::from(".")
}

fn default_stories_path() -> PathBuf {
    PathBuf::from("src/stories")
}

fn default_port() -> u16 {
    8080
}

impl Default for BookConfig {
    fn default() -> Self {
        Self {
            path: default_book_path(),
            stories: default_stories_path(),
        }
    }
}

impl Default for ServeConfig {
    fn default() -> Self {
        Self {
            port: default_port(),
            open: OpenBrowser::No,
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
