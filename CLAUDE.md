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

## Requirements

These are installed via Flox.

- Rust 1.88+
- Leptos 0.8+
- Tailwind CSS via `tailwind_fuse`
- `cargo install just trunk wasm-pack`

## gVisor / Cloud Sandbox Environments

If `just` commands fail because `flox activate` crashes with
`reading a line: Input/output error`, you are likely in a gVisor (`runsc`)
container. gVisor's pty implementation is incomplete, which breaks the
Nix build log channel even though builds complete successfully.

A **SessionStart hook** (`.claude/settings.json`) automatically runs the fix
on every new session when `$CLAUDE_CODE_REMOTE` is set. No manual action is
needed for Claude Code web sessions.

To run the fix manually:

```bash
bash .claude/fix-flox-gvisor.sh
```

This configures Nix (`sandbox=false`, `filter-syscalls=false`), starts
`nix-daemon`, triggers the environment build, and manually registers the
outputs that Nix failed to record. After that, `flox activate` and all
`just` commands work normally.
