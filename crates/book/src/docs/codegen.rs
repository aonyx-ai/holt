use proc_macro2::TokenStream;
use quote::quote;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use thiserror::Error;

use crate::docs::extractor::StoryMetadata;

/// Custom error type for codegen operations
#[derive(Error, Debug)]
pub enum CodegenError {
    #[error("Failed to create file {path}: {source}")]
    FileCreationFailed {
        path: String,
        source: std::io::Error,
    },

    #[error("Failed to write to file {path}: {source}")]
    FileWriteFailed {
        path: String,
        source: std::io::Error,
    },

    #[error("Failed to generate tokens: {reason}")]
    TokenGenerationFailed { reason: String },

    #[error("Failed to parse generated tokens: {source}")]
    TokenParsingFailed { source: syn::Error },

    #[error("Invalid story data: {reason}")]
    InvalidStoryData { reason: String },

    #[error("No stories provided for code generation")]
    EmptyStoryList,
}

/// Trait for code generators that produce Rust code
pub trait CodeGenerator<T> {
    /// Generate code from the given input and write it to the specified output path
    fn generate(&self, input: Vec<T>, output_path: &Path) -> Result<(), CodegenError>;

    /// Ensure that the output file exists
    fn ensure_file_exists(&self, output_path: &Path) -> Result<(), CodegenError>;
}

/// Generator for PHF map of story documentation
pub struct PhfMapGenerator;

impl CodeGenerator<StoryMetadata> for PhfMapGenerator {
    fn generate(
        &self,
        stories: Vec<StoryMetadata>,
        output_path: &Path,
    ) -> Result<(), CodegenError> {
        if stories.is_empty() {
            return Err(CodegenError::EmptyStoryList);
        }

        let f = File::create(output_path).map_err(|source| CodegenError::FileCreationFailed {
            path: output_path.display().to_string(),
            source,
        })?;

        let mut codegen = phf_codegen::Map::new();
        codegen.phf_path("holt_book");

        for story in stories {
            let name = story.name;
            let docs = story.docs;

            // Validate story data
            if name.is_empty() {
                return Err(CodegenError::InvalidStoryData {
                    reason: "Story name cannot be empty".to_string(),
                });
            }

            // Use quote to handle the docs string correctly
            let docs_quoted = quote! { #docs }.to_string();
            codegen.entry(name, &docs_quoted);
        }

        let map_tokens: TokenStream = codegen.build().to_string().parse().map_err(|e| {
            CodegenError::TokenGenerationFailed {
                reason: format!("Failed to parse PHF map tokens: {}", e),
            }
        })?;

        // Use quote macro for generating the full static definition
        let tokens = quote! {
            pub static STORY_DOCS: holt_book::Map<&'static str, &'static str> = #map_tokens;
        };

        let mut f = BufWriter::new(f);
        let parse_file = syn::parse_file(&tokens.to_string())
            .map_err(|source| CodegenError::TokenParsingFailed { source })?;
        let buf = prettyplease::unparse(&parse_file);
        f.write_all(buf.as_bytes())
            .map_err(|source| CodegenError::FileWriteFailed {
                path: output_path.display().to_string(),
                source,
            })?;

        println!("Generated code at: {}", output_path.display());
        Ok(())
    }

    fn ensure_file_exists(&self, output_path: &Path) -> Result<(), CodegenError> {
        File::create(output_path).map_err(|source| CodegenError::FileCreationFailed {
            path: output_path.display().to_string(),
            source,
        })?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    fn create_test_stories() -> Vec<StoryMetadata> {
        vec![
            StoryMetadata {
                name: "ButtonStory".to_string(),
                docs: "A button component story".to_string(),
            },
            StoryMetadata {
                name: "InputStory".to_string(),
                docs: "An input component story".to_string(),
            },
        ]
    }

    fn create_single_story() -> Vec<StoryMetadata> {
        vec![StoryMetadata {
            name: "ButtonStory".to_string(),
            docs: "A button component story".to_string(),
        }]
    }

    fn create_stories_with_special_chars() -> Vec<StoryMetadata> {
        vec![
            StoryMetadata {
                name: "QuoteStory".to_string(),
                docs: r#"A story with "quotes" and 'single quotes'"#.to_string(),
            },
            StoryMetadata {
                name: "NewlineStory".to_string(),
                docs: "A story with\nnewlines\nand\ttabs".to_string(),
            },
            StoryMetadata {
                name: "UnicodeStory".to_string(),
                docs: "A story with unicode: 🚀 ñ ü ® ©".to_string(),
            },
        ]
    }

    fn create_story_with_empty_docs() -> Vec<StoryMetadata> {
        vec![StoryMetadata {
            name: "EmptyDocsStory".to_string(),
            docs: "".to_string(),
        }]
    }

    fn create_very_long_story() -> Vec<StoryMetadata> {
        vec![StoryMetadata {
            name: "VeryLongStory".to_string(),
            docs: "A".repeat(10000),
        }]
    }

    #[test]
    fn test_generate_single_story() {
        let stories = create_single_story();
        let temp_file = NamedTempFile::new().unwrap();
        let generator = PhfMapGenerator;

        let result = generator.generate(stories, temp_file.path());
        assert!(result.is_ok());

        // Verify the file was created and contains expected content
        let content = std::fs::read_to_string(temp_file.path()).unwrap();
        assert!(content.contains("STORY_DOCS"));
        assert!(content.contains("ButtonStory"));
        assert!(content.contains("A button component story"));
    }

    #[test]
    fn test_generate_multiple_stories() {
        let stories = create_test_stories();
        let temp_file = NamedTempFile::new().unwrap();
        let generator = PhfMapGenerator;

        let result = generator.generate(stories, temp_file.path());
        assert!(result.is_ok());

        let content = std::fs::read_to_string(temp_file.path()).unwrap();
        assert!(content.contains("STORY_DOCS"));
        assert!(content.contains("ButtonStory"));
        assert!(content.contains("InputStory"));
        assert!(content.contains("A button component story"));
        assert!(content.contains("An input component story"));
    }

    #[test]
    fn test_generate_empty_story_list() {
        let stories = vec![];
        let temp_file = NamedTempFile::new().unwrap();
        let generator = PhfMapGenerator;

        let result = generator.generate(stories, temp_file.path());
        assert!(result.is_err());
        match result.unwrap_err() {
            CodegenError::EmptyStoryList => {}
            _ => panic!("Expected EmptyStoryList error"),
        }
    }

    #[test]
    fn test_generate_story_with_empty_name() {
        let stories = vec![StoryMetadata {
            name: "".to_string(),
            docs: "Some docs".to_string(),
        }];
        let temp_file = NamedTempFile::new().unwrap();
        let generator = PhfMapGenerator;

        let result = generator.generate(stories, temp_file.path());
        assert!(result.is_err());
        match result.unwrap_err() {
            CodegenError::InvalidStoryData { reason } => {
                assert!(reason.contains("Story name cannot be empty"));
            }
            _ => panic!("Expected InvalidStoryData error"),
        }
    }

    #[test]
    fn test_generate_stories_with_special_characters() {
        let stories = create_stories_with_special_chars();
        let temp_file = NamedTempFile::new().unwrap();
        let generator = PhfMapGenerator;

        let result = generator.generate(stories, temp_file.path());
        assert!(result.is_ok());

        let content = std::fs::read_to_string(temp_file.path()).unwrap();
        assert!(content.contains("QuoteStory"));
        assert!(content.contains("NewlineStory"));
        assert!(content.contains("UnicodeStory"));
    }

    #[test]
    fn test_generate_story_with_empty_docs() {
        let stories = create_story_with_empty_docs();
        let temp_file = NamedTempFile::new().unwrap();
        let generator = PhfMapGenerator;

        let result = generator.generate(stories, temp_file.path());
        assert!(result.is_ok());

        let content = std::fs::read_to_string(temp_file.path()).unwrap();
        assert!(content.contains("EmptyDocsStory"));
    }

    #[test]
    fn test_generate_very_long_story() {
        let stories = create_very_long_story();
        let temp_file = NamedTempFile::new().unwrap();
        let generator = PhfMapGenerator;

        let result = generator.generate(stories, temp_file.path());
        assert!(result.is_ok());

        let content = std::fs::read_to_string(temp_file.path()).unwrap();
        assert!(content.contains("VeryLongStory"));
    }

    #[test]
    fn test_generate_invalid_output_path() {
        let stories = create_single_story();
        let generator = PhfMapGenerator;

        // Try to write to a directory that doesn't exist
        let result = generator.generate(stories, Path::new("/nonexistent/path/file.rs"));
        assert!(result.is_err());
        match result.unwrap_err() {
            CodegenError::FileCreationFailed { path, source: _ } => {
                assert!(path.contains("/nonexistent/path/file.rs"));
            }
            _ => panic!("Expected FileCreationFailed error"),
        }
    }

    #[test]
    fn test_ensure_file_exists_success() {
        let temp_file = NamedTempFile::new().unwrap();
        let generator = PhfMapGenerator;

        let result = generator.ensure_file_exists(temp_file.path());
        assert!(result.is_ok());
        assert!(temp_file.path().exists());
    }

    #[test]
    fn test_ensure_file_exists_invalid_path() {
        let generator = PhfMapGenerator;

        let result = generator.ensure_file_exists(Path::new("/nonexistent/path/file.rs"));
        assert!(result.is_err());
        match result.unwrap_err() {
            CodegenError::FileCreationFailed { path, source: _ } => {
                assert!(path.contains("/nonexistent/path/file.rs"));
            }
            _ => panic!("Expected FileCreationFailed error"),
        }
    }

    #[test]
    fn test_generated_code_is_valid_rust() {
        let stories = create_test_stories();
        let temp_file = NamedTempFile::new().unwrap();
        let generator = PhfMapGenerator;

        let result = generator.generate(stories, temp_file.path());
        assert!(result.is_ok());

        // Verify the generated code can be parsed as valid Rust
        let content = std::fs::read_to_string(temp_file.path()).unwrap();
        let parsed = syn::parse_file(&content);
        assert!(parsed.is_ok(), "Generated code should be valid Rust syntax");
    }

    #[test]
    fn test_error_display() {
        let error = CodegenError::EmptyStoryList;
        assert_eq!(error.to_string(), "No stories provided for code generation");

        let error = CodegenError::InvalidStoryData {
            reason: "test reason".to_string(),
        };
        assert_eq!(error.to_string(), "Invalid story data: test reason");

        let error = CodegenError::FileCreationFailed {
            path: "test.rs".to_string(),
            source: std::io::Error::new(std::io::ErrorKind::NotFound, "file not found"),
        };
        assert!(error.to_string().contains("Failed to create file test.rs"));

        let error = CodegenError::TokenGenerationFailed {
            reason: "test reason".to_string(),
        };
        assert_eq!(error.to_string(), "Failed to generate tokens: test reason");
    }
}
