# CLAUDE.md

Holt is a UI toolkit for Leptos implementing Shadcn/Radix-style components with
behavior/presentation separation.

## Workspace Structure

- **`holt-kit`** (`crates/ui/`) - Core UI library
- **`holt-kit-docs`** (`crates/ui-book/`) - Storybook app
- **`holt-book`** (`crates/book/`) - Storybook framework
- **`holt-story-macro`** (`crates/story-macro/`) - Story macros

## Development

Do NOT use `cargo` commands directly.

```bash
# Main workflow
just kit-docs serve

# Testing
just test-rust          # Unit tests
just kit test-e2e        # Browser integration tests

# Code quality
just format-rust true   # Format code
just lint-rust          # Lint code
just pre-commit         # Run all checks

# See all commands
just
```

## Testing

When claiming a feature is "complete", verify that ALL applicable test types
have been written and are passing. Never claim completeness without running the
test suite.

- New features MUST include E2E tests (`just kit test-e2e`) in addition to unit
  tests
- Before declaring work done, explicitly list what tests were written and show
  they pass
- If a test category doesn't apply, state why — don't silently skip it

## Pre-PR Checklist

Before opening a PR or claiming work is done:

1. Run `just pre-commit` — this runs formatting, linting, and unit tests
2. Run `just lint-rust` — catches clippy warnings that CI will flag
3. Run `git status` and address any untracked files (commit, `.gitignore`, or
   delete them)
4. Verify no warnings in build output

Do NOT rely solely on pre-commit hooks — run the full check suite manually since
CI runs additional checks that pre-commit doesn't cover.

## Requirements

These are installed via Flox.

- Rust 1.88+
- Leptos 0.8+
- Tailwind CSS via `tailwind_fuse`
- `cargo install just trunk wasm-pack`
