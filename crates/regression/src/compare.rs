//! Image comparison traits and default implementations.

/// Compares two images to determine if they match.
///
/// The default implementation ([`ExactComparator`]) does a byte-exact comparison. Implement this
/// trait to plug in perceptual diff algorithms or tolerance-based comparisons.
pub trait ImageComparator: Send + Sync {
    /// Returns `true` if the two images are considered equivalent.
    fn images_match(&self, baseline: &[u8], screenshot: &[u8]) -> bool;
}

/// Byte-exact image comparison.
///
/// Two images match if and only if their raw bytes are identical. This is the strictest comparator
/// and the default used by [`Config`](crate::Config).
pub struct ExactComparator;

impl ImageComparator for ExactComparator {
    fn images_match(&self, baseline: &[u8], screenshot: &[u8]) -> bool {
        baseline == screenshot
    }
}

impl Default for Box<dyn ImageComparator> {
    fn default() -> Self {
        Box::new(ExactComparator)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_matches_identical_bytes() {
        let comparator = ExactComparator;
        let data = b"fake png data";
        assert!(comparator.images_match(data, data));
    }

    #[test]
    fn exact_rejects_different_bytes() {
        let comparator = ExactComparator;
        assert!(!comparator.images_match(b"baseline", b"screenshot"));
    }

    #[test]
    fn default_comparator_is_exact() {
        let comparator: Box<dyn ImageComparator> = Default::default();
        assert!(comparator.images_match(b"same", b"same"));
        assert!(!comparator.images_match(b"a", b"b"));
    }
}
