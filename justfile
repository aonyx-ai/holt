mod ui_book "crates/ui-book"

[private]
default:
    @just --list --list-submodules --justfile {{ justfile() }}
