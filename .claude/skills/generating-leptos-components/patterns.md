# Code Translation Patterns

This guide shows how to translate common React patterns used in Shadcn/Radix
components into idiomatic Leptos/Rust code.

## Imports

```rust
// Import from behavior module (items are re-exported from mod.rs)
use crate::behavior::{SwitchRoot, SwitchThumb};
// NOT: use crate::behavior::switch::{...}  // ❌ module is private

// Standard imports
use leptos::prelude::*;
use tailwind_fuse::*;
```

## Props and Attributes

```rust
// Optional props
#[prop(optional)] class: &'static str

// Optional props with into (handles type conversions)
#[prop(optional, into)] class: Option<String>
// Pass directly: class=my_string (NOT class=Some(my_string))

// Optional props with strip
#[prop(optional_no_strip, into)] id: Option<&'static str>

// Default values via variants
#[prop(optional)] variant: ButtonVariant  // Uses #[tw(default)]

// Required props
value: String

// Children
children: Children
```

### Rust Keywords in Attributes

```rust
// HTML 'for' attribute conflicts with Rust keyword
// Use raw identifier syntax:
view! {
    <Label r#for="input-id">"Label"</Label>
}
// NOT: for_="input-id" ❌
```

## State Management

```rust
// Signals for reactive state
let (checked, set_checked) = signal(false);

// Derived signals
let is_open = move || state.get().is_open;

// Effects for side effects
Effect::new(move |_| {
    if open.get() {
        // Do something
    }
});
```

## Event Handlers

```rust
// Click handlers
let on_click = move |_| {
    set_checked.update(|c| *c = !*c);
};

view! { <button on:click=on_click>Click me</button> }

// Keyboard handlers
on:keydown=move |e| {
    if e.key() == "Enter" {
        // Handle Enter
    }
}
```

## Context

```rust
// Provide context
#[component]
pub fn SelectRoot(children: Children) -> impl IntoView {
    let context = SelectContext::new();
    provide_context(context);

    view! {
        {children()}
    }
}

// Consume context
pub fn use_select() -> SelectContext {
    use_context::<SelectContext>()
        .expect("use_select must be called within SelectRoot")
}
```

### Context with Multiple Closures

When using context in multiple event handlers, clone for each closure:

```rust
let context = SwitchContext::new(checked, disabled);
let context_keydown = context.clone();  // Clone for keydown handler
let context_click = context.clone();    // Clone for click handler
provide_context(context);

let handle_keydown = move |e: KeyboardEvent| {
    context_keydown.toggle();  // Uses its own clone
};

view! {
    <button
        on:click=move |_| context_click.toggle()  // Uses separate clone
        on:keydown=handle_keydown
    >
}
```

## Common Component Patterns

### Dialog/Modal Components

- Use behavior module for open/close state and focus management
- Implement portal rendering for overlay
- Handle escape key and backdrop click
- Manage focus trap
- Style with z-index layers and animations

### Form Components (Input, Select, Checkbox)

- Support controlled and uncontrolled modes
- Implement proper ARIA attributes
- Handle validation states
- Style focus and error states
- Support form integration

### Compound Components (Select, Accordion)

- Use context to share state between sub-components
- Export all parts as separate components
- Allow composition and customization
- Document usage patterns in stories
