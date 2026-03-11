---
sidebar_position: 1
---

# Your First Storybook

This tutorial walks you through creating a component storybook using Holt Book.
By the end, you'll have an interactive component showcase running in your
browser.

## Prerequisites

- Rust 1.88 or later
- A Leptos project (or we'll create one)
- Basic familiarity with Leptos components

## Step 1: Create a Leptos Project

If you don't have an existing project, create one:

```bash
cargo new my-components
cd my-components
```

Add dependencies to your `Cargo.toml`:

```toml
[dependencies]
leptos = "0.8"
holt-book = "0.1"
tailwind_fuse = "0.3"
```

Holt doesn't ship a pre-built component library — you create your own components
and use Holt Book to showcase them. This is the Shadcn model: you own and
customize every component.

## Step 2: Set Up the Storybook Structure

Create the basic directory structure for your storybook:

```
my-components/
├── src/
│   ├── lib.rs
│   └── components/
│       └── mod.rs
└── stories/
    └── mod.rs
```

## Step 3: Create a Component

Let's build a simple Card component using Tailwind CSS via `tailwind_fuse`.

Create `src/components/card.rs`:

```rust
use leptos::prelude::*;
use tailwind_fuse::tw_merge;

/// Card component with optional header and footer
#[component]
pub fn Card(
    #[prop(optional)] class: &'static str,
    children: Children,
) -> impl IntoView {
    let base_classes = "rounded-lg border bg-card text-card-foreground shadow-sm";

    view! {
        <div class=tw_merge!(base_classes, class)>
            {children()}
        </div>
    }
}

#[component]
pub fn CardHeader(
    #[prop(optional)] class: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=tw_merge!("flex flex-col space-y-1.5 p-6", class)>
            {children()}
        </div>
    }
}

#[component]
pub fn CardTitle(
    #[prop(optional)] class: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <h3 class=tw_merge!("text-2xl font-semibold leading-none tracking-tight", class)>
            {children()}
        </h3>
    }
}

#[component]
pub fn CardContent(
    #[prop(optional)] class: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=tw_merge!("p-6 pt-0", class)>
            {children()}
        </div>
    }
}
```

Export it from `src/components/mod.rs`:

```rust
mod card;
pub use card::*;
```

## Step 4: Write a Story

Stories showcase your component in different states. Create `stories/card.rs`:

```rust
use holt_book::{story, variant};
use crate::components::*;
use leptos::prelude::*;

#[variant]
fn default() -> AnyView {
    view! {
        <Card>
            <CardHeader>
                <CardTitle>"Card Title"</CardTitle>
            </CardHeader>
            <CardContent>
                <p>"Card content goes here."</p>
            </CardContent>
        </Card>
    }.into_any()
}

#[variant]
fn fixed_width() -> AnyView {
    view! {
        <Card class="w-96">
            <CardHeader>
                <CardTitle>"Fixed Width Card"</CardTitle>
            </CardHeader>
            <CardContent>
                <p>"This card has a fixed width of 24rem."</p>
            </CardContent>
        </Card>
    }.into_any()
}

#[variant]
fn minimal() -> AnyView {
    view! {
        <Card>
            <CardContent>
                <p>"A card with just content, no header."</p>
            </CardContent>
        </Card>
    }.into_any()
}

/// A container for grouping related content
#[story(id = "card", name = "Card")]
const CARD_STORY: () = &[default, fixed_width, minimal];
```

Stories are registered automatically — no manual registration step needed. Just
make sure the module is included somewhere in your crate so the `#[story]` macro
can run.

## Step 5: Configure and Run the Storybook

If your storybook is in a subdirectory (like a workspace), create a `holt.toml`
at your project root:

```toml
[book]
path = "path/to/your/storybook"

[serve]
port = 3000
open = true
```

Then start the development server:

```bash
holt serve
```

Without a config file, Holt runs in the current directory on port 8080. You can
also override settings via flags:

```bash
holt serve --port 3000 --open
```

Open your browser to the configured port. You'll see your Card component with
all its variants in the sidebar.

## Step 6: Iterate

With the server running, edit your component or stories. Changes appear
automatically thanks to hot reloading.

Try adding a new variant to your story:

```rust
#[variant]
fn highlighted() -> AnyView {
    view! {
        <Card class="border-primary">
            <CardHeader>
                <CardTitle>"Highlighted Card"</CardTitle>
            </CardHeader>
            <CardContent>
                <p>"This card has a highlighted border."</p>
            </CardContent>
        </Card>
    }.into_any()
}
```

Then add `highlighted` to your story's variant array:

```rust
#[story(id = "card", name = "Card")]
const CARD_STORY: () = &[default, fixed_width, minimal, highlighted];
```

The new variant appears in the storybook automatically.

## Next Steps

- Read the [Styling guide](/docs/guides/customize-styling) to learn about
  customization
- Check the [Story Macro reference](/docs/reference/story-macro) for advanced
  story options
- Explore the [Architecture explanation](/docs/explanation/architecture) to
  understand why Holt works this way
