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
use crate::components::*;

// Different visual variants
view! {
    <Button>"Default"</Button>
    <Button variant=ButtonVariant::Secondary>"Secondary"</Button>
    <Button variant=ButtonVariant::Destructive>"Destructive"</Button>
    <Button variant=ButtonVariant::Outline>"Outline"</Button>
    <Button variant=ButtonVariant::Ghost>"Ghost"</Button>
    <Button variant=ButtonVariant::Link>"Link"</Button>
}
```

## Size Variants

Components support size variations:

```rust
view! {
    <Button size=ButtonSize::Sm>"Small"</Button>
    <Button>"Default"</Button>
    <Button size=ButtonSize::Lg>"Large"</Button>
    <Button size=ButtonSize::Icon>"🔍"</Button>
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

Since your components live in your own crate, Tailwind just needs to scan your
source files. With Tailwind v4 and Trunk, use `rel="tailwind-css"` in your
`index.html` and Tailwind will auto-detect classes from your Rust sources:

```html
<link data-trunk rel="tailwind-css" href="public/styles.css" />
```
