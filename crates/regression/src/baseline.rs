//! Baseline image storage and cleanup.

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use crate::error::Error;
use crate::story::{self, StoryVariant};

/// Save a screenshot as a new or updated baseline.
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

/// Remove baseline images that no longer correspond to any discovered variant.
///
/// Walks `baseline_dir/<story_id>/<name>.png` and deletes any file not in
/// the expected set. Empty story directories are removed as well.
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
    for entry in fs::read_dir(baseline_dir).map_err(Error::Cleanup)? {
        let entry = entry.map_err(Error::Cleanup)?;
        if !entry.file_type().map_err(Error::Cleanup)?.is_dir() {
            continue;
        }

        for file_entry in fs::read_dir(entry.path()).map_err(Error::Cleanup)? {
            let file_entry = file_entry.map_err(Error::Cleanup)?;
            if !file_entry.file_type().map_err(Error::Cleanup)?.is_file() {
                continue;
            }

            let path = file_entry.path();
            if !expected.contains(&path) {
                orphaned.push(path);
            }
        }
    }

    for path in &orphaned {
        fs::remove_file(path).map_err(Error::Cleanup)?;

        if let Some(parent) = path.parent()
            && let Ok(mut entries) = fs::read_dir(parent)
            && entries.next().is_none()
        {
            let _ = fs::remove_dir(parent);
        }
    }

    Ok(orphaned)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    fn variant(story_id: &str, name: &str) -> StoryVariant {
        StoryVariant {
            story_id: story_id.to_string(),
            variant_index: 0,
            name: name.to_string(),
        }
    }

    #[test]
    fn save_creates_directories_and_file() {
        let dir = tempfile::tempdir().unwrap();
        let v = variant("button", "default");
        save(dir.path(), &v, b"png data").unwrap();

        let path = dir.path().join("button").join("default.png");
        assert_eq!(fs::read(&path).unwrap(), b"png data");
    }

    #[test]
    fn save_overwrites_existing() {
        let dir = tempfile::tempdir().unwrap();
        let v = variant("button", "default");
        save(dir.path(), &v, b"old").unwrap();
        save(dir.path(), &v, b"new").unwrap();

        let path = dir.path().join("button").join("default.png");
        assert_eq!(fs::read(&path).unwrap(), b"new");
    }

    #[test]
    fn cleanup_removes_orphaned_files() {
        let dir = tempfile::tempdir().unwrap();
        let kept = variant("button", "default");
        let orphan = variant("button", "old_variant");

        save(dir.path(), &kept, b"keep").unwrap();
        save(dir.path(), &orphan, b"remove").unwrap();

        let removed = cleanup_orphaned(dir.path(), &[kept]).unwrap();
        assert_eq!(removed.len(), 1);
        assert!(removed[0].ends_with("old_variant.png"));
        assert!(!removed[0].exists());
    }

    #[test]
    fn cleanup_removes_empty_story_dirs() {
        let dir = tempfile::tempdir().unwrap();
        let orphan = variant("obsolete", "only_variant");
        save(dir.path(), &orphan, b"data").unwrap();

        cleanup_orphaned(dir.path(), &[]).unwrap();
        assert!(!dir.path().join("obsolete").exists());
    }

    #[test]
    fn cleanup_noop_when_no_baseline_dir() {
        let dir = tempfile::tempdir().unwrap();
        let missing = dir.path().join("nonexistent");
        let removed = cleanup_orphaned(&missing, &[]).unwrap();
        assert!(removed.is_empty());
    }
}
