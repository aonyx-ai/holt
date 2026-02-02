---
sidebar_position: 2
---

# Story Macro

The `#[story]` attribute macro registers Leptos components as stories in Holt
Book.

## Basic Usage

Apply `#[story]` to a function that returns `impl IntoView`:

```rust
use holt_book::prelude::*;
use leptos::prelude::*;

#[story]
pub fn ButtonDefault() -> impl IntoView {
    view! {
        <Button>"Click me"</Button>
    }
}
```

The function name becomes the story name in the UI, converted from `PascalCase`
to a readable format (e.g., `ButtonDefault` becomes "Button Default").

## Story Metadata

Add metadata attributes to customize how stories appear:

### `name`

Override the display name:

```rust
#[story(name = "Primary Button")]
pub fn ButtonPrimary() -> impl IntoView {
    view! {
        <Button variant=ButtonVariant::Primary>"Primary"</Button>
    }
}
```

### `description`

Add a description shown in the story panel:

```rust
#[story(description = "The default button style used for most actions")]
pub fn ButtonDefault() -> impl IntoView {
    view! {
        <Button>"Default"</Button>
    }
}
```

### `category`

Group stories by category:

```rust
#[story(category = "Forms")]
pub fn InputText() -> impl IntoView {
    view! {
        <Input placeholder="Enter text..." />
    }
}

#[story(category = "Forms")]
pub fn InputPassword() -> impl IntoView {
    view! {
        <Input type_="password" placeholder="Password" />
    }
}
```

Stories with the same category appear together in the sidebar.

### Combining Attributes

Attributes can be combined:

```rust
#[story(
    name = "Destructive Action",
    category = "Buttons",
    description = "Use for dangerous actions like delete"
)]
pub fn ButtonDestructive() -> impl IntoView {
    view! {
        <Button variant=ButtonVariant::Destructive>"Delete"</Button>
    }
}
```

## Multiple Variants

Show multiple variants in a single story:

```rust
#[story(name = "All Variants")]
pub fn ButtonVariants() -> impl IntoView {
    view! {
        <div class="flex gap-4">
            <Button variant=ButtonVariant::Primary>"Primary"</Button>
            <Button variant=ButtonVariant::Secondary>"Secondary"</Button>
            <Button variant=ButtonVariant::Destructive>"Destructive"</Button>
            <Button variant=ButtonVariant::Ghost>"Ghost"</Button>
        </div>
    }
}
```

## Interactive Stories

Stories can include state and interactivity:

```rust
#[story(name = "Counter Button")]
pub fn ButtonCounter() -> impl IntoView {
    let count = RwSignal::new(0);

    view! {
        <Button on:click=move |_| count.update(|n| *n += 1)>
            "Clicked: " {count}
        </Button>
    }
}
```

## Registering Stories

Stories must be registered to appear in the storybook. Create a registration
function:

```rust
// stories/mod.rs
mod button;
mod card;
mod input;

pub use button::*;
pub use card::*;
pub use input::*;

use holt_book::prelude::*;

pub fn register_stories() -> Stories {
    stories![
        // Button stories
        ButtonDefault,
        ButtonPrimary,
        ButtonVariants,
        ButtonCounter,

        // Card stories
        CardDefault,
        CardWithHeader,

        // Input stories
        InputText,
        InputPassword,
    ]
}
```

The `stories!` macro collects stories into a `Stories` collection.

## Story Context

Access the story context for advanced use cases:

```rust
#[story]
pub fn ResponsiveComponent() -> impl IntoView {
    let ctx = use_story_context();

    view! {
        <div class="p-4" style:width=ctx.viewport_width>
            <Card>"Responsive content"</Card>
        </div>
    }
}
```

Context provides:

- `viewport_width` - Current viewport width
- `viewport_height` - Current viewport height
- `dark_mode` - Whether dark mode is active

## Generated Code

The `#[story]` macro generates:

1. A struct implementing the `Story` trait
2. Registration in the story registry
3. Metadata accessors

For a story like:

```rust
#[story(name = "My Button", category = "Buttons")]
pub fn ButtonExample() -> impl IntoView {
    view! { <Button>"Example"</Button> }
}
```

The macro generates approximately:

```rust
pub struct ButtonExample;

impl Story for ButtonExample {
    fn name(&self) -> &'static str {
        "My Button"
    }

    fn category(&self) -> Option<&'static str> {
        Some("Buttons")
    }

    fn render(&self) -> impl IntoView {
        view! { <Button>"Example"</Button> }
    }
}
```
