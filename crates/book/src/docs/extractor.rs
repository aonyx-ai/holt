use thiserror::Error;

use crate::docs::parser::RustdocData;

/// Custom error type for extractor operations
#[derive(Error, Debug)]
pub enum ExtractorError {
    #[error("holt_book crate not found in external_crates")]
    HoltBookCrateNotFound,

    #[error("Story trait not found in paths (expected path: holt_book::ui::story::Story)")]
    StoryTraitNotFound,

    #[error("No Story implementations found in index")]
    StoryImplementationNotFound,

    #[error("Invalid crate ID '{id}': {source}")]
    InvalidCrateId {
        id: String,
        source: std::num::ParseIntError,
    },

    #[error("Invalid story trait ID '{id}': {source}")]
    InvalidStoryId {
        id: String,
        source: std::num::ParseIntError,
    },

    #[error("Invalid story implementation ID: {0}")]
    InvalidStoryImplementationId(String),

    #[error("Story metadata not found in index for ID {id}")]
    StoryMetadataNotFound { id: u64 },
}

/// Story metadata extracted from rustdoc
#[derive(Debug, Clone)]
pub struct StoryMetadata {
    pub name: String,
    pub docs: String,
    // Additional fields like category, tags, etc.
}

/// Trait for extracting domain-specific data from rustdoc JSON
pub trait RustdocDataExtractor<T> {
    /// Extract domain-specific data from rustdoc JSON
    fn extract(&self, data: &RustdocData) -> Result<Vec<T>, ExtractorError>;
}

/// Extractor specifically for Story implementations
pub struct DefaultStoryExtractor;

impl RustdocDataExtractor<StoryMetadata> for DefaultStoryExtractor {
    fn extract(&self, data: &RustdocData) -> Result<Vec<StoryMetadata>, ExtractorError> {
        // Find the holt_book crate ID
        let holt_book_crate = data
            .external_crates
            .iter()
            .find(|(_, val)| {
                val["name"]
                    .as_str()
                    .map(|s| s == "holt_book")
                    .unwrap_or(false)
            })
            .ok_or(ExtractorError::HoltBookCrateNotFound)?;

        let holt_book_crate_id =
            holt_book_crate
                .0
                .parse::<u64>()
                .map_err(|source| ExtractorError::InvalidCrateId {
                    id: holt_book_crate.0.clone(),
                    source,
                })?;

        // Find the "Story" trait ID
        let mut story_id = None;
        for (id, obj) in &data.paths {
            if let Some(obj) = obj.as_object() {
                if let (Some(crate_id), Some(path)) = (
                    obj.get("crate_id").and_then(|v| v.as_u64()),
                    obj.get("path").and_then(|v| v.as_array()).and_then(|a| {
                        a.iter()
                            .map(|item| item.as_str())
                            .collect::<Option<Vec<&str>>>()
                    }),
                ) {
                    if crate_id == holt_book_crate_id
                        && path == ["holt_book", "ui", "story", "Story"]
                    {
                        story_id = Some(id.clone())
                            .map(|s| s.parse::<u64>())
                            .transpose()
                            .map_err(|source| ExtractorError::InvalidStoryId {
                                id: id.clone(),
                                source,
                            })?;
                        break;
                    }
                }
            }
        }

        let story_id = story_id.ok_or(ExtractorError::StoryTraitNotFound)?;

        // Find all Story implementations
        let mut stories_ids: Vec<u64> = vec![];

        for (_, obj) in &data.index {
            if let Some(obj) = obj.get("inner") {
                if let Some(obj) = obj.get("impl") {
                    if let Some(obj) = obj.get("trait") {
                        if obj.get("path").and_then(|p| p.as_str()) != Some("Story")
                            || obj.get("id").and_then(|i| i.as_u64()) != Some(story_id)
                        {
                            continue;
                        }
                    }

                    if let Some(obj) = obj.get("for") {
                        if let Some(obj) = obj.get("resolved_path") {
                            let id = obj.get("id").and_then(|id| id.as_u64()).ok_or_else(|| {
                                ExtractorError::InvalidStoryImplementationId(
                                    "id not found or not a number".to_string(),
                                )
                            })?;
                            stories_ids.push(id);
                        }
                    }
                }
            }
        }

        // Check if we found any story implementations
        if stories_ids.is_empty() {
            return Err(ExtractorError::StoryImplementationNotFound);
        }

        // Extract story metadata
        let mut stories = Vec::new();
        for id in stories_ids {
            let obj = data
                .index
                .get(&id.to_string())
                .and_then(|o| o.as_object())
                .ok_or(ExtractorError::StoryMetadataNotFound { id })?;
            let name = obj.get("name").and_then(|name| name.as_str()).unwrap_or("");
            let docs = obj.get("docs").and_then(|docs| docs.as_str()).unwrap_or("");

            stories.push(StoryMetadata {
                name: name.to_string(),
                docs: docs.to_string(),
            });
        }

        Ok(stories)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{Map, Value, json};

    fn create_rustdoc_data(
        external_crates: Map<String, Value>,
        index: Map<String, Value>,
        paths: Map<String, Value>,
    ) -> RustdocData {
        RustdocData {
            external_crates,
            index,
            paths,
        }
    }

    fn valid_rustdoc_with_single_story() -> RustdocData {
        let mut external_crates = Map::new();
        external_crates.insert(
            "1".to_string(),
            json!({
                "name": "holt_book",
                "version": "0.1.0"
            }),
        );

        let mut paths = Map::new();
        paths.insert(
            "42".to_string(),
            json!({
                "crate_id": 1,
                "path": ["holt_book", "ui", "story", "Story"]
            }),
        );

        let mut index = Map::new();
        index.insert(
            "100".to_string(),
            json!({
                "name": "ButtonStory",
                "docs": "A button component story"
            }),
        );
        index.insert(
            "200".to_string(),
            json!({
                "inner": {
                    "impl": {
                        "trait": {
                            "path": "Story",
                            "id": 42
                        },
                        "for": {
                            "resolved_path": {
                                "id": 100
                            }
                        }
                    }
                }
            }),
        );

        create_rustdoc_data(external_crates, index, paths)
    }

    fn valid_rustdoc_with_multiple_stories() -> RustdocData {
        let mut external_crates = Map::new();
        external_crates.insert(
            "1".to_string(),
            json!({
                "name": "holt_book",
                "version": "0.1.0"
            }),
        );

        let mut paths = Map::new();
        paths.insert(
            "42".to_string(),
            json!({
                "crate_id": 1,
                "path": ["holt_book", "ui", "story", "Story"]
            }),
        );

        let mut index = Map::new();
        index.insert(
            "100".to_string(),
            json!({
                "name": "ButtonStory",
                "docs": "A button component story"
            }),
        );
        index.insert(
            "101".to_string(),
            json!({
                "name": "InputStory",
                "docs": "An input component story"
            }),
        );
        index.insert(
            "102".to_string(),
            json!({
                "name": "EmptyStory",
                "docs": ""
            }),
        );

        // Add impl entries for each story
        index.insert(
            "200".to_string(),
            json!({
                "inner": {
                    "impl": {
                        "trait": {
                            "path": "Story",
                            "id": 42
                        },
                        "for": {
                            "resolved_path": {
                                "id": 100
                            }
                        }
                    }
                }
            }),
        );
        index.insert(
            "201".to_string(),
            json!({
                "inner": {
                    "impl": {
                        "trait": {
                            "path": "Story",
                            "id": 42
                        },
                        "for": {
                            "resolved_path": {
                                "id": 101
                            }
                        }
                    }
                }
            }),
        );
        index.insert(
            "202".to_string(),
            json!({
                "inner": {
                    "impl": {
                        "trait": {
                            "path": "Story",
                            "id": 42
                        },
                        "for": {
                            "resolved_path": {
                                "id": 102
                            }
                        }
                    }
                }
            }),
        );

        create_rustdoc_data(external_crates, index, paths)
    }

    #[test]
    fn test_extract_single_story() {
        let data = valid_rustdoc_with_single_story();
        let extractor = DefaultStoryExtractor;
        let result = extractor.extract(&data);

        assert!(result.is_ok());
        let stories = result.unwrap();
        assert_eq!(stories.len(), 1);
        assert_eq!(stories[0].name, "ButtonStory");
        assert_eq!(stories[0].docs, "A button component story");
    }

    #[test]
    fn test_extract_multiple_stories() {
        let data = valid_rustdoc_with_multiple_stories();
        let extractor = DefaultStoryExtractor;
        let result = extractor.extract(&data);

        assert!(result.is_ok());
        let stories = result.unwrap();
        assert_eq!(stories.len(), 3);

        assert_eq!(stories[0].name, "ButtonStory");
        assert_eq!(stories[0].docs, "A button component story");

        assert_eq!(stories[1].name, "InputStory");
        assert_eq!(stories[1].docs, "An input component story");

        assert_eq!(stories[2].name, "EmptyStory");
        assert_eq!(stories[2].docs, "");
    }

    #[test]
    fn test_extract_holt_book_crate_not_found() {
        let data = create_rustdoc_data(Map::new(), Map::new(), Map::new());
        let extractor = DefaultStoryExtractor;
        let result = extractor.extract(&data);

        assert!(result.is_err());
        match result.unwrap_err() {
            ExtractorError::HoltBookCrateNotFound => {}
            _ => panic!("Expected HoltBookCrateNotFound error"),
        }
    }

    #[test]
    fn test_extract_invalid_crate_id() {
        let mut external_crates = Map::new();
        external_crates.insert(
            "invalid".to_string(),
            json!({
                "name": "holt_book",
                "version": "0.1.0"
            }),
        );

        let data = create_rustdoc_data(external_crates, Map::new(), Map::new());
        let extractor = DefaultStoryExtractor;
        let result = extractor.extract(&data);

        assert!(result.is_err());
        match result.unwrap_err() {
            ExtractorError::InvalidCrateId { id, source: _ } => {
                assert_eq!(id, "invalid");
            }
            _ => panic!("Expected InvalidCrateId error"),
        }
    }

    #[test]
    fn test_extract_story_trait_not_found() {
        let mut external_crates = Map::new();
        external_crates.insert(
            "1".to_string(),
            json!({
                "name": "holt_book",
                "version": "0.1.0"
            }),
        );

        let data = create_rustdoc_data(external_crates, Map::new(), Map::new());
        let extractor = DefaultStoryExtractor;
        let result = extractor.extract(&data);

        assert!(result.is_err());
        match result.unwrap_err() {
            ExtractorError::StoryTraitNotFound => {}
            _ => panic!("Expected StoryTraitNotFound error"),
        }
    }

    #[test]
    fn test_extract_story_trait_wrong_path() {
        let mut external_crates = Map::new();
        external_crates.insert(
            "1".to_string(),
            json!({
                "name": "holt_book",
                "version": "0.1.0"
            }),
        );

        let mut paths = Map::new();
        paths.insert(
            "42".to_string(),
            json!({
                "crate_id": 1,
                "path": ["holt_book", "ui", "component", "Component"]
            }),
        );

        let data = create_rustdoc_data(external_crates, Map::new(), paths);
        let extractor = DefaultStoryExtractor;
        let result = extractor.extract(&data);

        assert!(result.is_err());
        match result.unwrap_err() {
            ExtractorError::StoryTraitNotFound => {}
            _ => panic!("Expected StoryTraitNotFound error"),
        }
    }

    #[test]
    fn test_extract_invalid_story_trait_id() {
        let mut external_crates = Map::new();
        external_crates.insert(
            "1".to_string(),
            json!({
                "name": "holt_book",
                "version": "0.1.0"
            }),
        );

        let mut paths = Map::new();
        paths.insert(
            "invalid".to_string(),
            json!({
                "crate_id": 1,
                "path": ["holt_book", "ui", "story", "Story"]
            }),
        );

        let data = create_rustdoc_data(external_crates, Map::new(), paths);
        let extractor = DefaultStoryExtractor;
        let result = extractor.extract(&data);

        assert!(result.is_err());
        match result.unwrap_err() {
            ExtractorError::InvalidStoryId { id, source: _ } => {
                assert_eq!(id, "invalid");
            }
            _ => panic!("Expected InvalidStoryId error"),
        }
    }

    #[test]
    fn test_extract_no_story_implementations() {
        let mut external_crates = Map::new();
        external_crates.insert(
            "1".to_string(),
            json!({
                "name": "holt_book",
                "version": "0.1.0"
            }),
        );

        let mut paths = Map::new();
        paths.insert(
            "42".to_string(),
            json!({
                "crate_id": 1,
                "path": ["holt_book", "ui", "story", "Story"]
            }),
        );

        let data = create_rustdoc_data(external_crates, Map::new(), paths);
        let extractor = DefaultStoryExtractor;
        let result = extractor.extract(&data);

        assert!(result.is_err());
        match result.unwrap_err() {
            ExtractorError::StoryImplementationNotFound => {}
            _ => panic!("Expected StoryImplementationNotFound error"),
        }
    }

    #[test]
    fn test_extract_invalid_story_implementation_id() {
        let mut external_crates = Map::new();
        external_crates.insert(
            "1".to_string(),
            json!({
                "name": "holt_book",
                "version": "0.1.0"
            }),
        );

        let mut paths = Map::new();
        paths.insert(
            "42".to_string(),
            json!({
                "crate_id": 1,
                "path": ["holt_book", "ui", "story", "Story"]
            }),
        );

        let mut index = Map::new();
        index.insert(
            "100".to_string(),
            json!({
                "inner": {
                    "impl": {
                        "trait": {
                            "path": "Story",
                            "id": 42
                        },
                        "for": {
                            "resolved_path": {
                                "id": "not_a_number"
                            }
                        }
                    }
                }
            }),
        );

        let data = create_rustdoc_data(external_crates, index, paths);
        let extractor = DefaultStoryExtractor;
        let result = extractor.extract(&data);

        assert!(result.is_err());
        match result.unwrap_err() {
            ExtractorError::InvalidStoryImplementationId(_) => {}
            _ => panic!("Expected InvalidStoryImplementationId error"),
        }
    }

    #[test]
    fn test_extract_story_metadata_not_found() {
        let mut external_crates = Map::new();
        external_crates.insert(
            "1".to_string(),
            json!({
                "name": "holt_book",
                "version": "0.1.0"
            }),
        );

        let mut paths = Map::new();
        paths.insert(
            "42".to_string(),
            json!({
                "crate_id": 1,
                "path": ["holt_book", "ui", "story", "Story"]
            }),
        );

        let mut index = Map::new();
        index.insert(
            "100".to_string(),
            json!({
                "inner": {
                    "impl": {
                        "trait": {
                            "path": "Story",
                            "id": 42
                        },
                        "for": {
                            "resolved_path": {
                                "id": 999
                            }
                        }
                    }
                }
            }),
        );

        let data = create_rustdoc_data(external_crates, index, paths);
        let extractor = DefaultStoryExtractor;
        let result = extractor.extract(&data);

        assert!(result.is_err());
        match result.unwrap_err() {
            ExtractorError::StoryMetadataNotFound { id } => {
                assert_eq!(id, 999);
            }
            _ => panic!("Expected StoryMetadataNotFound error"),
        }
    }

    #[test]
    fn test_extract_story_with_missing_name_and_docs() {
        let mut external_crates = Map::new();
        external_crates.insert(
            "1".to_string(),
            json!({
                "name": "holt_book",
                "version": "0.1.0"
            }),
        );

        let mut paths = Map::new();
        paths.insert(
            "42".to_string(),
            json!({
                "crate_id": 1,
                "path": ["holt_book", "ui", "story", "Story"]
            }),
        );

        let mut index = Map::new();
        index.insert("100".to_string(), json!({})); // Missing name and docs
        index.insert(
            "200".to_string(),
            json!({
                "inner": {
                    "impl": {
                        "trait": {
                            "path": "Story",
                            "id": 42
                        },
                        "for": {
                            "resolved_path": {
                                "id": 100
                            }
                        }
                    }
                }
            }),
        );

        let data = create_rustdoc_data(external_crates, index, paths);
        let extractor = DefaultStoryExtractor;
        let result = extractor.extract(&data);

        assert!(result.is_ok());
        let stories = result.unwrap();
        assert_eq!(stories.len(), 1);
        assert_eq!(stories[0].name, "");
        assert_eq!(stories[0].docs, "");
    }

    #[test]
    fn test_error_display() {
        let error = ExtractorError::HoltBookCrateNotFound;
        assert_eq!(
            error.to_string(),
            "holt_book crate not found in external_crates"
        );

        let error = ExtractorError::StoryTraitNotFound;
        assert_eq!(
            error.to_string(),
            "Story trait not found in paths (expected path: holt_book::ui::story::Story)"
        );

        let error = ExtractorError::InvalidCrateId {
            id: "invalid".to_string(),
            source: "invalid".parse::<u64>().unwrap_err(),
        };
        assert!(error.to_string().contains("Invalid crate ID 'invalid'"));

        let error = ExtractorError::StoryMetadataNotFound { id: 42 };
        assert_eq!(
            error.to_string(),
            "Story metadata not found in index for ID 42"
        );
    }
}
