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
immediately — before users do.

For a component library, snapshot testing is essential because:

1. **Styling is the product**: Components exist to look right
2. **CSS is global and fragile**: One change can cascade unexpectedly
3. **Rendering varies**: Font hinting, subpixel rendering, and layout engines
   differ across environments

## How It Works

Holt uses [doco](https://crates.io/crates/doco) to run screenshots in Docker
containers. When you run `holt snapshot`:

1. `trunk build --release` compiles your storybook to static files
2. A Caddy server starts in a Docker container, serving the built storybook
3. A headless Chrome instance (also in Docker) navigates to each story variant
4. Screenshots are captured at a fixed viewport (1280x720)
5. Each screenshot is compared byte-for-byte against its baseline PNG

Because both the server and browser run inside containers, results are
consistent across machines — no need to worry about local browser versions or OS
font rendering.

## How Screenshot Comparison Works

Holt uses byte-level comparison: if the PNG files are identical byte-for-byte,
the test passes. This approach is:

- **Fast**: No image decoding or pixel analysis needed
- **Deterministic**: Either it matches or it doesn't
- **Sensitive**: Any change, even a single pixel, triggers a diff

The tradeoff is sensitivity. Identical visual output can produce different bytes
due to PNG encoder differences or timing issues with animations. Running
everything in Docker mitigates most of these problems.

## Local vs CI Workflows

Snapshot testing behaves differently depending on context.

### Local Development

When you run `holt snapshot` locally:

1. Screenshots are captured at 1280x720
2. Differences trigger a GUI comparison window
3. You decide whether to accept or reject each change
4. Orphaned baselines (from deleted stories) are cleaned up

The GUI makes it easy to review changes interactively. Toggle between baseline
and new screenshot to spot differences, then accept or reject with a click.

### CI Environment

When you run `holt snapshot --check`:

1. The browser runs headless
2. Screenshots are captured at the same resolution
3. Any difference causes the test to fail immediately
4. No screenshots are saved, no prompts are shown

Adding `--report report.html` generates an interactive HTML comparison report
with slider, side-by-side, and toggle modes for reviewing differences. The CI
workflow uploads this as a clickable artifact, so reviewers can inspect changes
directly in their browser without downloading anything.

The visual regression CI workflow compares against main-branch baselines and
posts a PR comment summarizing changes with instructions for accepting them.

## GUI vs Terminal Approval

The comparison GUI requires a display. On systems without one (SSH sessions,
containers), Holt falls back to terminal mode:

1. Both images are saved to temp files
2. Your OS's default image viewer opens them (if available)
3. You're prompted to accept or reject via stdin

Terminal mode is functional but less convenient. The GUI is preferred when
available.

## Orphan Cleanup

When you delete a story or rename a variant, its baseline becomes orphaned — it
no longer corresponds to anything in your storybook. Holt detects and removes
these automatically during local runs.

Orphan cleanup only runs locally (not in CI) because:

- CI shouldn't modify committed baselines
- Detecting orphans requires knowing all current variants
- Accidental deletions are easier to recover locally

If you see "Cleaning up N orphaned baseline(s)" in output, those files will be
deleted. Review the list to make sure they're actually obsolete.

## Limitations and Tradeoffs

### Byte Comparison is Strict

Any rendering difference fails the test. This catches real regressions but can
cause false positives from timing issues with async content or animations.

Mitigation: Use static data in stories and disable animations in test mode.

### Chrome Only

Holt uses Chrome via doco's Docker-based browser. This means you're not testing
Firefox or Safari rendering.

For cross-browser snapshot testing, consider additional tooling or a service
like Percy or Chromatic that tests multiple browsers.

### No Pixel Tolerance

Some snapshot testing tools allow "fuzzy" matching that ignores small
differences. Holt's byte comparison doesn't. This is intentional — if pixels
changed, you should know.

The comparison trait (`ImageComparator`) is pluggable, so tolerance-based
comparators could be added without changing the rest of the system.

## Design Decisions

### Why Docker?

Running the server and browser inside Docker containers means:

- No local browser or driver installation required
- Consistent rendering across developer machines and CI
- No version skew between local and CI environments
- Single prerequisite: a running Docker daemon

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
