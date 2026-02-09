---
name: pr
description:
  Pre-PR checklist to ensure completeness before opening a pull request. Run
  this before creating a PR to catch missing tests, lint issues, and untracked
  files.
---

# Pre-PR Checklist

Run this skill before opening a pull request to ensure your changes are complete
and CI-ready.

## Checklist

Execute all checks in order. Fix any issues before proceeding to the next step.

### 1. Run Lint Suite

```bash
just lint-rust
```

Fix all clippy warnings — CI runs with `-D warnings` so any warning fails the
build.

### 2. Run Full Pre-commit Checks

```bash
just pre-commit
```

This runs:

- Formatting (prettier, taplo, leptosfmt)
- Markdown lint
- YAML lint
- GitHub Actions lint
- Unit tests

Fix any failures before continuing.

### 3. Run E2E Tests (if applicable)

If you added or modified UI components:

```bash
just kit test-e2e
```

### 4. Check for Untracked Files

```bash
git status
```

For any untracked files:

- Stage them if they should be committed
- Add to `.gitignore` if they should be ignored
- Delete if they're temporary artifacts

Do NOT leave untracked files behind.

### 5. Verify Test Coverage

List all tests that cover your changes:

- Unit tests in `**/tests/**` or `#[cfg(test)]` modules
- E2E tests in `crates/kit-docs/tests/` (for UI changes)

If any test category is missing for new features, write the tests now.

### 6. Final Verification

```bash
cargo test --all-features --all-targets
git status
```

Only proceed to create the PR when:

- [ ] All lints pass with no warnings
- [ ] All tests pass
- [ ] No untracked files
- [ ] E2E tests exist for new UI features
