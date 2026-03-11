---
sidebar_position: 2
---

# Set Up Dark Mode

Holt components support dark mode through Tailwind CSS. This guide shows how to
implement a theme toggle.

## CSS Variables for Theming

Define your color palette with CSS variables using OKLCH values. These go in
your `styles.css` file alongside the Tailwind import:

```css
@import "tailwindcss" source("../../");

@custom-variant dark (&:is(.dark *));

:root {
  --background: oklch(1 0 0);
  --foreground: oklch(0.145 0 0);
  --card: oklch(1 0 0);
  --card-foreground: oklch(0.145 0 0);
  --primary: oklch(0.205 0 0);
  --primary-foreground: oklch(0.985 0 0);
  --secondary: oklch(0.97 0 0);
  --secondary-foreground: oklch(0.205 0 0);
  --muted: oklch(0.97 0 0);
  --muted-foreground: oklch(0.556 0 0);
  --accent: oklch(0.97 0 0);
  --accent-foreground: oklch(0.205 0 0);
  --destructive: oklch(0.577 0.245 27.325);
  --destructive-foreground: oklch(0.985 0 0);
  --border: oklch(0.922 0 0);
  --input: oklch(0.922 0 0);
  --ring: oklch(0.708 0 0);
  --radius: 0.625rem;
}

.dark {
  --background: oklch(0.145 0 0);
  --foreground: oklch(0.985 0 0);
  --card: oklch(0.145 0 0);
  --card-foreground: oklch(0.985 0 0);
  --primary: oklch(0.985 0 0);
  --primary-foreground: oklch(0.205 0 0);
  --secondary: oklch(0.269 0 0);
  --secondary-foreground: oklch(0.985 0 0);
  --muted: oklch(0.269 0 0);
  --muted-foreground: oklch(0.708 0 0);
  --accent: oklch(0.269 0 0);
  --accent-foreground: oklch(0.985 0 0);
  --destructive: oklch(0.396 0.141 25.723);
  --destructive-foreground: oklch(0.985 0 0);
  --border: oklch(0.269 0 0);
  --input: oklch(0.269 0 0);
  --ring: oklch(0.439 0 0);
}
```

## Tailwind Theme Mapping

Map the CSS variables to Tailwind's theme using `@theme inline`:

```css
@theme inline {
  --color-background: var(--background);
  --color-foreground: var(--foreground);
  --color-card: var(--card);
  --color-card-foreground: var(--card-foreground);
  --color-primary: var(--primary);
  --color-primary-foreground: var(--primary-foreground);
  --color-secondary: var(--secondary);
  --color-secondary-foreground: var(--secondary-foreground);
  --color-muted: var(--muted);
  --color-muted-foreground: var(--muted-foreground);
  --color-accent: var(--accent);
  --color-accent-foreground: var(--accent-foreground);
  --color-destructive: var(--destructive);
  --color-destructive-foreground: var(--destructive-foreground);
  --color-border: var(--border);
  --color-input: var(--input);
  --color-ring: var(--ring);
  --radius-sm: calc(var(--radius) - 4px);
  --radius-md: calc(var(--radius) - 2px);
  --radius-lg: var(--radius);
  --radius-xl: calc(var(--radius) + 4px);
}

@layer base {
  * {
    @apply border-border outline-ring/50;
  }

  body {
    @apply bg-background text-foreground;
  }
}
```

This uses Tailwind v4's CSS-first configuration — no `tailwind.config.js`
needed.

## Theme Toggle Component

Create a toggle button to switch themes:

```rust
use leptos::prelude::*;
use crate::components::*;

#[component]
pub fn ThemeToggle() -> impl IntoView {
    let toggle_theme = move |_| {
        // Access the document and toggle the dark class
        if let Some(document) = web_sys::window()
            .and_then(|w| w.document())
        {
            if let Some(html) = document.document_element() {
                let class_list = html.class_list();
                let _ = class_list.toggle("dark");
            }
        }
    };

    view! {
        <Button
            variant=ButtonVariant::Ghost
            on:click=toggle_theme
        >
            // Sun icon for light mode, moon for dark
            <span class="dark:hidden">"🌙"</span>
            <span class="hidden dark:inline">"☀️"</span>
        </Button>
    }
}
```

## Persist User Preference

Store the user's theme choice in local storage:

```rust
use leptos::prelude::*;

#[component]
pub fn ThemeProvider(children: Children) -> impl IntoView {
    // Initialize theme from localStorage or system preference
    Effect::new(move || {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let stored = storage.get_item("theme").ok().flatten();

                let should_be_dark = match stored.as_deref() {
                    Some("dark") => true,
                    Some("light") => false,
                    _ => {
                        // Fall back to system preference
                        window
                            .match_media("(prefers-color-scheme: dark)")
                            .ok()
                            .flatten()
                            .map(|mq| mq.matches())
                            .unwrap_or(false)
                    }
                };

                if let Some(document) = window.document() {
                    if let Some(html) = document.document_element() {
                        let class_list = html.class_list();
                        if should_be_dark {
                            let _ = class_list.add_1("dark");
                        } else {
                            let _ = class_list.remove_1("dark");
                        }
                    }
                }
            }
        }
    });

    children()
}
```

Update the toggle to save the preference:

```rust
let toggle_theme = move |_| {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if let Some(html) = document.document_element() {
                let class_list = html.class_list();
                let is_dark = class_list.contains("dark");

                if is_dark {
                    let _ = class_list.remove_1("dark");
                    if let Ok(Some(storage)) = window.local_storage() {
                        let _ = storage.set_item("theme", "light");
                    }
                } else {
                    let _ = class_list.add_1("dark");
                    if let Ok(Some(storage)) = window.local_storage() {
                        let _ = storage.set_item("theme", "dark");
                    }
                }
            }
        }
    }
};
```

## Tips

- **Use CSS variables** - They enable smooth transitions between themes
- **Test both modes** - Components should be readable in both light and dark
- **Consider system preference** - Respect `prefers-color-scheme` as a default
- **Avoid flash** - Initialize theme before rendering to prevent flash of wrong
  theme
