# CLAUDE.md

Holt is a UI toolkit for Leptos implementing Shadcn/Radix-style components with behavior/presentation separation.

## Workspace Structure

- **`holt-ui`** (`crates/ui/`) - Core UI library
- **`holt-ui-book`** (`crates/ui-book/`) - Storybook app
- **`holt-book`** (`crates/book/`) - Storybook framework
- **`holt-story-macro`** (`crates/story-macro/`) - Story macros

## Development

```bash
# Main workflow
just ui_book serve

# Testing
just test-rust          # Unit tests
just ui test-e2e        # Browser integration tests

# Code quality
just format-rust true   # Format code
just lint-rust          # Lint code
just pre-commit         # Run all checks

# See all commands
just
```

## Requirements

These are installed via Flox.

- Rust 1.88+
- Leptos 0.8+
- Tailwind CSS via `tailwind_fuse`
- `cargo install just trunk wasm-pack`
