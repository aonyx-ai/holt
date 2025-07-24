use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

use crate::config::BuildConfig;
use crate::generator::CodeGenerator;
use crate::parser::StoryParser;

type BuildResult<T> = Result<T, BuildError>;

/// Errors that can occur during the build process.
#[derive(Error, Debug)]
pub enum BuildError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Environment variable error: {0}")]
    EnvError(#[from] env::VarError),
    #[error("Invalid story file: {0}")]
    InvalidStory(String),
    #[error("Component not found: {0}")]
    ComponentNotFound(String),
    #[error("No valid component found in story: {0}")]
    NoValidComponent(String),
}

/// Main build script entry point.
/// Parses story files and generates source code for each component.
fn main() -> BuildResult<()> {
    setup_cargo_rerun_conditions();

    let config = BuildConfig::new()?;
    let stories = StoryParser::new(&config).parse_stories()?;
    let generator = CodeGenerator::new(&config);

    generator.generate_story_files(&stories)?;

    Ok(())
}

fn setup_cargo_rerun_conditions() {
    println!("cargo:rerun-if-changed=src/stories/");
    println!("cargo:rerun-if-changed=../ui/src/visual/");
}

mod config {
    use super::*;

    /// Configuration for the build process, including directory paths.
    #[derive(Debug)]
    pub struct BuildConfig {
        pub stories_dir: PathBuf,
        pub stories_output_dir: PathBuf,
        pub ui_components_dir: PathBuf,
    }

    impl BuildConfig {
        /// Creates a new build configuration from environment variables.
        pub fn new() -> BuildResult<Self> {
            let out_dir = PathBuf::from(env::var("OUT_DIR")?);
            let stories_output_dir = out_dir.join("stories");

            Ok(Self {
                stories_dir: PathBuf::from("src/stories"),
                stories_output_dir,
                ui_components_dir: PathBuf::from("../ui/src/visual"),
            })
        }
    }
}

mod data {
    /// Information about a UI component extracted from story files.
    #[derive(Debug, Clone)]
    pub struct ComponentInfo {
        pub source: String,
    }

    impl ComponentInfo {
        pub fn new(source: String) -> Self {
            Self { source }
        }
    }

    #[derive(Debug)]
    pub struct ParsedStory {
        pub name: String,
        pub info: ComponentInfo,
    }
}

mod imports {
    use super::*;

    /// Extracts component names from holt_ui::visual imports in the given content.
    /// Returns a sorted, deduplicated list of main component names (excluding variants).
    pub fn extract_holt_ui_imports(content: &str) -> Vec<String> {
        let mut components: Vec<String> = content
            .lines()
            .filter_map(|line| parse_import_line(line.trim()))
            .flatten()
            .filter(|import| is_main_component(import))
            .collect::<HashSet<_>>() // Dedup
            .into_iter()
            .collect();

        components.sort();
        components
    }

    pub fn parse_import_line(line: &str) -> Option<Vec<String>> {
        if !line.starts_with("use holt_ui::visual::") {
            return None;
        }

        let imports_part = line.strip_prefix("use holt_ui::visual::")?;

        if let Some(imports) = extract_braced_imports(imports_part) {
            Some(imports)
        } else {
            extract_single_import(imports_part).map(|imp| vec![imp])
        }
    }

    pub fn extract_braced_imports(imports_part: &str) -> Option<Vec<String>> {
        if !imports_part.starts_with('{') || !imports_part.contains('}') {
            return None;
        }

        let start = imports_part.find('{')?;
        let end = imports_part.find('}')?;
        let imports = &imports_part[start + 1..end];

        Some(
            imports
                .split(',')
                .map(|import| import.trim().to_string())
                .filter(|import| !import.is_empty())
                .collect(),
        )
    }

    pub fn extract_single_import(imports_part: &str) -> Option<String> {
        let import = imports_part.trim_end_matches(';').trim();
        if import.is_empty() {
            None
        } else {
            Some(import.to_string())
        }
    }

    pub fn is_main_component(import: &str) -> bool {
        !import.contains("Variant") && !import.contains("Size") && !import.is_empty()
    }
}

mod parser {
    use super::*;

    /// Parses story files to extract component information.
    pub struct StoryParser<'a> {
        config: &'a config::BuildConfig,
    }

    impl<'a> StoryParser<'a> {
        /// Creates a new story parser with the given configuration.
        pub fn new(config: &'a config::BuildConfig) -> Self {
            Self { config }
        }

        /// Parses all story files in the configured directory.
        /// Returns a map of story names to their component information.
        pub fn parse_stories(&self) -> BuildResult<HashMap<String, data::ComponentInfo>> {
            let mut stories = HashMap::new();

            let entries = fs::read_dir(&self.config.stories_dir)?;

            for entry in entries {
                let entry = entry?;
                let path = entry.path();

                if let Some(story) = self.try_parse_story_file(&path)? {
                    stories.insert(story.name, story.info);
                }
            }

            Ok(stories)
        }

        fn try_parse_story_file(&self, path: &Path) -> BuildResult<Option<data::ParsedStory>> {
            if !is_valid_story_file(path) {
                return Ok(None);
            }

            let story_name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .ok_or_else(|| BuildError::InvalidStory(path.display().to_string()))?
                .to_string();

            let story_contents = fs::read_to_string(path)?;
            let components = imports::extract_holt_ui_imports(&story_contents);

            if components.len() != 1 {
                return Ok(None);
            }

            let component_name = components.first().unwrap();
            let component_path = self
                .config
                .ui_components_dir
                .join(format!("{}.rs", component_name.to_lowercase()));

            if !component_path.exists() {
                return Err(BuildError::ComponentNotFound(component_name.clone()));
            }

            let component_source = fs::read_to_string(&component_path)?;

            let info = data::ComponentInfo::new(component_source);

            Ok(Some(data::ParsedStory {
                name: story_name,
                info,
            }))
        }
    }

    pub fn is_valid_story_file(path: &Path) -> bool {
        // Only process .rs files, excluding mod.rs
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

    /// Generates source code files from parsed component information.
    pub struct CodeGenerator<'a> {
        config: &'a config::BuildConfig,
    }

    impl<'a> CodeGenerator<'a> {
        /// Creates a new code generator with the given configuration.
        pub fn new(config: &'a config::BuildConfig) -> Self {
            Self { config }
        }

        /// Generates source code files for all stories.
        /// Creates the output directory if it doesn't exist.
        pub fn generate_story_files(
            &self,
            stories: &HashMap<String, data::ComponentInfo>,
        ) -> BuildResult<()> {
            fs::create_dir_all(&self.config.stories_output_dir)?;

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
                .stories_output_dir
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
            story_name, const_name, info.source
        )
    }
}
