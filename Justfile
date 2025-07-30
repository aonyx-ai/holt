[private]
default:
    @just --list --justfile {{ justfile() }}

# Serve the Holt Book for our UI components
[working-directory('crates/ui-book')]
serve:
    trunk serve
