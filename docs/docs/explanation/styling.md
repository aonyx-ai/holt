---
sidebar_position: 2
---

# Styling

Holt uses Tailwind CSS via `tailwind_fuse` for type-safe, composable styling.

## Tailwind Fuse

[tailwind_fuse](https://github.com/gaucho-labs/tailwind-fuse) provides:

- Type-safe class composition
- Automatic conflict resolution
- Variant-based styling
- Zero runtime CSS-in-Rust overhead

## Variant System

Components use a variant system for consistent styling:

```rust
use holt_kit::prelude::*;

// Different visual variants
view! {
    <Button variant=ButtonVariant::Primary>"Primary"</Button>
    <Button variant=ButtonVariant::Secondary>"Secondary"</Button>
    <Button variant=ButtonVariant::Destructive>"Destructive"</Button>
    <Button variant=ButtonVariant::Ghost>"Ghost"</Button>
}
```

## Size Variants

Components support size variations:

```rust
view! {
    <Button size=ButtonSize::Sm>"Small"</Button>
    <Button size=ButtonSize::Md>"Medium"</Button>
    <Button size=ButtonSize::Lg>"Large"</Button>
}
```

## Custom Classes

Override or extend styling with the `class` prop:

```rust
view! {
    <Button class="rounded-full">"Rounded"</Button>
    <Button class="shadow-lg">"With Shadow"</Button>
}
```

Classes are merged intelligently - your custom classes take precedence over
defaults when there are conflicts.

## Theme Customization

Holt follows Tailwind conventions for theming:

1. **Colors** - Use CSS variables for theme colors
2. **Spacing** - Standard Tailwind spacing scale
3. **Typography** - System font stack by default

### Dark Mode

Components automatically support dark mode via Tailwind's `dark:` variant:

```css
/* Your global styles */
:root {
  --primary: theme("colors.indigo.600");
}

.dark {
  --primary: theme("colors.indigo.400");
}
```

## Tailwind Configuration

Add Holt to your Tailwind content paths:

```js
// tailwind.config.js
module.exports = {
  content: [
    "./src/**/*.rs",
    // Include Holt's component styles
    "./node_modules/holt-kit/**/*.rs",
  ],
  // ...
};
```
