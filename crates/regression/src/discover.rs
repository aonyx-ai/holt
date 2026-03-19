//! Source-level story discovery by scanning Rust files for `#[story]` and `#[variant]` macros

use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use regex::Regex;

use crate::story::StoryVariant;

/// Discovers all story variants by scanning Rust source files in the given directory
///
/// Walks `dir` recursively for `.rs` files and extracts story IDs and variant names
/// from `#[story(id = "...")]` and `#[variant]` macro annotations.
///
/// # Errors
///
/// Returns an error if the directory cannot be read or a file cannot be parsed.
pub fn discover_variants(dir: &Path) -> Result<Vec<StoryVariant>> {
    let mut variants = Vec::new();
    scan_directory(dir, &mut variants).with_context(|| format!("scan {}", dir.display()))?;
    variants.sort_by(|a, b| {
        a.story_id
            .cmp(&b.story_id)
            .then(a.variant_index.cmp(&b.variant_index))
    });
    Ok(variants)
}

fn scan_directory(dir: &Path, variants: &mut Vec<StoryVariant>) -> Result<()> {
    let entries = fs::read_dir(dir).with_context(|| format!("read directory {}", dir.display()))?;

    for entry in entries {
        let entry = entry.with_context(|| format!("read entry in {}", dir.display()))?;
        let path = entry.path();

        if path.is_dir() {
            scan_directory(&path, variants).with_context(|| format!("scan {}", path.display()))?;
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            let source =
                fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?;
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
                results.push(StoryVariant {
                    story_id: story_id.to_string(),
                    variant_index: index,
                    name: name.to_string(),
                });
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

    #[test]
    fn discover_variants_from_directory_finds_stories() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("button.rs"), BUTTON_SOURCE).unwrap();

        let variants = discover_variants(dir.path()).unwrap();

        assert_eq!(variants.len(), 3);
        assert_eq!(variants[0].story_id, "button");
        assert_eq!(variants[0].name, "default");
        assert_eq!(variants[0].variant_index, 0);
        assert_eq!(variants[1].name, "outline");
        assert_eq!(variants[1].variant_index, 1);
        assert_eq!(variants[2].name, "destructive");
        assert_eq!(variants[2].variant_index, 2);
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
    fn extract_variants_with_inline_array() {
        let variants = extract_variants(BUTTON_SOURCE);

        assert_eq!(variants.len(), 3);
        assert_eq!(variants[0].story_id, "button");
        assert_eq!(variants[0].variant_index, 0);
        assert_eq!(variants[0].name, "default");
        assert_eq!(variants[2].variant_index, 2);
        assert_eq!(variants[2].name, "destructive");
    }

    #[test]
    fn extract_variants_with_multiline_array() {
        let variants = extract_variants(MULTILINE_ARRAY_SOURCE);

        assert_eq!(variants.len(), 3);
        assert_eq!(variants[0].story_id, "badge");
        assert_eq!(variants[0].name, "default");
        assert_eq!(variants[1].name, "secondary");
        assert_eq!(variants[2].name, "outline");
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

    #[test]
    fn extract_variants_preserves_index_order() {
        let variants = extract_variants(BUTTON_SOURCE);

        for (i, variant) in variants.iter().enumerate() {
            assert_eq!(variant.variant_index, i);
        }
    }
}
