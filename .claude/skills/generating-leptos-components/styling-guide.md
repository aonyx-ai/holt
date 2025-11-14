# Styling with Tailwind Fuse

This guide explains how to use `tailwind_fuse` for managing Tailwind CSS classes
in Holt components.

## For Simple Components (no variants)

Use `tw_merge!` macro for base classes + user overrides:

```rust
use tailwind_fuse::tw_merge;

#[component]
pub fn Card(
    #[prop(optional)] class: &'static str,
    children: Children
) -> impl IntoView {
    let classes = tw_merge!(
        "rounded-lg border bg-card text-card-foreground shadow-sm",
        class
    );
    view! { <div class=classes>{children()}</div> }
}
```

**Use `tw_merge!` for**: Card, Label, Separator, Breadcrumb items, etc.

## For Components with Variants

Use `#[derive(TwClass)]` and `#[derive(TwVariant)]`:

**Step 1**: Define style struct with base classes:

```rust
use tailwind_fuse::*;

#[derive(TwClass)]
#[tw(
    class = "inline-flex items-center justify-center rounded-md font-medium transition-colors focus-visible:outline-none disabled:pointer-events-none disabled:opacity-50"
)]
struct ButtonStyle {
    variant: ButtonVariant,
    size: ButtonSize,
}
```

**Step 2**: Define variant enums:

```rust
#[derive(TwVariant)]
pub enum ButtonVariant {
    #[tw(default, class = "bg-primary text-primary-foreground hover:bg-primary/90")]
    Default,
    #[tw(class = "bg-destructive text-destructive-foreground hover:bg-destructive/90")]
    Destructive,
    #[tw(class = "border border-input bg-background hover:bg-accent hover:text-accent-foreground")]
    Outline,
    #[tw(class = "bg-secondary text-secondary-foreground hover:bg-secondary/80")]
    Secondary,
    #[tw(class = "hover:bg-accent hover:text-accent-foreground")]
    Ghost,
}

#[derive(TwVariant)]
pub enum ButtonSize {
    #[tw(default, class = "h-10 px-4 py-2")]
    Default,
    #[tw(class = "h-8 px-3 py-1")]
    Sm,
    #[tw(class = "h-12 px-6 py-3")]
    Lg,
    #[tw(class = "h-10 w-10 p-2")]
    Icon,
}
```

**Step 3**: Use in component with `.with_class()`:

```rust
#[component]
pub fn Button(
    #[prop(optional)] class: &'static str,
    #[prop(optional)] variant: ButtonVariant,
    #[prop(optional)] size: ButtonSize,
    children: Children,
) -> impl IntoView {
    let final_class = ButtonStyle { variant, size }.with_class(class);
    view! { <button class=final_class>{children()}</button> }
}
```

**Use `TwClass` + `TwVariant` for**: Button, Badge, Input, Textarea, Checkbox,
Switch, Toggle, etc.

## Styling Guidelines

- **Always provide a `class` prop** for user customization
- Mark one variant as `#[tw(default, class = "...")]`
- Use `.with_class(class)` to merge user classes with component styles
- Follow Tailwind's utility-first approach
- Use semantic color tokens (e.g., `bg-primary`, `text-muted-foreground`)
- Include responsive classes when appropriate
- Add focus-visible states for accessibility
- Include disabled states where applicable

## Size Variant Coordination

When components have moving parts (like switch thumbs, slider handles),
coordinate sizes carefully:

```rust
// Switch example: thumb translation must match container width
#[derive(TwVariant)]
pub enum SwitchSize {
    #[tw(default, class = "h-6 w-11")]  // Container: width 11 (44px)
    Default,
    #[tw(class = "h-5 w-9")]  // Container: width 9 (36px)
    Sm,
    #[tw(class = "h-7 w-14")]  // Container: width 14 (56px)
    Lg,
}

// Thumb must translate by: container_width - thumb_width
let thumb_class = match size {
    // Sm: translate-x-4 (9 - 4 = thumb fits on right)
    SwitchSize::Sm => "h-4 w-4 data-[state=checked]:translate-x-4",
    // Default: translate-x-5 (11 - 5 = thumb fits on right)
    SwitchSize::Default => "h-5 w-5 data-[state=checked]:translate-x-5",
    // Lg: translate-x-7 (14 - 6 = thumb fits on right)
    SwitchSize::Lg => "h-6 w-6 data-[state=checked]:translate-x-7",
};
```

**Key principle**: Moving elements need math! Container size - element size =
movement distance.

## Data Attributes for State-Based Styling

Use `data-*` attributes set by behavior components to style states:

```rust
// Behavior component sets the attribute
data-state=move || if checked.get() { "checked" } else { "unchecked" }

// Tailwind styles based on attribute
class = "data-[state=checked]:bg-primary data-[state=unchecked]:bg-input"
```

Common data attributes:

- `data-state`: "open" | "closed" | "checked" | "unchecked"
- `data-disabled`: Present when disabled
- `data-selected`: Present when selected
- `data-orientation`: "horizontal" | "vertical"
