---
sidebar_position: 2
---

# Story Macro

The `#[story]` and `#[variant]` attribute macros define stories and their
variants in Holt Book. Stories are registered automatically via the `inventory`
crate — no manual registration step is needed.

## Basic Usage

A story consists of one or more **variant functions** annotated with
`#[variant]` and a **story constant** annotated with `#[story]`.

```rust
use holt_book::{story, variant};
use holt_kit::visual::Button;
use leptos::prelude::*;

#[variant]
fn default() -> AnyView {
    view! { <Button>"Click me!"</Button> }.into_any()
}

#[variant]
fn secondary() -> AnyView {
    view! {
        <Button variant=ButtonVariant::Secondary>"Click me!"</Button>
    }.into_any()
}

/// Buttons are for clicking and doing button things
#[story(id = "button", name = "Button")]
const BUTTON_STORY: () = &[default, secondary];
```

## `#[variant]`

Applied to a function that returns `AnyView`. Each variant function represents
one visual state of the component.

```rust
#[variant]
fn outline() -> AnyView {
    view! {
        <Button variant=ButtonVariant::Outline>"Click me!"</Button>
    }.into_any()
}
```

Variant functions take no arguments and must return `AnyView` (use `.into_any()`
on the `view!` result).

## `#[story]`

Applied to a `const` that references an array of variant functions. The macro
accepts these attributes:

| Attribute    | Required | Description                                      |
| ------------ | -------- | ------------------------------------------------ |
| `id`         | yes      | Unique identifier used in URLs and snapshots     |
| `name`       | yes      | Display name shown in the storybook sidebar      |
| `extra_docs` | no       | Additional documentation (typically source code) |

```rust
#[story(id = "button", name = "Button")]
const BUTTON_STORY: () = &[default, outline, destructive];
```

The const's **doc comment** becomes the story description displayed in the UI:

```rust
/// Buttons are for clicking and doing button things
#[story(id = "button", name = "Button")]
const BUTTON_STORY: () = &[default, outline];
```

## Source Code Display

Stories can optionally include their own source code in the UI. This uses a
build-time `include!` macro that pulls in a generated file:

```rust
include!(concat!(env!("OUT_DIR"), "/stories/button_source.rs"));

#[story(id = "button", name = "Button", extra_docs = BUTTON_SOURCE)]
const BUTTON_STORY: () = &[default, outline];
```

The `extra_docs` attribute references a constant produced by the build script.
This is an advanced feature used in the Holt Kit storybook itself — most stories
don't need it.

## Complete Example

From the Holt Kit storybook (`crates/kit-docs/src/stories/button.rs`):

```rust
use holt_book::{story, variant};
use holt_kit::visual::{Button, ButtonVariant};
use leptos::prelude::*;

#[variant]
fn default() -> AnyView {
    view! { <Button>"Click me!"</Button> }.into_any()
}

#[variant]
fn outline() -> AnyView {
    view! {
        <Button variant=ButtonVariant::Outline>"Click me!"</Button>
    }.into_any()
}

#[variant]
fn destructive() -> AnyView {
    view! {
        <Button variant=ButtonVariant::Destructive>"Click me!"</Button>
    }.into_any()
}

/// Buttons are for clicking and doing button things
#[story(id = "button", name = "Button")]
const BUTTON_STORY: () = &[default, outline, destructive];
```
