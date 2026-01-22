---
sidebar_position: 1
---

# CLI Commands

The `holt` CLI provides commands for developing and building component
storybooks.

## Installation

```bash
cargo install holt
```

## Commands

### `holt serve`

Start a development server with hot reloading.

```bash
holt serve [OPTIONS]
```

**Options:**

| Option         | Default     | Description                |
| -------------- | ----------- | -------------------------- |
| `--port`, `-p` | `8080`      | Port to run the server on  |
| `--host`       | `127.0.0.1` | Host address to bind to    |
| `--open`, `-o` | `false`     | Open browser automatically |

**Examples:**

```bash
# Start on default port
holt serve

# Start on custom port
holt serve --port 3000

# Start and open browser
holt serve --open

# Bind to all interfaces (for network access)
holt serve --host 0.0.0.0
```

### `holt build`

Build a static storybook for deployment.

```bash
holt build [OPTIONS]
```

**Options:**

| Option            | Default | Description                 |
| ----------------- | ------- | --------------------------- |
| `--out-dir`, `-o` | `dist`  | Output directory            |
| `--base-path`     | `/`     | Base path for deployed site |

**Examples:**

```bash
# Build to default directory
holt build

# Build to custom directory
holt build --out-dir public

# Build for subdirectory deployment
holt build --base-path /my-project/storybook/
```

### `holt init`

Initialize a new storybook in an existing Leptos project.

```bash
holt init [OPTIONS]
```

**Options:**

| Option          | Default   | Description               |
| --------------- | --------- | ------------------------- |
| `--stories-dir` | `stories` | Directory for story files |
| `--force`, `-f` | `false`   | Overwrite existing files  |

**Examples:**

```bash
# Initialize with defaults
holt init

# Use custom stories directory
holt init --stories-dir src/stories

# Reinitialize (overwrite existing)
holt init --force
```

This command creates:

- Story module directory
- Basic story template
- Configuration file (if needed)

## Exit Codes

| Code | Meaning           |
| ---- | ----------------- |
| `0`  | Success           |
| `1`  | General error     |
| `2`  | Invalid arguments |
| `3`  | Build failure     |

## Environment Variables

| Variable         | Description                                   |
| ---------------- | --------------------------------------------- |
| `HOLT_BOOK_PORT` | Default port for `serve` command              |
| `HOLT_BOOK_HOST` | Default host for `serve` command              |
| `RUST_LOG`       | Logging level (e.g., `debug`, `info`, `warn`) |

## Configuration File

Holt Book looks for `holt.toml` in the project root:

```toml
[serve]
port = 8080
host = "127.0.0.1"
open = false

[build]
out_dir = "dist"
base_path = "/"

[stories]
dir = "stories"
```

Command-line options override configuration file values.
