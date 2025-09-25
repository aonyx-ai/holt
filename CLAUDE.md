# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with
code in this repository.

## Project Overview

Holt is a UI toolkit for the Leptos web framework, implementing
Shadcn/Radix-style components with a clean separation between behavior and
presentation layers.

## Architecture

### Crate Structure

The project is organized as a Cargo workspace with these key crates:

- **`holt-ui`** (`crates/ui/`) - Core UI library with behavior and visual components
- **`holt-ui-book`** (`crates/ui-book/`) - Storybook application for component development
- **`holt-book`** (`crates/book/`) - Storybook framework and runtime
- **`holt-story-macro`** (`crates/story-macro/`) - Procedural macros for story generation

### Component Architecture

**Behavior vs Visual Separation**: Components are split into two layers:

- **Behavior components** (`src/behavior/`) - Handle state management,
  interactions, and business logic using Leptos primitives
- **Visual components** (`src/visual/`) - Apply styling (Tailwind CSS) and
  visual presentation, wrapping behavior components

**Example**:

- `SelectRoot`, `SelectTrigger`, `SelectContent` in `behavior/select.rs`
  manage state and interactions
- `Select`, `SelectTrigger`, `SelectContent` in `visual/select.rs` add
  Shadcn-style classes and visual polish

### Floating Positioning System

The `floating` module provides a comprehensive positioning system for dropdowns,
tooltips, and popovers:

- **Real DOM positioning** using `getBoundingClientRect()`
- **Reactive positioning** that updates when NodeRefs are populated
- **Side positioning** (Top, Right, Bottom, Left) with offsets
- **Collision detection** and viewport constraints (planned)

Key components:

- `use_floating()` hook for reactive positioning
- `FloatingOptions` for configuration
- `calculate_position()` for coordinate calculation

### Story System

Components are documented and tested using a custom storybook system:

- Stories defined using `#[story]` and `#[variant]` macros
- Auto-generated source code examples
- Component variants showcase different states and configurations

## Development Commands

### Just Task Runner

The project uses [Just](https://github.com/casey/just) for task automation.
Install with `cargo install just`.

```bash
# List all available commands
just

# Start the component storybook (main development workflow)
just ui_book serve
```

### Storybook Development

```bash
# Preferred: Use Just task runner
just ui_book serve

# Alternative: Direct Trunk command
cd crates/ui-book && trunk serve
```

### Testing

**Using Just (Recommended):**

```bash
# Run all unit tests
just test

# Run unit tests for specific package
just test-package holt-ui

# Run browser-based integration tests (WASM)
just test-wasm

# Run browser tests with Chrome
just test-wasm-chrome

# Debug browser tests (visible browser window)
just test-wasm-debug

# Run all tests (unit + integration)
just test-all
```

**Direct Cargo Commands:**

```bash
# Run all unit tests
cargo test

# Test specific crate
cargo test -p holt-ui

# Test specific module (e.g., floating positioning)
cargo test -p holt-ui floating::tests

# Test with output
cargo test -- --nocapture

# Browser integration tests (requires wasm-pack)
wasm-pack test --headless --firefox crates/ui
wasm-pack test --headless --chrome crates/ui
```

### Build Commands

```bash
# Using Just
just check
just check-package holt-ui

# Direct Cargo
cargo check
cargo check -p holt-ui
cargo build --release
```

## Key Implementation Patterns

### Component Development

1. Create behavior component in `src/behavior/` with state management
2. Create visual wrapper in `src/visual/` with Tailwind styling
3. Add story in `crates/ui-book/src/stories/` with variants
4. Export both from respective `mod.rs` files

### Floating Component Integration

When creating components that need positioning (dropdowns, tooltips):

1. Use `NodeRef` for trigger and floating elements
2. Call `use_floating()` with positioning options
3. Apply returned coordinates to style attributes
4. Ensure NodeRefs track properly for reactive updates

### Testing Strategy

- **Unit tests** for core logic and calculations
- **Integration tests** for browser-specific functionality using `wasm-bindgen-test`
- **Stories** serve as visual tests and documentation
- Critical: Test both the calculation logic AND the reactive updates

## Dependencies and Requirements

- **Rust 1.88+** (required by Leptos 0.8)
- **Leptos 0.8+** - Web framework
- **Tailwind CSS** - Styling via `tailwind_fuse`
- **wasm-pack** - For browser testing (`cargo install wasm-pack`)
- **Trunk** - For serving the storybook (`cargo install trunk`)
- **Just** - Task runner (`cargo install just`) - recommended for easier development

## Important Notes

### Leptos Integration

- Components use Leptos signals for reactivity
- `NodeRef` is used for DOM element access
- Effects must track dependencies properly for reactive updates
- Portal component used for dropdowns/overlays

### Browser Testing

- Integration tests require actual browser environment
- Use `wasm-bindgen-test` with `run_in_browser` configuration
- Tests validate both data structures AND DOM interactions
- Critical for floating positioning system validation

### Styling Approach

- Uses `tailwind_fuse` for dynamic class composition
- Shadcn/ui design system aesthetic
- Consistent with Radix UI component patterns
- Visual components wrap behavior with styling only
