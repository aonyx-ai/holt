---
sidebar_position: 3
---

# Snapshot Testing

Snapshot testing catches unintended visual changes by comparing screenshots of
your component stories against committed baselines. Holt uses
[doco](https://crates.io/crates/doco) under the hood, which spins up a Docker
container to serve your storybook and a headless Chrome instance to capture
screenshots. The only prerequisite is a running Docker daemon.

## Run Snapshot Tests

From your project root:

```bash
holt snapshot
```

This builds your storybook with `trunk build --release`, starts a Caddy server
in a Docker container, launches a browser, captures a screenshot of every story
variant, and compares each against its baseline in `tests/visual-baselines/`.

### CLI Flags

| Flag            | Description                                                     |
| --------------- | --------------------------------------------------------------- |
| `--check`       | CI mode: pass/fail only, no saving, no prompts. Exits non-zero. |
| `--headless`    | Run the browser without a visible window.                       |
| `--no-headless` | Force a visible browser even in non-interactive shells.         |
| `--save`        | Save new/changed screenshots (default: true).                   |
| `--no-save`     | Don't save screenshots.                                         |

Headless mode is auto-detected: if stdout is not a terminal, the browser runs
headless.

## Review and Accept Changes

When a screenshot differs from its baseline, the behavior depends on whether
you're running locally or in CI.

### Local Review

A comparison window opens showing two tabs:

- **Baseline**: The expected appearance
- **New Screenshot**: The current appearance

Toggle between them to spot differences, then:

- **Accept**: Update the baseline with the new screenshot
- **Reject**: Keep the old baseline

On systems without a display server, Holt falls back to terminal mode — it opens
both images in your default viewer and prompts you to accept or reject.

### CI Mode

In CI, run with `--check`. Any mismatch or new variant fails the build. The
visual regression workflow posts a comment on the PR with a summary of changes
and instructions for accepting them.

## Update Baselines from CI

When the visual regression workflow detects changes, it uploads the new
screenshots as an artifact and posts a PR comment with a one-liner to accept
them:

```bash
just kit-docs load-baselines-from-gh-artifact <run-id>
```

This downloads the artifact, replaces your local baselines, and shows you the
git diff. Review the changes, then commit:

```bash
git add tests/visual-baselines/
git commit -m "Accept new visual baselines"
```

## Add Snapshot Tests to CI

### GitHub Actions

The only infrastructure requirement is Docker. No need to install Firefox,
geckodriver, or any browser — doco handles all of that inside containers.

A minimal job:

```yaml
snapshot-testing:
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4

    - name: Install Rust
      uses: dtolnay/rust-action@stable

    - name: Run snapshot tests
      run: cargo run -p holt-cli -- snapshot --check

    - name: Upload screenshots on failure
      if: failure()
      uses: actions/upload-artifact@v4
      with:
        name: visual-regression-screenshots
        path: tests/visual-baselines/
```

See `.github/workflows/visual-regression.yml` in the Holt repo for the full
workflow, which also compares against main-branch baselines and posts PR
comments.

## Handle Flaky Tests

Snapshot tests can be sensitive to rendering differences. Tips for stability:

- **Use Docker for consistency.** Since doco runs the browser inside a
  container, font rendering and viewport size are identical across machines.
- **Disable animations** in your storybook's test mode.
- **Use static data** in stories instead of async fetches.

## Reset All Baselines

To regenerate all baselines from scratch:

```bash
rm -rf tests/visual-baselines/
holt snapshot
git add tests/visual-baselines/
git commit -m "Regenerate all baselines"
```
