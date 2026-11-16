use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

use crate::config::BuildConfig;
use crate::generator::CodeGenerator;
use crate::parser::StoryParser;

static KIT_DIR: &str = "../kit/";

type BuildResult<T> = Result<T, BuildError>;

/// Errors that can occur during the build process
#[derive(Debug, Error)]
pub(crate) enum BuildError {
    #[error("failed to read or write a file")]
    Io {
        #[from]
        source: std::io::Error,
    },
    #[error("failed to read an environment variable")]
    Env {
        #[from]
        source: env::VarError,
    },
    #[error("invalid story file: {path}")]
    InvalidStory { path: String },
    #[error("component not found: {component}")]
    ComponentNotFound { component: String },
}

fn main() -> BuildResult<()> {
    setup_cargo_rerun_conditions();

    let config = BuildConfig::new()?;
    let stories = StoryParser::new(&config).parse_stories()?;
    let generator = CodeGenerator::new(&config);

    generator.generate_story_files(&stories)?;

    Ok(())
}

fn setup_cargo_rerun_conditions() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/stories/");
    println!(
        "cargo:rerun-if-changed={}",
        PathBuf::from(KIT_DIR).join("src/visual/").display()
    );
}

mod config {
    use super::*;

    /// Configuration for the build process, including directory paths
    #[derive(Debug, getset::Getters)]
    pub struct BuildConfig {
        #[getset(get = "pub")]
        stories_dir: PathBuf,
        #[getset(get = "pub")]
        stories_output_dir: PathBuf,
        #[getset(get = "pub")]
        ui_components_dir: PathBuf,
    }

    impl BuildConfig {
        /// Creates a new build configuration from environment variables
        pub fn new() -> BuildResult<Self> {
            let stories_output_dir = PathBuf::from(env::var("OUT_DIR")?).join("stories");

            Ok(Self {
                stories_dir: PathBuf::from("src/stories"),
                stories_output_dir,
                ui_components_dir: PathBuf::from(KIT_DIR).join("src/visual"),
            })
        }
    }
}

mod data {
    /// Information about a UI component extracted from story files
    #[derive(Clone, Debug, getset::Getters)]
    pub struct ComponentInfo {
        #[getset(get = "pub")]
        source: String,
    }

    impl ComponentInfo {
        pub fn new(source: String) -> Self {
            Self { source }
        }
    }

    #[derive(Debug, bon::Builder, getset::Getters)]
    pub struct ParsedStory {
        #[getset(get = "pub")]
        name: String,
        #[getset(get = "pub")]
        info: ComponentInfo,
    }
}

mod imports {

    /// Extracts the component name from a helper comment in the format `// @component ComponentName`.
    /// If found, returns Some(ComponentName). Otherwise returns None
    pub fn extract_component_from_comment(content: &str) -> Option<String> {
        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("// @component ") {
                let component_name = line.strip_prefix("// @component ")?.trim();
                if !component_name.is_empty() {
                    return Some(component_name.to_string());
                }
            }
        }
        None
    }
}

mod parser {
    use super::*;

    /// Parses story files to extract component information
    pub struct StoryParser<'a> {
        config: &'a config::BuildConfig,
    }

    impl<'a> StoryParser<'a> {
        /// Creates a new story parser with the given configuration
        pub fn new(config: &'a config::BuildConfig) -> Self {
            Self { config }
        }

        /// Parses all story files in the configured directory.
        /// Returns a map of story names to their component information
        pub fn parse_stories(&self) -> BuildResult<HashMap<String, data::ComponentInfo>> {
            let mut stories = HashMap::new();

            let entries = fs::read_dir(self.config.stories_dir())?;

            for entry in entries {
                let entry = entry?;
                let path = entry.path();

                if let Some(story) = self.try_parse_story_file(&path)? {
                    stories.insert(story.name().clone(), story.info().clone());
                }
            }

            Ok(stories)
        }

        /// Parses one story file
        ///
        /// A file without an `@component` comment is skipped with a warning.
        fn try_parse_story_file(&self, path: &Path) -> BuildResult<Option<data::ParsedStory>> {
            if !is_valid_story_file(path) {
                return Ok(None);
            }

            let story_name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .ok_or_else(|| BuildError::InvalidStory {
                    path: path.display().to_string(),
                })?
                .to_string();

            let story_contents = fs::read_to_string(path)?;

            let component_name = if let Some(comment_component) =
                imports::extract_component_from_comment(&story_contents)
            {
                comment_component
            } else {
                println!(
                    "cargo:warning=No @component comment found in story '{story_name}'. Skipping file."
                );
                return Ok(None);
            };

            let component_path = self
                .config
                .ui_components_dir()
                .join(format!("{}.rs", component_name.to_lowercase()));

            if !component_path.exists() {
                return Err(BuildError::ComponentNotFound {
                    component: component_name.clone(),
                });
            }

            let component_source = fs::read_to_string(&component_path)?;

            let info = data::ComponentInfo::new(component_source);

            Ok(Some(
                data::ParsedStory::builder()
                    .name(story_name)
                    .info(info)
                    .build(),
            ))
        }
    }

    /// Returns whether the path is a story file
    ///
    /// Only `.rs` files are story files, and `mod.rs` is excluded.
    pub fn is_valid_story_file(path: &Path) -> bool {
        if path.extension().is_none_or(|ext| ext != "rs") {
            return false;
        }

        let file_name = match path.file_stem().and_then(|s| s.to_str()) {
            Some(name) => name,
            None => return false,
        };

        file_name != "mod"
    }
}

mod generator {
    use super::*;

    /// Generates source code files from parsed component information
    pub struct CodeGenerator<'a> {
        config: &'a config::BuildConfig,
    }

    impl<'a> CodeGenerator<'a> {
        /// Creates a new code generator with the given configuration
        pub fn new(config: &'a config::BuildConfig) -> Self {
            Self { config }
        }

        /// Generates source code files for all stories.
        /// Creates the output directory if it doesn't exist
        pub fn generate_story_files(
            &self,
            stories: &HashMap<String, data::ComponentInfo>,
        ) -> BuildResult<()> {
            fs::create_dir_all(self.config.stories_output_dir())?;

            for (story_name, component_info) in stories {
                self.generate_story_file(story_name, component_info)?;
            }

            Ok(())
        }

        fn generate_story_file(
            &self,
            story_name: &str,
            info: &data::ComponentInfo,
        ) -> BuildResult<()> {
            let file_path = self
                .config
                .stories_output_dir()
                .join(format!("{story_name}_source.rs"));
            let content = build_file_content(story_name, info);

            fs::write(&file_path, content)?;
            Ok(())
        }
    }

    pub fn build_file_content(story_name: &str, info: &data::ComponentInfo) -> String {
        let const_name = format!("{}_SOURCE", story_name.to_uppercase());

        format!(
            "// Auto-generated source code for {} component\n\n\
             const {}: &str = r###\"\n```rust\n{}\n```\n\"###;\n\n",
            story_name,
            const_name,
            info.source()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_config_is_send_sync_and_unpin() {
        fn assert_send_sync_and_unpin<T: Send + Sync + Unpin>() {}

        assert_send_sync_and_unpin::<BuildConfig>();
    }

    #[test]
    fn build_error_is_send_sync_and_unpin() {
        fn assert_send_sync_and_unpin<T: Send + Sync + Unpin>() {}

        assert_send_sync_and_unpin::<BuildError>();
    }

    #[test]
    fn code_generator_is_send_sync_and_unpin() {
        fn assert_send_sync_and_unpin<T: Send + Sync + Unpin>() {}

        assert_send_sync_and_unpin::<CodeGenerator<'static>>();
    }

    #[test]
    fn component_info_is_send_sync_and_unpin() {
        fn assert_send_sync_and_unpin<T: Send + Sync + Unpin>() {}

        assert_send_sync_and_unpin::<data::ComponentInfo>();
    }

    #[test]
    fn parsed_story_is_send_sync_and_unpin() {
        fn assert_send_sync_and_unpin<T: Send + Sync + Unpin>() {}

        assert_send_sync_and_unpin::<data::ParsedStory>();
    }

    #[test]
    fn story_parser_is_send_sync_and_unpin() {
        fn assert_send_sync_and_unpin<T: Send + Sync + Unpin>() {}

        assert_send_sync_and_unpin::<StoryParser<'static>>();
    }
}
