mod ui_book "crates/ui-book"

wasm_pack := require("wasm-pack")

[private]
default:
    @just --list --list-submodules --justfile {{ justfile() }}

# Run all unit tests
test:
    cargo test

# Run unit tests for specific package
test-package package:
    cargo test -p {{ package }}

# Run browser-based integration tests (WASM)
test-wasm:
    {{ wasm_pack }} test --headless --firefox crates/ui

# Run browser-based integration tests with Chrome
test-wasm-chrome:
    {{ wasm_pack }} test --headless --chrome crates/ui

# Run browser tests with visible browser (for debugging)
test-wasm-debug:
    {{ wasm_pack }} test --firefox crates/ui

# Run all tests (unit + integration)
test-all: test test-wasm

# Check code compilation
check:
    cargo check

# Check specific package
check-package package:
    cargo check -p {{ package }}
