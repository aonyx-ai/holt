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

Here's how a Button separates these concerns:

```rust
// Behavior: What the button does
pub struct ButtonBehavior {
    pub disabled: Signal<bool>,
    pub pressed: Signal<bool>,
}

// Presentation: How the button looks
#[component]
pub fn Button(
    #[prop(optional)] variant: ButtonVariant,
    #[prop(optional)] size: ButtonSize,
    children: Children,
) -> impl IntoView {
    // Behavior handles state and accessibility
    let behavior = use_button_behavior();

    // Presentation applies styles based on variant
    let classes = button_styles(variant, size);

    view! {
        <button
            class=classes
            disabled=behavior.disabled
            aria-pressed=behavior.pressed
        >
            {children()}
        </button>
    }
}
```

## Benefits

- **Customizable** - Swap out styling without touching behavior
- **Testable** - Test behaviors in isolation
- **Consistent** - Same interactions across different visual designs
- **Accessible** - Behaviors include accessibility by default
