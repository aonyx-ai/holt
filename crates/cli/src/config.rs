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

#[derive(Debug, Default, Deserialize, getset::Getters)]
pub struct Config {
    #[serde(default)]
    #[getset(get = "pub")]
    book: BookConfig,
    #[serde(default)]
    #[getset(get = "pub")]
    serve: ServeConfig,
}

#[derive(Debug, Deserialize, getset::Getters)]
pub struct BookConfig {
    #[serde(default = "default_book_path")]
    #[getset(get = "pub")]
    path: PathBuf,
    #[serde(default = "default_stories_path")]
    #[getset(get = "pub")]
    stories: PathBuf,
}

#[derive(Debug, Deserialize, getset::CopyGetters)]
pub struct ServeConfig {
    #[serde(default = "default_port")]
    #[getset(get_copy = "pub")]
    port: u16,
    #[serde(default)]
    #[getset(get_copy = "pub")]
    open: OpenBrowser,
}

/// Errors that can occur while loading the configuration file
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    /// Failed to read the configuration file from disk
    #[error("failed to read {}", path.display())]
    Read {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    /// Failed to parse the configuration file as TOML
    #[error("failed to parse {}", path.display())]
    Parse {
        path: PathBuf,
        #[source]
        source: toml::de::Error,
    },
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
    pub fn load() -> Result<Self, ConfigError> {
        let path = Path::new("holt.toml");
        if path.exists() {
            let content = std::fs::read_to_string(path).map_err(|source| ConfigError::Read {
                path: path.to_path_buf(),
                source,
            })?;
            toml::from_str(&content).map_err(|source| ConfigError::Parse {
                path: path.to_path_buf(),
                source,
            })
        } else {
            Ok(Config::default())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn book_config_is_send_sync_and_unpin() {
        fn assert_send_sync_and_unpin<T: Send + Sync + Unpin>() {}

        assert_send_sync_and_unpin::<BookConfig>();
    }

    #[test]
    fn config_error_is_send_sync_and_unpin() {
        fn assert_send_sync_and_unpin<T: Send + Sync + Unpin>() {}

        assert_send_sync_and_unpin::<ConfigError>();
    }

    #[test]
    fn config_is_send_sync_and_unpin() {
        fn assert_send_sync_and_unpin<T: Send + Sync + Unpin>() {}

        assert_send_sync_and_unpin::<Config>();
    }

    #[test]
    fn open_browser_is_send_sync_and_unpin() {
        fn assert_send_sync_and_unpin<T: Send + Sync + Unpin>() {}

        assert_send_sync_and_unpin::<OpenBrowser>();
    }

    #[test]
    fn serve_config_is_send_sync_and_unpin() {
        fn assert_send_sync_and_unpin<T: Send + Sync + Unpin>() {}

        assert_send_sync_and_unpin::<ServeConfig>();
    }
}
