VERSION 0.8

IMPORT github.com/earthly/lib/rust AS rust

FROM rust:1.87.0-slim
WORKDIR /holt

COPY_RUST_SOURCES:
    FUNCTION

    # Copy the source code in a cache-friendly way
    COPY --keep-ts --if-exists Cargo.toml Cargo.lock ./
    COPY --keep-ts --dir crates ./

webapp-container:
    # Install system-level dependencies
    RUN apt update && apt upgrade -y && apt install -y curl libssl-dev pkg-config

    # Initialize Rust
    DO rust+INIT --keep_fingerprints=true

    # Add the WASM target
    RUN rustup target add wasm32-unknown-unknown

    # Install trunk to compile the web application
    RUN cargo install trunk

    # Explicitly cache the container at this point
    SAVE IMAGE --cache-hint

webapp-build:
    FROM +webapp-container

    # Copy the source code in a cache-friendly way
    DO +COPY_RUST_SOURCES

    # Build the web application
    RUN cd crates/book && trunk build

# This project's continuous integration pipeline executes all Earthly targets
# that start with the prefixes `check-`, `format-`, `lint-`, and `test-`. Fixing
# formatting issues is disabled to prevent parallely running targets from
# overwriting each other's changes.
checks:
    BUILD +check-docs
    BUILD +check-features
    BUILD +check-latest-deps
    BUILD +check-minimal-deps
    BUILD +check-msrv
    BUILD +format-json --FIX="false"
    BUILD +format-markdown --FIX="false"
    BUILD +format-rust --FIX="false"
    BUILD +format-toml --FIX="false"
    BUILD +format-yaml --FIX="false"
    BUILD +lint-markdown
    BUILD +lint-rust
    BUILD +lint-yaml
    BUILD +test-rust

# These targets get executed by pre-commit before every commit. Some need to be
# run sequentially to avoid overwriting each other's changes.
pre-commit:
    WAIT
        BUILD +prettier
    END
    WAIT
        BUILD +format-toml
    END
    BUILD +format-rust
    BUILD +lint-markdown
    BUILD +lint-rust
    BUILD +lint-yaml

check-docs:
    DO ./.earthly/rust+DOCS

check-features:
    DO ./.earthly/rust+FEATURES

check-latest-deps:
    DO ./.earthly/rust+DEPS_LATEST

check-minimal-deps:
    DO ./.earthly/rust+DEPS_MINIMAL

check-msrv:
    ARG MSRV="1.81.0"
    DO ./.earthly/rust+MSRV --MSRV="$MSRV"

format-json:
    ARG FIX="false"
    DO ./.earthly/prettier+PRETTIER --EXTENSION="{json,json5}" --FIX="$FIX"

format-markdown:
    ARG FIX="false"
    DO ./.earthly/prettier+PRETTIER --EXTENSION="md" --FIX="$FIX"

format-rust:
    ARG FIX="false"
    DO ./.earthly/rust+FORMAT --FIX="$FIX"

format-toml:
    ARG FIX="false"
    DO ./.earthly/toml+FORMAT --FIX="$FIX"

format-yaml:
    ARG FIX="false"
    DO ./.earthly/prettier+PRETTIER --EXTENSION="{yaml,yml}" --FIX="$FIX"

lint-markdown:
    DO ./.earthly/markdown+LINT

lint-rust:
    DO ./.earthly/rust+LINT

lint-yaml:
    DO ./.earthly/yaml+LINT

prettier:
    ARG FIX="false"
    DO ./.earthly/prettier+PRETTIER --FIX="$FIX"

publish-crate:
    ARG CRATE=""
    DO ./.earthly/rust+PUBLISH --CRATE="$CRATE"

test-rust:
    DO ./.earthly/rust+TEST
