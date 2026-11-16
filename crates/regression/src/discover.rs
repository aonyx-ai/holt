//! Source-level story discovery by scanning Rust files for `#[story]` and `#[variant]` macros

use std::fs;
use std::path::Path;

use regex::Regex;

use crate::error::{Error, Result};
use crate::story::StoryVariant;

/// Discovers all story variants by scanning Rust source files in the given directory
///
/// Walks `dir` recursively for `.rs` files and extracts story IDs and variant names
/// from `#[story(id = "...")]` and `#[variant]` macro annotations.
///
/// # Errors
///
/// Returns an error if the directory cannot be read or a file cannot be parsed.
///
/// # Examples
///
/// ```
/// use holt_regression::discover_variants;
///
/// let dir = tempfile::tempdir().unwrap();
///
/// let variants = discover_variants(dir.path()).unwrap();
///
/// assert!(variants.is_empty());
/// ```
pub fn discover_variants(dir: &Path) -> Result<Vec<StoryVariant>> {
    let mut variants = Vec::new();
    scan_directory(dir, &mut variants)?;
    variants.sort_by(|a, b| {
        a.story_id()
            .cmp(b.story_id())
            .then(a.variant_index().cmp(&b.variant_index()))
    });
    Ok(variants)
}

fn discover_error(path: &Path) -> impl FnOnce(std::io::Error) -> Error {
    let path = path.to_path_buf();
    move |source| Error::Discover { path, source }
}

fn scan_directory(dir: &Path, variants: &mut Vec<StoryVariant>) -> Result<()> {
    let entries = fs::read_dir(dir).map_err(discover_error(dir))?;

    for entry in entries {
        let entry = entry.map_err(discover_error(dir))?;
        let path = entry.path();

        if path.is_dir() {
            scan_directory(&path, variants)?;
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            let source = fs::read_to_string(&path).map_err(discover_error(&path))?;
            variants.extend(extract_variants(&source));
        }
    }

    Ok(())
}

fn extract_variants(source: &str) -> Vec<StoryVariant> {
    let story_re = Regex::new(r#"#\[story\([^)]*id\s*=\s*"([^"]*)""#).expect("valid regex");
    let array_re = Regex::new(r"=\s*&\[([^\]]+)\]").expect("valid regex");
    let variant_re = Regex::new(r"#\[variant\]\s*\n\s*(?:pub\s+)?(?:async\s+)?fn\s+([a-z_]\w*)")
        .expect("valid regex");

    let variant_names: Vec<&str> = variant_re
        .captures_iter(source)
        .map(|cap| cap.get(1).expect("capture group 1").as_str())
        .collect();

    let mut results = Vec::new();

    for story_cap in story_re.captures_iter(source) {
        let story_id = story_cap.get(1).expect("capture group 1").as_str();
        let story_end = story_cap.get(0).expect("full match").end();
        let rest = &source[story_end..];

        let Some(array_cap) = array_re.captures(rest) else {
            continue;
        };

        let array_body = array_cap.get(1).expect("capture group 1").as_str();
        let referenced_names: Vec<&str> = array_body
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();

        for (index, name) in referenced_names.iter().enumerate() {
            if variant_names.contains(name) {
                results.push(
                    StoryVariant::builder()
                        .story_id(story_id)
                        .variant_index(index)
                        .name(*name)
                        .build(),
                );
            }
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    const BUTTON_SOURCE: &str = r#"
use holt_book::{story, variant};
use holt_kit::visual::{Button, ButtonVariant};
use leptos::prelude::*;

#[variant]
fn default() -> AnyView {
    view! { <Button>"Click me!"</Button> }.into_any()
}

#[variant]
fn outline() -> AnyView {
    view! { <Button variant=ButtonVariant::Outline>"Click me!"</Button> }.into_any()
}

#[variant]
fn destructive() -> AnyView {
    view! { <Button variant=ButtonVariant::Destructive>"Click me!"</Button> }.into_any()
}

#[story(id = "button", name = "Button", extra_docs = BUTTON_SOURCE)]
const BUTTON_STORY: () = &[default, outline, destructive];
"#;

    const MULTILINE_ARRAY_SOURCE: &str = r#"
#[variant]
fn default() -> AnyView {
    view! { <Badge>Default</Badge> }.into_any()
}

#[variant]
fn secondary() -> AnyView {
    view! { <Badge>Secondary</Badge> }.into_any()
}

#[variant]
fn outline() -> AnyView {
    view! { <Badge>Outline</Badge> }.into_any()
}

#[story(id = "badge", name = "Badge", extra_docs = BADGE_SOURCE)]
/// Badges are small status indicators
const BADGE_STORY: () = &[
    default,
    secondary,
    outline,
];
"#;

    fn indexes(variants: &[StoryVariant]) -> Vec<usize> {
        variants.iter().map(|v| v.variant_index()).collect()
    }

    fn names(variants: &[StoryVariant]) -> Vec<&str> {
        variants.iter().map(|v| v.name().as_str()).collect()
    }

    fn story_ids(variants: &[StoryVariant]) -> Vec<&str> {
        variants.iter().map(|v| v.story_id().as_str()).collect()
    }

    fn button_directory() -> tempfile::TempDir {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("button.rs"), BUTTON_SOURCE).unwrap();
        dir
    }

    #[test]
    fn discover_variants_from_directory_finds_stories_indexes_variants() {
        let dir = button_directory();

        let variants = discover_variants(dir.path()).unwrap();

        assert_eq!(indexes(&variants), [0, 1, 2]);
    }

    #[test]
    fn discover_variants_from_directory_finds_stories_names_variants() {
        let dir = button_directory();

        let variants = discover_variants(dir.path()).unwrap();

        assert_eq!(names(&variants), ["default", "outline", "destructive"]);
    }

    #[test]
    fn discover_variants_from_directory_finds_stories_returns_three_variants() {
        let dir = button_directory();

        let variants = discover_variants(dir.path()).unwrap();

        assert_eq!(variants.len(), 3);
    }

    #[test]
    fn discover_variants_from_directory_finds_stories_uses_story_id() {
        let dir = button_directory();

        let variants = discover_variants(dir.path()).unwrap();

        assert_eq!(story_ids(&variants), ["button", "button", "button"]);
    }

    #[test]
    fn discover_variants_from_directory_scans_subdirs() {
        let dir = tempfile::tempdir().unwrap();
        let sub = dir.path().join("sub");
        fs::create_dir(&sub).unwrap();
        fs::write(sub.join("button.rs"), BUTTON_SOURCE).unwrap();

        let variants = discover_variants(dir.path()).unwrap();

        assert_eq!(variants.len(), 3);
    }

    #[test]
    fn discover_variants_from_empty_directory_returns_empty() {
        let dir = tempfile::tempdir().unwrap();

        let variants = discover_variants(dir.path()).unwrap();

        assert!(variants.is_empty());
    }

    #[test]
    fn extract_variants_preserves_index_order() {
        let variants = extract_variants(BUTTON_SOURCE);

        for (i, variant) in variants.iter().enumerate() {
            assert_eq!(variant.variant_index(), i);
        }
    }

    #[test]
    fn extract_variants_with_inline_array_indexes_variants() {
        let variants = extract_variants(BUTTON_SOURCE);

        assert_eq!(indexes(&variants), [0, 1, 2]);
    }

    #[test]
    fn extract_variants_with_inline_array_names_variants() {
        let variants = extract_variants(BUTTON_SOURCE);

        assert_eq!(names(&variants), ["default", "outline", "destructive"]);
    }

    #[test]
    fn extract_variants_with_inline_array_returns_three_variants() {
        let variants = extract_variants(BUTTON_SOURCE);

        assert_eq!(variants.len(), 3);
    }

    #[test]
    fn extract_variants_with_inline_array_uses_story_id() {
        let variants = extract_variants(BUTTON_SOURCE);

        assert_eq!(story_ids(&variants), ["button", "button", "button"]);
    }

    #[test]
    fn extract_variants_with_multiline_array_names_variants() {
        let variants = extract_variants(MULTILINE_ARRAY_SOURCE);

        assert_eq!(names(&variants), ["default", "secondary", "outline"]);
    }

    #[test]
    fn extract_variants_with_multiline_array_returns_three_variants() {
        let variants = extract_variants(MULTILINE_ARRAY_SOURCE);

        assert_eq!(variants.len(), 3);
    }

    #[test]
    fn extract_variants_with_multiline_array_uses_story_id() {
        let variants = extract_variants(MULTILINE_ARRAY_SOURCE);

        assert_eq!(story_ids(&variants), ["badge", "badge", "badge"]);
    }

    #[test]
    fn extract_variants_with_no_stories_returns_empty() {
        let source = r#"
fn main() {
    println!("no stories here");
}
"#;

        let variants = extract_variants(source);

        assert!(variants.is_empty());
    }
}
