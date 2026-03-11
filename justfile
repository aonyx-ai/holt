mod book "crates/book"
mod kit-docs "crates/kit-docs"
mod kit "crates/kit"

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
    just format-leptos true
    just lint-github-actions
    just lint-markdown
    just lint-yaml
    just test-rust

pre-commit:
    just pre-commit-inner

# Check documentation
check-docs:
    cargo doc --all-features --no-deps

# Check features with cargo-hack
check-features:
    cargo hack --workspace --feature-powerset --mutually-exclusive-features csr,ssr,hydrate check --tests

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
format-leptos fix="false": (format-rust fix)
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
    zizmor -p -c .zizmor.yml .

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

# Generate pre-compiled CSS for holt-book
generate-book-css:
    #!/usr/bin/env -S flox activate -- bash
    set -euo pipefail
    cd crates/book
    tailwindcss -i tailwind-input.css -o assets/holt-book.css
    # Strip theme variables, @property declarations, and @layer properties
    # fallback — consumers already have these from their own @import "tailwindcss".
    if [[ "$(uname)" == "Darwin" ]]; then SED=(sed -i ''); else SED=(sed -i); fi
    "${SED[@]}" '/^@layer properties;$/d; /^:root {$/,$d' assets/holt-book.css
    "${SED[@]}" -e :a -e '/^[[:space:]]*$/{' -e '$d' -e N -e ba -e '}' assets/holt-book.css

# Publish crates to crates.io
publish: generate-book-css
    cargo publish -p holt-macros -v --all-features
    cargo publish -p holt-book -v --all-features
    cargo publish -p holt-cli -v --all-features

# Run the tests
test-rust:
    cargo test --all-targets -p holt-kit -p holt-cli -p holt-regression -p holt-macros
    cargo test --all-targets -p holt-book -p holt-kit-docs --features ssr --no-default-features

# Run integration test with the example crate
test-example: generate-book-css
    cd examples/basic && cargo run -p holt-cli -- snapshot --check

# Run browser integration tests for the example crate
test-example-e2e: generate-book-css
    cd examples/basic && trunk build --release && cargo test --test e2e
