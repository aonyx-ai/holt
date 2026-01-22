---
sidebar_position: 1
---

# CLI Commands

The `holt` CLI provides commands for developing and building component
storybooks. It wraps [Trunk](https://trunkrs.dev/) and adds project-aware
configuration.

## Installation

```bash
cargo install holt-cli
```

## Configuration File

Holt looks for `holt.toml` in the current directory. This lets you run commands
from anywhere in your project:

```toml
[book]
path = "crates/kit-docs"  # Directory containing Trunk.toml and index.html

[serve]
port = 3000               # Default port (overridable via --port)
open = false              # Default for --open flag
```

All sections and fields are optional. Without a config file, Holt uses the
current directory and default values.

Command-line options override configuration file values.

## Commands

### `holt serve`

Start a development server with hot reloading.

```bash
holt serve [OPTIONS]
```

**Options:**

| Option         | Default             | Description                |
| -------------- | ------------------- | -------------------------- |
| `--port`, `-p` | `8080` (or config)  | Port to run the server on  |
| `--open`, `-o` | `false` (or config) | Open browser automatically |

The server runs Trunk in the directory specified by `book.path` in your config
(or current directory if not set).

**Examples:**

```bash
# Start on default port
holt serve

# Start on custom port
holt serve --port 3000

# Start and open browser
holt serve --open
```

### `holt build`

Build a static storybook for deployment.

```bash
holt build [OPTIONS]
```

**Options:**

| Option            | Default | Description           |
| ----------------- | ------- | --------------------- |
| `--release`, `-r` | `false` | Build in release mode |

The build runs Trunk in the directory specified by `book.path` in your config.

**Examples:**

```bash
# Development build
holt build

# Production build
holt build --release
```

## Example Project Setup

For a workspace with your storybook in a subdirectory:

```
my-project/
├── holt.toml
├── crates/
│   ├── my-lib/
│   └── kit-docs/        # Storybook lives here
│       ├── Trunk.toml
│       ├── index.html
│       └── src/
└── ...
```

Create `holt.toml` at the project root:

```toml
[book]
path = "crates/kit-docs"

[serve]
port = 3000
open = true
```

Now you can run `holt serve` from anywhere in the project.
