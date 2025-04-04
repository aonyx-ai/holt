VERSION 0.8

IMPORT github.com/earthly/lib/rust AS rust

FROM rust:1.84.0-slim
WORKDIR /holt

all:
    BUILD +format
    BUILD +lint
    BUILD +test
    BUILD +webapp-build

COPY_RUST_SOURCES:
    FUNCTION

    # Copy the source code in a cache-friendly way
    COPY --keep-ts --if-exists Cargo.toml Cargo.lock ./
    COPY --keep-ts --dir crates ./

container:
    # Initialize Rust
    DO rust+INIT --keep_fingerprints=true

    # Install clippy and rustfmt
    RUN rustup component add clippy rustfmt

    # Explicitly cache the container at this point
    SAVE IMAGE --cache-hint

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

sources:
    FROM +container

    # Copy the source code in a cache-friendly way
    DO +COPY_RUST_SOURCES

cargo-build:
    FROM +sources

    # Build the project
    DO rust+CARGO --args="build --all-features"

    # Explicitly cache the container at this point
    SAVE IMAGE --cache-hint

format:
    FROM +sources

    # Check the code formatting
    DO rust+CARGO --args="fmt --all --check"

lint:
    FROM +cargo-build

    # Check the code for linting errors
    DO rust+CARGO --args="clippy --all-targets --all-features -- -D warnings --no-deps"

test:
    FROM +cargo-build

    # Run the tests and measure the code coverage
    DO rust+CARGO --args="test --all-targets --all-features"

webapp-build:
    FROM +webapp-container

    # Copy the source code in a cache-friendly way
    DO +COPY_RUST_SOURCES

    # Build the web application
    RUN cd crates/book && trunk build
