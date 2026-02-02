---
sidebar_position: 2
---

# Set Up Dark Mode

Holt components support dark mode through Tailwind CSS. This guide shows how to
implement a theme toggle.

## CSS Variables for Theming

Define your color palette with CSS variables:

```css
/* styles/globals.css */
:root {
  --background: 0 0% 100%;
  --foreground: 222.2 84% 4.9%;

  --card: 0 0% 100%;
  --card-foreground: 222.2 84% 4.9%;

  --primary: 222.2 47.4% 11.2%;
  --primary-foreground: 210 40% 98%;

  --secondary: 210 40% 96.1%;
  --secondary-foreground: 222.2 47.4% 11.2%;

  --muted: 210 40% 96.1%;
  --muted-foreground: 215.4 16.3% 46.9%;

  --accent: 210 40% 96.1%;
  --accent-foreground: 222.2 47.4% 11.2%;

  --destructive: 0 84.2% 60.2%;
  --destructive-foreground: 210 40% 98%;

  --border: 214.3 31.8% 91.4%;
  --input: 214.3 31.8% 91.4%;
  --ring: 222.2 84% 4.9%;

  --radius: 0.5rem;
}

.dark {
  --background: 222.2 84% 4.9%;
  --foreground: 210 40% 98%;

  --card: 222.2 84% 4.9%;
  --card-foreground: 210 40% 98%;

  --primary: 210 40% 98%;
  --primary-foreground: 222.2 47.4% 11.2%;

  --secondary: 217.2 32.6% 17.5%;
  --secondary-foreground: 210 40% 98%;

  --muted: 217.2 32.6% 17.5%;
  --muted-foreground: 215 20.2% 65.1%;

  --accent: 217.2 32.6% 17.5%;
  --accent-foreground: 210 40% 98%;

  --destructive: 0 62.8% 30.6%;
  --destructive-foreground: 210 40% 98%;

  --border: 217.2 32.6% 17.5%;
  --input: 217.2 32.6% 17.5%;
  --ring: 212.7 26.8% 83.9%;
}
```

## Tailwind Configuration

Configure Tailwind to use your CSS variables:

```js
// tailwind.config.js
module.exports = {
  darkMode: "class",
  content: ["./src/**/*.rs"],
  theme: {
    extend: {
      colors: {
        background: "hsl(var(--background))",
        foreground: "hsl(var(--foreground))",
        card: {
          DEFAULT: "hsl(var(--card))",
          foreground: "hsl(var(--card-foreground))",
        },
        primary: {
          DEFAULT: "hsl(var(--primary))",
          foreground: "hsl(var(--primary-foreground))",
        },
        secondary: {
          DEFAULT: "hsl(var(--secondary))",
          foreground: "hsl(var(--secondary-foreground))",
        },
        muted: {
          DEFAULT: "hsl(var(--muted))",
          foreground: "hsl(var(--muted-foreground))",
        },
        accent: {
          DEFAULT: "hsl(var(--accent))",
          foreground: "hsl(var(--accent-foreground))",
        },
        destructive: {
          DEFAULT: "hsl(var(--destructive))",
          foreground: "hsl(var(--destructive-foreground))",
        },
        border: "hsl(var(--border))",
        input: "hsl(var(--input))",
        ring: "hsl(var(--ring))",
      },
      borderRadius: {
        lg: "var(--radius)",
        md: "calc(var(--radius) - 2px)",
        sm: "calc(var(--radius) - 4px)",
      },
    },
  },
};
```

## Theme Toggle Component

Create a toggle button to switch themes:

```rust
use leptos::prelude::*;
use holt_kit::prelude::*;

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
