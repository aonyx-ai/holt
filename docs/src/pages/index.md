---
title: Holt - UI Toolkit & Storybook for Leptos
---

<div className="hero-logo">
  <img src="/img/logo.svg" alt="Holt" />
</div>

A UI toolkit and component storybook framework for [Leptos](https://leptos.dev).

## What is Holt?

Holt provides two things:

1. **Holt Kit** - A UI component library implementing Shadcn/Radix-style
   components with behavior/presentation separation
2. **Holt Book** - A storybook framework for developing and showcasing your
   Leptos components

## Quick Start

### Run a Storybook

```bash
# Install the CLI
cargo install holt-cli

# Start the dev server
holt serve
```

### Showcase Your Components

Copy component source code into your own crate, then use `holt-book` to build an
interactive storybook:

```rust
use holt_book::{story, variant};
use leptos::prelude::*;

#[variant]
fn default() -> AnyView {
    view! { <Button>"Click me"</Button> }.into_any()
}

#[story(id = "button", name = "Button")]
const BUTTON_STORY: () = &[default];
```

## Features

- **Behavior/Presentation Separation** - Core behaviors are decoupled from
  styling, letting you customize the look while keeping consistent interactions
- **Tailwind CSS** - Built with `tailwind_fuse` for composable, type-safe
  styling
- **Leptos 0.8+** - Modern reactive Rust framework for the web
- **Shadcn/Radix Patterns** - Familiar component APIs inspired by the JavaScript
  ecosystem
- **Component Storybook** - Develop and document components in isolation

## Documentation

<div class="doc-grid">

### [Tutorials](/docs/tutorials)

Step-by-step guides for getting started. Build your first storybook and learn
the fundamentals.

### [How-to Guides](/docs/guides)

Practical recipes for common tasks like customizing styles and setting up dark
mode.

### [Explanation](/docs/explanation)

Background and context about Holt's design decisions and architecture.

### [Reference](/docs/reference)

Technical documentation for the CLI and story macro API.

</div>
