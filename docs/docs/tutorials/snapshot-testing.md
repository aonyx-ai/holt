---
sidebar_position: 2
---

# Snapshot Testing

This tutorial walks you through setting up and running snapshot tests for your
component storybook. By the end, you'll have baseline screenshots for your
components and know how to catch unintended visual changes.

## Prerequisites

- A working Holt storybook (see
  [Your First Storybook](/docs/tutorials/first-storybook))
- Firefox installed
- geckodriver installed and in your PATH

To check if geckodriver is available:

```bash
geckodriver --version
```

If not installed, get it from
[Mozilla's geckodriver releases](https://github.com/mozilla/geckodriver/releases).

## Step 1: Run Your First Snapshot Test

From your project root (where `holt.toml` lives), run:

```bash
holt snapshot
```

You'll see output like:

```
Holt Snapshot Testing
================================

Starting geckodriver...
Starting Trunk server...
Connecting to WebDriver...

Processing 12 story variants...

  [new] card/default (new baseline)
  -> Baseline created (test will fail until committed)
  [new] card/with-custom-class (new baseline)
  -> Baseline created (test will fail until committed)
  ...

================================
Results: 0 passed, 12 failed
```

The "failures" are expected—there were no baselines to compare against. Holt
created them for you.

## Step 2: Explore the Baseline Directory

Check what was created:

```bash
ls -la tests/visual-baselines/
```

You'll see a directory structure like:

```
tests/visual-baselines/
├── card/
│   ├── default.png
│   ├── with-custom-class.png
│   └── minimal.png
└── button/
    ├── primary.png
    └── secondary.png
```

Each story variant gets its own screenshot, organized by story name.

## Step 3: Verify Baselines Pass

Run the test again:

```bash
holt snapshot
```

Now you should see:

```
Processing 12 story variants...

  [ok] card/default matches baseline
  [ok] card/with-custom-class matches baseline
  ...

================================
Results: 12 passed, 0 failed
```

All tests pass because the screenshots match the baselines.

## Step 4: Make a Visual Change

Let's see what happens when a component changes. Open one of your components and
make a visible change—perhaps change a padding value, border color, or font
size.

For example, if you have a Card component:

```rust
// Change this:
let base_classes = "rounded-lg border bg-card";
// To this:
let base_classes = "rounded-xl border-2 bg-card";
```

## Step 5: See the Diff

Run the snapshot tests again:

```bash
holt snapshot
```

For any changed component, a comparison window opens showing:

- **Baseline** tab: The original screenshot
- **New Screenshot** tab: What the component looks like now

You can toggle between views to spot the differences.

## Step 6: Accept or Reject the Change

In the comparison window:

- Click **Accept New** to update the baseline with the new screenshot
- Click **Reject** to keep the old baseline (test stays failed)

If you accept, the baseline file is updated immediately.

## Step 7: Commit Your Baselines

Baseline images should be committed to version control so CI can compare against
them:

```bash
git add tests/visual-baselines/
git commit -m "Update baselines for card component"
```

## Troubleshooting

### geckodriver not found

Make sure geckodriver is in your PATH:

```bash
# macOS with Homebrew
brew install geckodriver

# Or download from GitHub and add to PATH
export PATH="$PATH:/path/to/geckodriver"
```

### Firefox crashes or times out

Ensure Firefox is installed and up to date. The WebDriver needs a compatible
Firefox version.

### Comparison window doesn't open

On headless systems (like SSH sessions), Holt falls back to terminal mode. It
will print file paths and ask for yes/no input instead of showing a GUI.

## Next Steps

- Read the [Snapshot Testing Guide](/docs/guides/snapshot-testing) for CI
  integration and workflow tips
- Check the [CLI Reference](/docs/reference/cli) for all `holt snapshot` options
- Understand [Why Snapshot Testing](/docs/explanation/snapshot-testing) to learn
  about the design decisions
