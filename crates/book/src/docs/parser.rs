use serde_json::Value;
use std::fs;
use std::path::Path;
use thiserror::Error;

/// Custom error type for parser operations
#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Failed to read file {path}: {source}")]
    FileRead {
        path: String,
        source: std::io::Error,
    },

    #[error("Invalid JSON format: {0}")]
    InvalidJson(#[from] serde_json::Error),

    #[error("Missing required field '{field}' in rustdoc JSON")]
    MissingField { field: String },

    #[error("Field '{field}' must be an object, found {found_type}")]
    InvalidFieldType { field: String, found_type: String },
}

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
    fn parse(&self, path: &Path) -> Result<RustdocData, ParserError>;
}

/// Default implementation of RustdocParser
pub struct DefaultRustdocParser;

/// Helper trait to get type name for error messages
trait ValueTypeName {
    fn type_name(&self) -> &'static str;
}

impl ValueTypeName for Value {
    fn type_name(&self) -> &'static str {
        match self {
            Value::Null => "null",
            Value::Bool(_) => "boolean",
            Value::Number(_) => "number",
            Value::String(_) => "string",
            Value::Array(_) => "array",
            Value::Object(_) => "object",
        }
    }
}

impl RustdocParser for DefaultRustdocParser {
    fn parse(&self, path: &Path) -> Result<RustdocData, ParserError> {
        // Read the JSON file
        let json_content = fs::read_to_string(path).map_err(|source| ParserError::FileRead {
            path: path.display().to_string(),
            source,
        })?;

        let json: Value = serde_json::from_str(&json_content)?;

        // Extract the main components
        let external_crates = json
            .get("external_crates")
            .ok_or_else(|| ParserError::MissingField {
                field: "external_crates".to_string(),
            })?
            .as_object()
            .ok_or_else(|| ParserError::InvalidFieldType {
                field: "external_crates".to_string(),
                found_type: json["external_crates"].type_name().to_string(),
            })?
            .clone();

        let index = json
            .get("index")
            .ok_or_else(|| ParserError::MissingField {
                field: "index".to_string(),
            })?
            .as_object()
            .ok_or_else(|| ParserError::InvalidFieldType {
                field: "index".to_string(),
                found_type: json["index"].type_name().to_string(),
            })?
            .clone();

        let paths = json
            .get("paths")
            .ok_or_else(|| ParserError::MissingField {
                field: "paths".to_string(),
            })?
            .as_object()
            .ok_or_else(|| ParserError::InvalidFieldType {
                field: "paths".to_string(),
                found_type: json["paths"].type_name().to_string(),
            })?
            .clone();

        Ok(RustdocData {
            external_crates,
            index,
            paths,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn create_temp_file(content: &str) -> NamedTempFile {
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(content.as_bytes()).unwrap();
        file
    }

    fn valid_rustdoc_json() -> Value {
        json!({
            "external_crates": {
                "1": {
                    "name": "holt_book",
                    "version": "0.1.0"
                }
            },
            "index": {
                "42": {
                    "name": "SomeStruct",
                    "docs": "Some documentation"
                }
            },
            "paths": {
                "42": {
                    "crate_id": 1,
                    "path": ["holt_book", "SomeStruct"]
                }
            }
        })
    }

    #[test]
    fn test_parse_valid_rustdoc_json() {
        let json_str = serde_json::to_string_pretty(&valid_rustdoc_json()).unwrap();
        let temp_file = create_temp_file(&json_str);

        let parser = DefaultRustdocParser;
        let result = parser.parse(temp_file.path());

        assert!(result.is_ok());
        let data = result.unwrap();
        assert!(data.external_crates.contains_key("1"));
        assert!(data.index.contains_key("42"));
        assert!(data.paths.contains_key("42"));
    }

    #[test]
    fn test_parse_minimal_valid_json() {
        let json_str = json!({
            "external_crates": {},
            "index": {},
            "paths": {}
        })
        .to_string();
        let temp_file = create_temp_file(&json_str);

        let parser = DefaultRustdocParser;
        let result = parser.parse(temp_file.path());

        assert!(result.is_ok());
        let data = result.unwrap();
        assert!(data.external_crates.is_empty());
        assert!(data.index.is_empty());
        assert!(data.paths.is_empty());
    }

    #[test]
    fn test_parse_file_not_found() {
        let parser = DefaultRustdocParser;
        let result = parser.parse(Path::new("nonexistent_file.json"));

        assert!(result.is_err());
        match result.unwrap_err() {
            ParserError::FileRead { path, source: _ } => {
                assert_eq!(path, "nonexistent_file.json");
            }
            _ => panic!("Expected FileRead error"),
        }
    }

    #[test]
    fn test_parse_invalid_json() {
        let invalid_json = "{ invalid json content }";
        let temp_file = create_temp_file(invalid_json);

        let parser = DefaultRustdocParser;
        let result = parser.parse(temp_file.path());

        assert!(result.is_err());
        match result.unwrap_err() {
            ParserError::InvalidJson(_) => {}
            _ => panic!("Expected InvalidJson error"),
        }
    }

    #[test]
    fn test_parse_missing_external_crates() {
        let json_str = json!({
            "index": {},
            "paths": {}
        })
        .to_string();
        let temp_file = create_temp_file(&json_str);

        let parser = DefaultRustdocParser;
        let result = parser.parse(temp_file.path());

        assert!(result.is_err());
        match result.unwrap_err() {
            ParserError::MissingField { field } => {
                assert_eq!(field, "external_crates");
            }
            _ => panic!("Expected MissingField error"),
        }
    }

    #[test]
    fn test_parse_missing_index() {
        let json_str = json!({
            "external_crates": {},
            "paths": {}
        })
        .to_string();
        let temp_file = create_temp_file(&json_str);

        let parser = DefaultRustdocParser;
        let result = parser.parse(temp_file.path());

        assert!(result.is_err());
        match result.unwrap_err() {
            ParserError::MissingField { field } => {
                assert_eq!(field, "index");
            }
            _ => panic!("Expected MissingField error"),
        }
    }

    #[test]
    fn test_parse_missing_paths() {
        let json_str = json!({
            "external_crates": {},
            "index": {}
        })
        .to_string();
        let temp_file = create_temp_file(&json_str);

        let parser = DefaultRustdocParser;
        let result = parser.parse(temp_file.path());

        assert!(result.is_err());
        match result.unwrap_err() {
            ParserError::MissingField { field } => {
                assert_eq!(field, "paths");
            }
            _ => panic!("Expected MissingField error"),
        }
    }

    #[test]
    fn test_parse_external_crates_not_object() {
        let json_str = json!({
            "external_crates": "not an object",
            "index": {},
            "paths": {}
        })
        .to_string();
        let temp_file = create_temp_file(&json_str);

        let parser = DefaultRustdocParser;
        let result = parser.parse(temp_file.path());

        assert!(result.is_err());
        match result.unwrap_err() {
            ParserError::InvalidFieldType { field, found_type } => {
                assert_eq!(field, "external_crates");
                assert_eq!(found_type, "string");
            }
            _ => panic!("Expected InvalidFieldType error"),
        }
    }

    #[test]
    fn test_parse_index_not_object() {
        let json_str = json!({
            "external_crates": {},
            "index": 42,
            "paths": {}
        })
        .to_string();
        let temp_file = create_temp_file(&json_str);

        let parser = DefaultRustdocParser;
        let result = parser.parse(temp_file.path());

        assert!(result.is_err());
        match result.unwrap_err() {
            ParserError::InvalidFieldType { field, found_type } => {
                assert_eq!(field, "index");
                assert_eq!(found_type, "number");
            }
            _ => panic!("Expected InvalidFieldType error"),
        }
    }

    #[test]
    fn test_parse_paths_not_object() {
        let json_str = json!({
            "external_crates": {},
            "index": {},
            "paths": []
        })
        .to_string();
        let temp_file = create_temp_file(&json_str);

        let parser = DefaultRustdocParser;
        let result = parser.parse(temp_file.path());

        assert!(result.is_err());
        match result.unwrap_err() {
            ParserError::InvalidFieldType { field, found_type } => {
                assert_eq!(field, "paths");
                assert_eq!(found_type, "array");
            }
            _ => panic!("Expected InvalidFieldType error"),
        }
    }

    #[test]
    fn test_parse_empty_file() {
        let temp_file = create_temp_file("");

        let parser = DefaultRustdocParser;
        let result = parser.parse(temp_file.path());

        assert!(result.is_err());
        match result.unwrap_err() {
            ParserError::InvalidJson(_) => {}
            _ => panic!("Expected InvalidJson error"),
        }
    }

    #[test]
    fn test_error_display() {
        let file_error = ParserError::FileRead {
            path: "test.json".to_string(),
            source: std::io::Error::new(std::io::ErrorKind::NotFound, "file not found"),
        };
        assert!(
            file_error
                .to_string()
                .contains("Failed to read file test.json")
        );

        let missing_field_error = ParserError::MissingField {
            field: "external_crates".to_string(),
        };
        assert_eq!(
            missing_field_error.to_string(),
            "Missing required field 'external_crates' in rustdoc JSON"
        );

        let invalid_type_error = ParserError::InvalidFieldType {
            field: "index".to_string(),
            found_type: "string".to_string(),
        };
        assert_eq!(
            invalid_type_error.to_string(),
            "Field 'index' must be an object, found string"
        );
    }
}
