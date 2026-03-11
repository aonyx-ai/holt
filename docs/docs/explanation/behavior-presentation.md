---
sidebar_position: 1
---

# Behavior/Presentation Separation

Holt separates component behavior from presentation, following patterns
established by Radix UI.

## Why Separate Behavior and Presentation?

Traditional UI libraries couple how a component looks with how it works. This
creates problems:

- Changing styles risks breaking functionality
- Testing requires rendering full visual components
- Reusing behaviors means duplicating code

Holt solves this by splitting components into two layers:

1. **Behavior** - Handles state, keyboard navigation, ARIA attributes
2. **Presentation** - Handles visual styling with Tailwind CSS

## How It Works

### Behavior Layer

The behavior layer manages:

- Component state (open/closed, selected, focused)
- Keyboard interactions (arrow keys, Enter, Escape)
- Accessibility attributes (ARIA roles, labels, live regions)
- Focus management

This layer provides hooks and primitives that work regardless of styling.

### Presentation Layer

The presentation layer handles:

- Visual styling via Tailwind classes
- Variant-based appearance (primary, secondary, destructive)
- Size and spacing variations
- Animation and transitions

## Example

Here's how the Checkbox separates these concerns:

```rust
// Behavior layer (behavior/checkbox.rs): manages state, ARIA, and interactions
#[derive(Clone)]
pub struct CheckboxContext {
    pub checked: RwSignal<bool>,
    pub disabled: Signal<bool>,
    pub on_change: Option<Callback<bool>>,
}

#[component]
pub fn CheckboxRoot(
    #[prop(optional)] checked: RwSignal<bool>,
    #[prop(into, default = Signal::stored(false))] disabled: Signal<bool>,
    #[prop(optional_no_strip)] on_change: Option<Callback<bool>>,
    children: Children,
) -> impl IntoView {
    let context = CheckboxContext::new(checked, disabled, on_change);
    provide_context(context);

    view! {
        <button
            type="button"
            role="checkbox"
            aria-checked=move || checked.get()
            disabled=disabled
            on:click=move |_| context.toggle()
        >
            {children()}
        </button>
    }
}

// Presentation layer (visual/checkbox.rs): adds Tailwind styling
#[component]
pub fn Checkbox(
    #[prop(optional, into)] class: String,
    #[prop(optional)] size: CheckboxSize,
    #[prop(optional)] checked: RwSignal<bool>,
    // ...
) -> impl IntoView {
    let final_class = CheckboxStyle { size }.with_class(&class);

    view! {
        <CheckboxRoot checked=checked class=final_class>
            <CheckboxIndicator class="flex items-center justify-center">
                <Icon icon=icondata::LuCheck />
            </CheckboxIndicator>
        </CheckboxRoot>
    }
}
```

The behavior layer (`CheckboxRoot`) owns state, ARIA roles, and click handling.
The visual layer (`Checkbox`) wraps it with Tailwind classes and an icon. You
can use `CheckboxRoot` directly if you need completely custom presentation.

## Benefits

- **Customizable** - Swap out styling without touching behavior
- **Testable** - Test behaviors in isolation
- **Consistent** - Same interactions across different visual designs
- **Accessible** - Behaviors include accessibility by default
