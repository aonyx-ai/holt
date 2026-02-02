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
holt-kit = "0.1"
holt = "0.1"
```

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

Let's build a simple Card component following Holt's behavior/presentation
pattern.

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
use holt_book::prelude::*;
use crate::components::*;

#[story]
pub fn CardDefault() -> impl IntoView {
    view! {
        <Card>
            <CardHeader>
                <CardTitle>"Card Title"</CardTitle>
            </CardHeader>
            <CardContent>
                <p>"Card content goes here."</p>
            </CardContent>
        </Card>
    }
}

#[story]
pub fn CardWithCustomClass() -> impl IntoView {
    view! {
        <Card class="w-96">
            <CardHeader>
                <CardTitle>"Fixed Width Card"</CardTitle>
            </CardHeader>
            <CardContent>
                <p>"This card has a fixed width of 24rem."</p>
            </CardContent>
        </Card>
    }
}

#[story]
pub fn CardMinimal() -> impl IntoView {
    view! {
        <Card>
            <CardContent>
                <p>"A card with just content, no header."</p>
            </CardContent>
        </Card>
    }
}
```

Register stories in `stories/mod.rs`:

```rust
mod card;
pub use card::*;

use holt_book::prelude::*;

pub fn register_stories() -> Stories {
    stories![
        CardDefault,
        CardWithCustomClass,
        CardMinimal,
    ]
}
```

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

Try adding a new variant:

```rust
#[story]
pub fn CardHighlighted() -> impl IntoView {
    view! {
        <Card class="border-primary">
            <CardHeader>
                <CardTitle>"Highlighted Card"</CardTitle>
            </CardHeader>
            <CardContent>
                <p>"This card has a highlighted border."</p>
            </CardContent>
        </Card>
    }
}
```

The new story appears in the sidebar immediately.

## Next Steps

- Read the [Styling guide](/docs/guides/customize-styling) to learn about
  customization
- Check the [Story Macro reference](/docs/reference/story-macro) for advanced
  story options
- Explore the [Architecture explanation](/docs/explanation/architecture) to
  understand why Holt works this way
