---
sidebar_position: 4
---

# Snapshot Testing

This page explains why Holt includes snapshot testing and how the system works
under the hood.

## Why Snapshot Testing?

Unit tests verify behavior. Type systems catch interface mismatches. But neither
catches when a CSS change accidentally hides a button, or when a dependency
update subtly shifts your layout.

Snapshot testing fills this gap by comparing screenshots of your components
against known-good baselines. If anything looks different, you find out
immediately—before users do.

For a component library like Holt, snapshot testing is essential because:

1. **Styling is the product**: Components exist to look right
2. **CSS is global and fragile**: One change can cascade unexpectedly
3. **Cross-browser rendering varies**: What works in Chrome may shift in Firefox

## How Screenshot Comparison Works

Holt uses byte-level comparison: if the PNG files are identical byte-for-byte,
the test passes. This approach is:

- **Fast**: No image decoding or pixel analysis needed
- **Deterministic**: Either it matches or it doesn't
- **Sensitive**: Any change, even a single pixel, triggers a diff

The tradeoff is sensitivity. Identical visual output can produce different bytes
due to:

- PNG encoder differences
- Font rendering variations between OS versions
- Timing differences in animations

That's why Holt captures all baselines with the same browser (Firefox) and
recommends pinning browser versions in CI.

## Local vs CI Workflows

Snapshot testing behaves differently depending on context.

### Local Development

When you run `holt snapshot` locally:

1. Firefox opens visibly (not headless)
2. Screenshots are captured at 1280x720
3. Differences trigger a GUI comparison window
4. You decide whether to accept or reject each change
5. Orphaned baselines (from deleted stories) are cleaned up

The GUI makes it easy to review changes interactively. Toggle between baseline
and new screenshot to spot differences, then accept or reject with a click.

### CI Environment

When the `CI` environment variable is set:

1. Firefox runs headless (no display needed)
2. Screenshots are captured at the same resolution
3. Differences cause the test to fail
4. New screenshots are saved for artifact upload
5. Orphan cleanup is skipped (baselines shouldn't change in CI)

This workflow lets you:

1. Run tests to detect regressions
2. Download artifacts when tests fail
3. Review the new screenshots locally
4. Update baselines and commit if the changes are intentional

## GUI vs Terminal Approval

The comparison GUI requires a display. On systems without one (SSH sessions,
some CI environments, Linux without X11), Holt falls back to terminal mode:

1. Both images are saved to temp files
2. Your OS's default image viewer opens them (if available)
3. You're prompted to accept or reject via stdin

Terminal mode is functional but less convenient. The GUI is preferred when
available.

## Orphan Cleanup

When you delete a story or rename a variant, its baseline becomes orphaned—it no
longer corresponds to anything in your storybook. Holt detects and removes these
automatically during local runs.

Orphan cleanup only runs locally (not in CI) because:

- CI shouldn't modify committed baselines
- Detecting orphans requires knowing all current variants
- Accidental deletions are easier to recover locally

If you see "Cleaning up N orphaned baseline(s)" in output, those files will be
deleted. Review the list to make sure they're actually obsolete.

## Limitations and Tradeoffs

### Byte Comparison is Strict

Any rendering difference fails the test. This catches real regressions but can
cause false positives from:

- OS font rendering changes
- Browser updates
- Timing issues with async content

Mitigation: Pin browser versions in CI and ensure components are fully rendered
before capture.

### Firefox Only

Holt uses Firefox via geckodriver. This simplifies setup (one browser to
install) but means you're not testing Chrome or Safari rendering.

For cross-browser snapshot testing, consider additional tooling or a service
like Percy or Chromatic that tests multiple browsers.

### No Pixel Tolerance

Some snapshot testing tools allow "fuzzy" matching that ignores small
differences. Holt's byte comparison doesn't. This is intentional—if pixels
changed, you should know.

If you need tolerance, you could implement perceptual hashing or diff
percentages, but that adds complexity and can mask real issues.

## Design Decisions

### Why geckodriver/Firefox?

- Open source with stable WebDriver support
- Consistent rendering across platforms
- Headless mode works reliably
- No licensing concerns

### Why Store Baselines in the Repo?

Baselines in version control mean:

- CI can compare without external dependencies
- Changes are code-reviewed along with the code that caused them
- History shows exactly when visual changes occurred
- Anyone can run tests without downloading baselines separately

The cost is repository size, but PNG screenshots of components are typically
small (10-50KB each).

### Why a GUI for Review?

Terminal-based approval (y/n prompts) works but makes it hard to actually see
the difference. The GUI lets you toggle between images instantly, making review
faster and more confident.

The fallback to terminal mode ensures the tool works everywhere, even if less
conveniently.
