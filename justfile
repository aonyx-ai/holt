mod ui_book "crates/ui-book"
mod ui "crates/ui"

set shell := ["flox", "activate", "--", "sh", "-cu"]

msrv := `grep 'rust-version' Cargo.toml | sed 's/.*rust-version = "\([^"]*\)".*/\1/'`

[private]
default:
    @just --list --list-submodules --justfile {{ justfile() }}

# Run a subset of checks as pre-commit hooks
pre-commit-inner:
    #!/usr/bin/env -S parallel --shebang --ungroup --jobs {{ num_cpus() }}
    just prettier true
    just format-toml true
    just format-rust true
    just lint-github-actions
    just lint-markdown
    just lint-rust
    just lint-yaml
    just test-rust

pre-commit:
    just pre-commit-inner

# Check documentation
check-docs:
    cargo doc --all-features --no-deps

# Check features with cargo-hack
check-features:
    cargo hack --workspace --feature-powerset check --tests

# Check latest dependencies with cargo-update
check-deps-latest:
    #!/usr/bin/env -S bash .flox/in-tmp-flox-env.sh check-deps-latest
    # TODO(marts): Figure out how to install beta through the CLI

    # cargo update
    # RUSTFLAGS="-D deprecated" cargo test --all-features --all-targets --locked

    echo "This is broken, for now..."

# Check minimal dependencies with cargo-update
check-deps-minimal:
    #!/usr/bin/env -S bash .flox/in-tmp-flox-env.sh check-deps-minimal
    flox uninstall cargo
    # TODO(marts): Figure out how to install beta through the CLI
    # flox install
    # cargo test --all-features --all-targets --locked

    echo "This is broken, for now..."

# Check MSRV
check-msrv:
    #!/usr/bin/env -S bash .flox/in-tmp-flox-env.sh check-msrv
    flox uninstall cargo
    flox install cargo@{{ msrv }}
    cargo check --workspace --all-features --all-targets

# Format JSON files
format-json fix="false": (prettier fix "{json,json5}")

# Format Markdown files
format-markdown fix="false": (prettier fix "md")

# Format Leptos files
format-leptos fix="false":
    leptosfmt {{ if fix != "true" { "--check" } else { "" } }} "crates/**/*.rs"

# Format Rust files
format-rust fix="false":
    cargo fmt {{ if fix != "true" { "--check" } else { "" } }}

# Format Just files
format-just fix="false":
    just --fmt {{ if fix != "true" { "--check" } else { "" } }} --unstable

# Format TOML files
format-toml fix="false":
    taplo fmt {{ if fix != "true" { "--diff" } else { "" } }}

# Format YAML files
format-yaml fix="false": (prettier fix "{yaml,yml}")

# Lint GitHub Actions workflows
lint-github-actions:
    zizmor -p .

# Lint Markdown files
lint-markdown:
    markdownlint **/*.md

# Lint Rust files
lint-rust:
    cargo clippy --all-targets --all-features -- -D warnings

# Lint TOML files
lint-toml:
    taplo check

# Lint YAML files
lint-yaml:
    yamllint .

# Auto-format files with prettier
prettier fix="false" extension="*":
    prettier {{ if fix == "true" { "--write" } else { "--list-different" } }} --ignore-unknown "**/*.{{ extension }}"

# Run the tests
test-rust:
    cargo test --all-features --all-targets
