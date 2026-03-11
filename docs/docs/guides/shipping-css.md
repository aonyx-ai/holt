---
sidebar_position: 4
---

# Ship Pre-compiled CSS from Component Libraries

Tailwind CSS works by scanning your source files for class names. When your
components live in a separate crate, Tailwind doesn't know where to find that
crate's source — so the classes go missing and components render unstyled.

The workaround: **ship a pre-compiled CSS file** with your library. A `build.rs`
copies it to `target/css/<crate>/` at build time, and consumers `@import` it.

## For library authors

### 1. Create a Tailwind input file

Import only `tailwindcss/utilities` (not all of `tailwindcss`) so the output
contains just the utility classes your library uses — no resets, no base styles,
no `@property` declarations. Consumers already have all of that.

Add a `@source` directive pointing at your Rust source, the theme variables your
components depend on, and an `@theme inline` block mapping them to Tailwind:

```css
@import "tailwindcss/utilities";
@source "src/**/*.rs";

@custom-variant dark (&:is(.dark *));

:root {
  --background: oklch(1 0 0);
  --foreground: oklch(0.145 0 0);
  /* ... */
}

@theme inline {
  --color-background: var(--background);
  --color-foreground: var(--foreground);
  /* ... */
}
```

The theme variables are needed at generation time so Tailwind can resolve
classes like `bg-sidebar-accent`. They're stripped from the output (see below)
and don't end up in the shipped CSS.

### 2. Generate the CSS

Run the Tailwind CLI, then strip the theme/property boilerplate that Tailwind
adds to every output — consumers already get this from their own
`@import "tailwindcss"`:

```bash
tailwindcss -i tailwind-input.css -o assets/my-lib.css

# Remove the @layer properties declaration and everything from :root onward
sed -i '' '/^@layer properties;$/d; /^:root {$/,$d' assets/my-lib.css
```

The result is pure utility CSS. Regenerate whenever you change styles — ideally
as part of your publish workflow.

### 3. Add a build.rs

The `build.rs` copies the generated CSS to `target/css/<crate>/` so consumers
have a stable import path:

```rust
fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let css_path = std::path::Path::new(&manifest_dir)
        .join("assets/my-lib.css");

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let target_dir = std::path::Path::new(&out_dir)
        .ancestors()
        .find(|p| p.file_name().is_some_and(|n| n == "target"))
        .unwrap()
        .to_path_buf();

    let dest = target_dir.join("css/my-lib");
    std::fs::create_dir_all(&dest).ok();

    if css_path.exists() {
        std::fs::copy(&css_path, dest.join("my-lib.css")).ok();
    }

    println!("cargo:rerun-if-changed=assets/my-lib.css");
}
```

### 4. Document it for your consumers

Tell consumers to add an `@import` in their stylesheet pointing at the
`target/css/` path. For example, your README might say:

> Add this import to your CSS file, after `@import "tailwindcss"`:
>
> ```css
> @import "../../target/css/my-lib/my-lib.css";
> ```
>
> The path is relative from your CSS file to your workspace's `target/`
> directory. The file is created automatically when `cargo build` runs.

Also document which CSS custom properties your components expect (e.g.
`--background`, `--sidebar-accent`). The shipped CSS contains only utility
classes that reference these variables — as long as the consumer defines them,
everything resolves correctly at runtime with no hard-coded color coupling.

## Tradeoffs

This is a pragmatic v1 approach with known limitations:

- **One `@import` per dependency.** No auto-discovery from the dependency graph.
- **No deduplication.** If two libraries both use `flex`, both ship that rule.
- **No tree-shaking.** Consumers get all of a library's CSS even if they only
  use some of its components.
- **Manual regeneration.** Re-run CSS generation when styles change.

See [#482](https://github.com/aonyx-ai/holt/issues/482) for the tracking issue
on automatic CSS discovery, deduplication, and tree-shaking.
