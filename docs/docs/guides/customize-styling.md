---
sidebar_position: 1
---

# Customize Component Styling

Holt components are designed for customization. This guide covers three
approaches, from quick overrides to full component modifications.

## Override with the `class` Prop

Every Holt component accepts a `class` prop for adding or overriding styles:

```rust
use crate::components::*;

view! {
    // Add rounded corners
    <Button class="rounded-full">"Rounded Button"</Button>

    // Add shadow
    <Card class="shadow-xl">"Elevated Card"</Card>

    // Override padding
    <CardContent class="p-8">"Extra Padding"</CardContent>
}
```

Classes are merged with `tailwind_fuse`, which handles conflicts intelligently.
Your custom classes take precedence when there's overlap:

```rust
// Base button has `rounded-md`, but `rounded-full` wins
<Button class="rounded-full">"Pill Button"</Button>
```

## Modify Component Source

For deeper changes, copy the component into your project and modify it directly.

### 1. Copy the Component

Find the component source in Holt's repository and copy it to your project:

```
your-project/
└── src/
    └── components/
        └── button.rs  // Your modified copy
```

### 2. Modify the Styles

Edit the base classes to match your design:

```rust
// Original
let base_classes = "inline-flex items-center justify-center rounded-md text-sm font-medium";

// Your modification
let base_classes = "inline-flex items-center justify-center rounded-xl text-base font-bold";
```

### 3. Use Your Version

Import your modified component instead of Holt's:

```rust
use crate::components::button::Button;
```

## Add Custom Variants

Extend components with new variant options.

### 1. Define the Variant

Add a new enum variant:

```rust
#[derive(Default, Clone, Copy)]
pub enum ButtonVariant {
    #[default]
    Default,
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
    // Your new variant
    Gradient,
}
```

### 2. Add Variant Styles

Update the style function:

```rust
fn button_variant_classes(variant: ButtonVariant) -> &'static str {
    match variant {
        ButtonVariant::Default => "bg-primary text-primary-foreground hover:bg-primary/90",
        ButtonVariant::Destructive => "bg-destructive text-destructive-foreground hover:bg-destructive/90",
        ButtonVariant::Outline => "border border-input bg-background hover:bg-accent hover:text-accent-foreground",
        ButtonVariant::Secondary => "bg-secondary text-secondary-foreground hover:bg-secondary/80",
        ButtonVariant::Ghost => "hover:bg-accent hover:text-accent-foreground",
        ButtonVariant::Link => "text-primary underline-offset-4 hover:underline",
        // New variant
        ButtonVariant::Gradient => "bg-gradient-to-r from-purple-500 to-pink-500 text-white hover:from-purple-600 hover:to-pink-600",
    }
}
```

### 3. Use the New Variant

```rust
<Button variant=ButtonVariant::Gradient>"Gradient Button"</Button>
```

## Tips

- **Start with `class` prop** - It handles most customization needs without
  copying code
- **Copy components sparingly** - Only copy when you need structural changes,
  not just style tweaks
- **Keep variants semantic** - Name variants by purpose (`Gradient`), not
  appearance (`PurplePink`)
- **Document your changes** - When you copy and modify a component, note what
  you changed and why
