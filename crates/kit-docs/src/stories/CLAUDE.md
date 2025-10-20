# Component Stories

Stories showcase component variants for visual testing and documentation.

## Structure

```rust
// @component Button
use holt_book::{story, variant};
use holt_kit::visual::Button;
use leptos::prelude::*;

#[variant]
fn default() -> AnyView {
    view! { <Button>"Click me!"</Button> }.into_any()
}

#[variant]
fn outline() -> AnyView {
    view! { <Button variant=ButtonVariant::Outline>"Click me!"</Button> }.into_any()
}

include!(concat!(env!("OUT_DIR"), "/stories/button_source.rs"));

#[story(id = "button", name = "Button", extra_docs = BUTTON_SOURCE)]
const BUTTON_STORY: () = &[default, outline];
```

## Key Points

- `// @component Name` comment at top
- Each variant is a function returning `AnyView`
- `#[story]` defines story with id, name, and source code docs
- Story constant references all variant functions
- Source code auto-generated at build time

## Development

```bash
just kit-docs serve  # View at http://localhost:8080
```
