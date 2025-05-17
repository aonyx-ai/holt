use serde_json::Value;
use std::error::Error;
use std::fs;
use std::path::Path;

/// Represents the structure of rustdoc JSON output
#[derive(Debug)]
pub struct RustdocData {
    pub external_crates: serde_json::Map<String, Value>,
    pub index: serde_json::Map<String, Value>,
    pub paths: serde_json::Map<String, Value>,
}

/// Trait for parsing rustdoc JSON output
pub trait RustdocParser {
    /// Parse rustdoc JSON output from a file
    fn parse(&self, path: &Path) -> Result<RustdocData, Box<dyn Error>>;
}

/// Default implementation of RustdocParser
pub struct DefaultRustdocParser;

impl RustdocParser for DefaultRustdocParser {
    fn parse(&self, path: &Path) -> Result<RustdocData, Box<dyn Error>> {
        if !path.exists() {
            return Err(format!("{} not found", path.display()).into());
        }

        // Read the JSON file
        let json_content = fs::read_to_string(path)?;
        let json: Value = serde_json::from_str(&json_content)?;

        // Extract the main components
        let external_crates = json["external_crates"]
            .as_object()
            .ok_or("External crates not found or not an object")?
            .clone();

        let index = json["index"]
            .as_object()
            .ok_or("Index not found or not an object")?
            .clone();

        let paths = json["paths"]
            .as_object()
            .ok_or("Paths not found or not an object")?
            .clone();

        Ok(RustdocData {
            external_crates,
            index,
            paths,
        })
    }
}
