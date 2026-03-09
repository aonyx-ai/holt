# syntax=docker/dockerfile:1

# ---------------------------------------------------------------------------
# Stage 1 – Build the kit-docs WASM app with Trunk
# ---------------------------------------------------------------------------
FROM rust:1.93-bookworm AS builder

# Install wasm target
RUN rustup target add wasm32-unknown-unknown

# Install trunk (WASM bundler)
RUN cargo install trunk --locked

WORKDIR /app

# Copy the full workspace (inter-crate deps require it)
COPY Cargo.toml rust-toolchain.toml ./
COPY crates/ crates/

# Build kit-docs in release mode from its directory
WORKDIR /app/crates/kit-docs
RUN trunk build --release

# ---------------------------------------------------------------------------
# Stage 2 – Serve with Caddy (SPA fallback via --try-files, no config file)
# ---------------------------------------------------------------------------
FROM caddy:alpine

COPY --from=builder /app/crates/kit-docs/dist/ /srv/

EXPOSE 80

CMD ["caddy", "file-server", "--root", "/srv", "--listen", ":80", "--try-files", "{path}", "/index.html"]
