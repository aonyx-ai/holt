---
sidebar_position: 3
---

# Snapshot Testing

This guide covers common tasks when working with snapshot tests in your
day-to-day development workflow.

## Run Snapshot Tests

From your project root:

```bash
holt snapshot
```

This starts a server, launches Firefox, captures screenshots of every story
variant, and compares them against baselines.

To use a different port:

```bash
holt snapshot --port 4000
```

## Review and Accept Changes

When a screenshot differs from its baseline, a comparison window opens (locally)
or the test fails (in CI).

### Local Review

The comparison window shows two tabs:

- **Baseline**: The expected appearance
- **New Screenshot**: The current appearance

Toggle between them to spot differences, then:

- **Accept New**: Update the baseline with the new screenshot
- **Reject**: Keep the old baseline

### Terminal Mode

On headless systems, Holt falls back to terminal mode. It opens both images in
your default viewer and prompts:

```
Screenshot differs for card/default. Accept new baseline? [y/N]:
```

## Update Baselines from CI Artifacts

When tests fail in CI, the new screenshots are saved as artifacts. To update
baselines:

1. Download the CI artifacts (usually a zip of `tests/visual-baselines/`)
2. Extract and replace your local baselines:

```bash
# Example for GitHub Actions
unzip visual-baselines.zip -d tests/
```

3. Review the changes:

```bash
git diff tests/visual-baselines/
# Or use a visual diff tool
```

4. Commit the updated baselines:

```bash
git add tests/visual-baselines/
git commit -m "Update baselines"
```

## Add Snapshot Tests to CI

### GitHub Actions

Add a job that runs after your storybook builds:

```yaml
snapshot-testing:
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4

    - name: Install Rust
      uses: dtolnay/rust-action@stable

    - name: Install geckodriver
      run: |
        wget -q https://github.com/mozilla/geckodriver/releases/download/v0.35.0/geckodriver-v0.35.0-linux64.tar.gz
        tar -xzf geckodriver-v0.35.0-linux64.tar.gz
        sudo mv geckodriver /usr/local/bin/

    - name: Install Firefox
      uses: browser-actions/setup-firefox@latest

    - name: Run snapshot tests
      run: holt snapshot

    - name: Upload baselines on failure
      if: failure()
      uses: actions/upload-artifact@v4
      with:
        name: visual-baselines
        path: tests/visual-baselines/
```

The `CI` environment variable triggers headless mode automatically.

### Other CI Systems

Set the `CI` environment variable to enable headless mode:

```bash
CI=true holt snapshot
```

## Handle Flaky Tests

Snapshot tests can be sensitive to:

- **Font rendering**: Different OS versions render fonts slightly differently
- **Timing**: Animations or async content may not be ready when captured
- **Viewport size**: Ensure consistent browser dimensions

Tips for stable tests:

1. **Pin browser versions** in CI to avoid rendering differences
2. **Disable animations** in your storybook's test mode
3. **Use static data** in stories instead of async fetches

## Reset All Baselines

To regenerate all baselines from scratch:

```bash
rm -rf tests/visual-baselines/
holt snapshot
git add tests/visual-baselines/
git commit -m "Regenerate all baselines"
```
