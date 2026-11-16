//! Image comparison traits and default implementations

/// Compares two images to determine if they match
///
/// The default implementation ([`ExactComparator`]) does a byte-exact comparison. Implement this
/// trait to plug in perceptual diff algorithms or tolerance-based comparisons.
///
/// # Examples
///
/// ```
/// use holt_regression::ImageComparator;
///
/// struct SameLength;
///
/// impl ImageComparator for SameLength {
///     fn images_match(&self, baseline: &[u8], screenshot: &[u8]) -> bool {
///         baseline.len() == screenshot.len()
///     }
/// }
///
/// assert!(SameLength.images_match(b"ab", b"cd"));
/// ```
pub trait ImageComparator: Send + Sync {
    /// Returns `true` if the two images are considered equivalent
    fn images_match(&self, baseline: &[u8], screenshot: &[u8]) -> bool;
}

/// Byte-exact image comparison
///
/// Two images match if and only if their raw bytes are identical. This is the strictest comparator
/// and the default used by [`Config`][config].
///
/// [config]: crate::Config
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
    fn default_comparator_matches_identical_bytes() {
        let comparator: Box<dyn ImageComparator> = Default::default();

        let matches = comparator.images_match(b"same", b"same");

        assert!(matches);
    }

    #[test]
    fn default_comparator_rejects_different_bytes() {
        let comparator: Box<dyn ImageComparator> = Default::default();

        let matches = comparator.images_match(b"a", b"b");

        assert!(!matches);
    }

    #[test]
    fn exact_comparator_is_send_sync_and_unpin() {
        fn assert_send_sync_and_unpin<T: Send + Sync + Unpin>() {}

        assert_send_sync_and_unpin::<ExactComparator>();
    }

    #[test]
    fn exact_matches_identical_bytes() {
        let comparator = ExactComparator;
        let data = b"fake png data";

        let matches = comparator.images_match(data, data);

        assert!(matches);
    }

    #[test]
    fn exact_rejects_different_bytes() {
        let comparator = ExactComparator;

        let matches = comparator.images_match(b"baseline", b"screenshot");

        assert!(!matches);
    }
}
