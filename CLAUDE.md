# CLAUDE.md

Holt is a UI toolkit for Leptos following the Shadcn model: users copy component
source code into their own crate and own it. `holt-kit` is an internal reference
implementation (`publish = false`) — it should never appear in docs, examples,
or user-facing code. Users depend on `holt-book` (the storybook framework) and
`holt-cli` (the CLI, which installs a binary called `holt`).

## Workspace Structure

- **`holt-kit`** (`crates/kit/`) - Core UI library
- **`holt-kit-docs`** (`crates/kit-docs/`) - Storybook app
- **`holt-book`** (`crates/book/`) - Storybook framework
- **`holt-macros`** (`crates/story-macro/`) - Story macros
- **`holt-cli`** (`crates/cli/`) - CLI tool
- **`holt-regression`** (`crates/regression/`) - Visual regression testing

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

## Releases

Releases follow [Keep a Changelog](https://keepachangelog.com/) and
[Semantic Versioning](https://semver.org/).

1. Update `CHANGELOG.md`: move items from `[Unreleased]` into a new version
   section dated today.
2. Bump the version in the root `Cargo.toml` `[workspace.package]`.
3. Run `cargo check` to update `Cargo.lock`.
4. Commit, open a PR, and merge.
5. Create a GitHub release with tag `vX.Y.Z` targeting main. The release
   workflow automatically publishes to crates.io (`just publish` runs
   `holt-macros` first, then `holt-book`, then `holt-regression`, then
   `holt-cli`).

Published crates: `holt-macros`, `holt-book`, `holt-regression`, `holt-cli`. The
`holt-kit`, `holt-kit-docs`, and example crates are `publish = false`.

### Labels

PRs are categorized in release notes using these labels:

| Label          | Release notes section |
| -------------- | --------------------- |
| `R-added`      | Added                 |
| `R-changed`    | Changed               |
| `R-deprecated` | Deprecated            |
| `R-removed`    | Removed               |
| `R-fixed`      | Fixed                 |
| `R-security`   | Security              |
| `R-ignore`     | Excluded              |

Area labels: `A-kit`, `A-book`, `A-macros`, `A-cli`, `A-docs`,
`A-github-actions`.

## gVisor / Cloud Sandbox Environments

If `just` commands fail because `flox activate` crashes with
`reading a line: Input/output error`, you are likely in a gVisor (`runsc`)
container. gVisor's pty implementation is incomplete, which breaks the Nix build
log channel even though builds complete successfully.

A **SessionStart hook** (`.claude/settings.json`) automatically runs the fix on
every new session when `$CLAUDE_CODE_REMOTE` is set. No manual action is needed
for Claude Code web sessions.

To run the fix manually:

```bash
bash .claude/fix-flox-gvisor.sh
```

This configures Nix (`sandbox=false`, `filter-syscalls=false`), starts
`nix-daemon`, triggers the environment build, and manually registers the outputs
that Nix failed to record. After that, `flox activate` and all `just` commands
work normally.
