//! Baseline image storage and cleanup

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use crate::error::Error;
use crate::story::{self, StoryVariant};

/// Saves a screenshot as a new or updated baseline
///
/// # Examples
///
/// ```
/// use holt_regression::{StoryVariant, save};
///
/// let dir = tempfile::tempdir().unwrap();
/// let variant = StoryVariant::builder()
///     .story_id("button")
///     .variant_index(0)
///     .name("default")
///     .build();
///
/// save(dir.path(), &variant, b"png data").unwrap();
///
/// assert!(dir.path().join("button").join("default.png").exists());
/// ```
pub fn save(baseline_dir: &Path, variant: &StoryVariant, screenshot: &[u8]) -> Result<(), Error> {
    let path = story::baseline_path(baseline_dir, variant);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| Error::BaselineWrite {
            path: path.clone(),
            source: e,
        })?;
    }
    fs::write(&path, screenshot).map_err(|e| Error::BaselineWrite { path, source: e })?;
    Ok(())
}

/// Removes baseline images that no longer correspond to any discovered variant
///
/// Walks `baseline_dir/<story_id>/<name>.png` and deletes any file not in
/// the expected set. Empty story directories are removed as well.
///
/// # Examples
///
/// ```
/// use holt_regression::{StoryVariant, cleanup_orphaned, save};
///
/// let dir = tempfile::tempdir().unwrap();
/// let variant = StoryVariant::builder()
///     .story_id("button")
///     .variant_index(0)
///     .name("old")
///     .build();
/// save(dir.path(), &variant, b"png data").unwrap();
///
/// let removed = cleanup_orphaned(dir.path(), &[]).unwrap();
///
/// assert_eq!(removed.len(), 1);
/// ```
pub fn cleanup_orphaned(
    baseline_dir: &Path,
    variants: &[StoryVariant],
) -> Result<Vec<PathBuf>, Error> {
    if !baseline_dir.exists() {
        return Ok(Vec::new());
    }

    let expected: HashSet<PathBuf> = variants
        .iter()
        .map(|v| story::baseline_path(baseline_dir, v))
        .collect();

    let mut orphaned = Vec::new();
    for entry in fs::read_dir(baseline_dir).map_err(cleanup_error)? {
        let entry = entry.map_err(cleanup_error)?;
        if !entry.file_type().map_err(cleanup_error)?.is_dir() {
            continue;
        }

        for file_entry in fs::read_dir(entry.path()).map_err(cleanup_error)? {
            let file_entry = file_entry.map_err(cleanup_error)?;
            if !file_entry.file_type().map_err(cleanup_error)?.is_file() {
                continue;
            }

            let path = file_entry.path();
            if !expected.contains(&path) {
                orphaned.push(path);
            }
        }
    }

    for path in &orphaned {
        fs::remove_file(path).map_err(cleanup_error)?;

        if let Some(parent) = path.parent()
            && let Ok(mut entries) = fs::read_dir(parent)
            && entries.next().is_none()
        {
            let _ = fs::remove_dir(parent);
        }
    }

    Ok(orphaned)
}

fn cleanup_error(source: std::io::Error) -> Error {
    Error::Cleanup { source }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[bon::builder]
    fn variant(story_id: &str, name: &str) -> StoryVariant {
        StoryVariant::builder()
            .story_id(story_id)
            .variant_index(0)
            .name(name)
            .build()
    }

    fn cleanup_with_one_orphan() -> (tempfile::TempDir, Vec<PathBuf>) {
        let dir = tempfile::tempdir().unwrap();
        let kept = variant().story_id("button").name("default").call();
        let orphan = variant().story_id("button").name("old_variant").call();
        save(dir.path(), &kept, b"keep").unwrap();
        save(dir.path(), &orphan, b"remove").unwrap();

        let removed = cleanup_orphaned(dir.path(), &[kept]).unwrap();

        (dir, removed)
    }

    #[test]
    fn cleanup_noop_when_no_baseline_dir() {
        let dir = tempfile::tempdir().unwrap();
        let missing = dir.path().join("nonexistent");

        let removed = cleanup_orphaned(&missing, &[]).unwrap();

        assert!(removed.is_empty());
    }

    #[test]
    fn cleanup_removes_empty_story_dirs() {
        let dir = tempfile::tempdir().unwrap();
        let orphan = variant().story_id("obsolete").name("only_variant").call();
        save(dir.path(), &orphan, b"data").unwrap();

        cleanup_orphaned(dir.path(), &[]).unwrap();

        assert!(!dir.path().join("obsolete").exists());
    }

    #[test]
    fn cleanup_removes_orphaned_files_deletes_file() {
        let (_dir, removed) = cleanup_with_one_orphan();

        assert!(!removed[0].exists());
    }

    #[test]
    fn cleanup_removes_orphaned_files_reports_one_path() {
        let (_dir, removed) = cleanup_with_one_orphan();

        assert_eq!(removed.len(), 1);
    }

    #[test]
    fn cleanup_removes_orphaned_files_reports_orphan_name() {
        let (_dir, removed) = cleanup_with_one_orphan();

        assert!(removed[0].ends_with("old_variant.png"));
    }

    #[test]
    fn save_creates_directories_and_file() {
        let dir = tempfile::tempdir().unwrap();
        let v = variant().story_id("button").name("default").call();

        save(dir.path(), &v, b"png data").unwrap();

        let path = dir.path().join("button").join("default.png");
        assert_eq!(fs::read(&path).unwrap(), b"png data");
    }

    #[test]
    fn save_overwrites_existing() {
        let dir = tempfile::tempdir().unwrap();
        let v = variant().story_id("button").name("default").call();
        save(dir.path(), &v, b"old").unwrap();

        save(dir.path(), &v, b"new").unwrap();

        let path = dir.path().join("button").join("default.png");
        assert_eq!(fs::read(&path).unwrap(), b"new");
    }
}
