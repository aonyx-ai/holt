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
use leptos::prelude::*;
use crate::components::{Card, CardHeader, CardTitle, CardContent};

#[variant]
fn default() -> AnyView {
    view! {
        <Card>
            <CardHeader><CardTitle>"Hello"</CardTitle></CardHeader>
            <CardContent><p>"Card content goes here."</p></CardContent>
        </Card>
    }.into_any()
}

#[variant]
fn compact() -> AnyView {
    view! {
        <Card class="w-64">
            <CardContent><p>"A compact card."</p></CardContent>
        </Card>
    }.into_any()
}

/// A container for grouping related content
#[story(id = "card", name = "Card")]
const CARD_STORY: () = &[default, compact];
```

## `#[variant]`

Applied to a function that returns `AnyView`. Each variant function represents
one visual state of the component.

```rust
#[variant]
fn minimal() -> AnyView {
    view! {
        <Card>
            <CardContent><p>"Just content, no header."</p></CardContent>
        </Card>
    }.into_any()
}
```

Variant functions take no arguments and must return `AnyView` (use `.into_any()`
on the `view!` result).

## `#[story]`

Applied to a `const` that references an array of variant functions. The macro
accepts these attributes:

| Attribute    | Required | Description                                                 |
| ------------ | -------- | ----------------------------------------------------------- |
| `id`         | yes      | Unique identifier used in URLs and snapshots                |
| `name`       | yes      | Display name shown in the storybook sidebar                 |
| `extra_docs` | no       | Additional Markdown documentation (`&'static str` constant) |

```rust
#[story(id = "card", name = "Card")]
const CARD_STORY: () = &[default, compact, minimal];
```

The const's **doc comment** becomes the story description displayed in the UI:

```rust
/// A container for grouping related content
#[story(id = "card", name = "Card")]
const CARD_STORY: () = &[default, compact];
```

## Extra Documentation

Stories can include additional documentation displayed in the UI via the
`extra_docs` attribute. This accepts a reference to a `&'static str` constant
containing any Markdown content — usage notes, design rationale, API details, or
anything else you want to show alongside the component.

```rust
const CARD_DOCS: &str = "
## Design Notes

Cards should always have a minimum width of 200px.
Use `CardHeader` for titles and `CardContent` for body text.
";

#[story(id = "card", name = "Card", extra_docs = CARD_DOCS)]
const CARD_STORY: () = &[default, compact];
```

The constant can come from anywhere — a literal in your source, an
`include_str!` of a Markdown file, or a build-script-generated value. Holt Kit's
own storybook uses this to embed source code, but the feature is
general-purpose.

## Complete Example

A complete story file for a Card component:

```rust
use holt_book::{story, variant};
use leptos::prelude::*;
use crate::components::{Card, CardHeader, CardTitle, CardContent};

#[variant]
fn default() -> AnyView {
    view! {
        <Card>
            <CardHeader><CardTitle>"Hello"</CardTitle></CardHeader>
            <CardContent><p>"Card content goes here."</p></CardContent>
        </Card>
    }.into_any()
}

#[variant]
fn compact() -> AnyView {
    view! {
        <Card class="w-64">
            <CardContent><p>"A compact card."</p></CardContent>
        </Card>
    }.into_any()
}

#[variant]
fn minimal() -> AnyView {
    view! {
        <Card>
            <CardContent><p>"Just content, no header."</p></CardContent>
        </Card>
    }.into_any()
}

/// A container for grouping related content
#[story(id = "card", name = "Card")]
const CARD_STORY: () = &[default, compact, minimal];
```
