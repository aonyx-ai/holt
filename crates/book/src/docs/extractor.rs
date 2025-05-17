use std::error::Error;

use crate::docs::parser::RustdocData;

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
    fn extract(&self, data: &RustdocData) -> Result<Vec<T>, Box<dyn Error>>;
}

/// Extractor specifically for Story implementations
pub struct StoryExtractor;

impl RustdocDataExtractor<StoryMetadata> for StoryExtractor {
    fn extract(&self, data: &RustdocData) -> Result<Vec<StoryMetadata>, Box<dyn Error>> {
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
            .ok_or("holt_book crate not found")?;

        let holt_book_crate_id = holt_book_crate.0.parse::<u64>()?;

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
                        story_id = Some(id.clone()).map(|s| s.parse::<u64>()).transpose()?;
                        break;
                    }
                }
            }
        }

        let story_id = story_id
            .ok_or_else(|| format!("Story trait with crate_id {} not found", holt_book_crate_id))?;

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
                            stories_ids.push(
                                obj.get("id")
                                    .and_then(|id| id.as_u64())
                                    .ok_or("id not found or not a number")?,
                            );
                        }
                    }
                }
            }
        }

        // Extract story metadata
        let mut stories = Vec::new();
        for id in stories_ids {
            let obj = data
                .index
                .get(&id.to_string())
                .and_then(|o| o.as_object())
                .ok_or("no item found or not an object")?;
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
