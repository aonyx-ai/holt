---
name: generating-leptos-components
description:
  Generates Leptos UI components by translating Shadcn or Radix components into
  idiomatic Rust/Leptos code with behavior/UI separation, Tailwind styling, and
  Storybook stories. Use when creating buttons, inputs, forms, cards, dialogs,
  selects, switches, checkboxes, or any UI component from Shadcn/Radix designs.
---

# Generating Leptos Components

This skill translates React-based UI components (Shadcn and Radix) into
production-ready Leptos/Rust code following Holt's architecture patterns.

## Requirements

- Rust 1.88+
- Leptos 0.8+
- `tailwind_fuse` for Tailwind CSS integration
- `just` task runner
- `wasm-pack` for testing
- Access to Shadcn UI (https://ui.shadcn.com/) and Radix UI
  (https://www.radix-ui.com/) documentation

## When to Use

Use this skill when creating new UI components based on Shadcn or Radix designs,
such as buttons, inputs, selects, dialogs, cards, or other UI primitives.

## Component Generation Workflow

### 1. Research and Analysis

Examine the Shadcn or Radix component and understand its implementation:

- [ ] Access official documentation and source code
- [ ] Understand component API, props, and behavior patterns
- [ ] Identify styling patterns, variants, and size options
- [ ] Note accessibility features (ARIA attributes, keyboard navigation, focus
      management)

#### Fetching Component Source Code

**For shadcn/ui Components:**

Use the Registry API to fetch component source:

```
https://ui.shadcn.com/r/styles/default/{component-name}.json
```

- Returns JSON with component source in `files[].content` field
- Component names are kebab-case (e.g., `switch`, `button`, `data-table`)
- This is the official distribution method

Example: `https://ui.shadcn.com/r/styles/default/button.json`

**Backup - Component Registry:**

If you can't find a specific component, check the full registry for all
available components:

```
https://ui.shadcn.com/r/styles/default/registry.json
```

This lists all available components and their metadata.

**For Radix UI Primitives:**

Use GitHub Raw Files to access TypeScript source:

```
https://raw.githubusercontent.com/radix-ui/primitives/main/packages/react/{component-name}/src/{ComponentName}.tsx
```

- Direct access to TypeScript source files
- Directory names are kebab-case (e.g., `switch`, `dropdown-menu`)
- File names are PascalCase (e.g., `Switch.tsx`, `DropdownMenu.tsx`)

Example:
`https://raw.githubusercontent.com/radix-ui/primitives/main/packages/react/switch/src/Switch.tsx`

### 2. Architecture Planning

**CRITICAL**: Before writing any code, read similar existing components to
understand established patterns.

For interactive components (switch, checkbox, toggle):

- [ ] Read `crates/kit/src/behavior/checkbox.rs` for behavior patterns
- [ ] Read `crates/kit/src/visual/checkbox.rs` for styling patterns
- [ ] Identify which existing component is most similar to your target

Design following Holt's separation of concerns:

**Behavior Module** (`crates/kit/src/behavior/component_name.rs`):

- State management and event handling
- Business logic and side effects
- Context providers if needed
- No styling or rendering

**UI Module** (`crates/kit/src/visual/component_name.rs`):

- Presentation and rendering
- Tailwind CSS styling with variants
- User-facing component API
- Wraps behavior primitives

### 3. Styling Implementation

**For simple components** (no variants): Use `tw_merge!` macro.

**For components with variants**: Use `TwClass` + `TwVariant` derives.

See [styling-guide.md](./styling-guide.md) for detailed patterns and examples.

### 4. Code Translation

Translate React patterns to Leptos:

- Props using `#[prop(optional)]` and `#[component]`
- State using Leptos signals
- Events using `on:click`, `on:keydown`, etc.
- Context using `Provider` and `expect_context`

See [patterns.md](./patterns.md) for code examples.

### 5. File Organization

**Create component files**:

```
crates/kit/src/
├── behavior/component_name.rs
└── visual/component_name.rs
```

**Update module exports**:

- [ ] Add `mod component_name;` and `pub use component_name::*;` to
      `behavior/mod.rs`
- [ ] Add `mod component_name;` and `pub use component_name::*;` to
      `visual/mod.rs`
- [ ] Run `just lint-rust` to verify module structure compiles

### 6. Story Development

Create stories in `crates/kit-docs/src/stories/component_name.rs` to showcase
the component:

```rust
use holt_book::{story, variant};
use holt_kit::visual::*;
use leptos::prelude::*;

#[variant]
fn default() -> AnyView {
    view! {
        <ComponentName>"Default variant"</ComponentName>
    }
    .into_any()
}

#[variant]
fn variant_example() -> AnyView {
    view! {
        <ComponentName variant=ComponentVariant::Example>"Example"</ComponentName>
    }
    .into_any()
}

include!(concat!(env!("OUT_DIR"), "/stories/component_name_source.rs"));

#[story(id = "component_name", name = "Component Name", extra_docs = COMPONENT_NAME_SOURCE)]
const COMPONENT_NAME_STORY: () = &[default, variant_example];
```

- [ ] Update `crates/kit-docs/src/stories/mod.rs` to add
      `pub mod component_name;`

### 7. Visual Verification - REQUIRED

**CRITICAL: This step is MANDATORY and must be completed before finishing.**

You MUST use `render-variant` to verify styling matches the original design. Do
NOT skip this step or suggest it as a "next step" for the user.

```bash
# Render at least the default and one other key variant
just kit-docs render-variant component_name default ./component-default.png
just kit-docs render-variant component_name checked ./component-checked.png
```

After rendering:

- [ ] Verify the output images look correct
- [ ] Check that styling matches the Shadcn/Radix reference
- [ ] Confirm all variants render properly
- [ ] Verify responsive behavior if applicable
- [ ] If issues found, iterate on styling and re-render
- [ ] Only proceed to Quality Assurance after confirming visuals are correct

### 8. Quality Assurance

Before completion, verify:

- [ ] Code compiles without errors or warnings
- [ ] Follows Rust naming conventions
- [ ] Type-safe (no unwrap() in production)
- [ ] Includes doc comments for public APIs
- [ ] Accessibility features preserved
- [ ] **Visual verification completed with `render-variant` (REQUIRED)**
- [ ] All rendered variants look correct and match reference design
- [ ] Responsive design tested
- [ ] Focus and disabled states styled
- [ ] Edge cases handled
- [ ] Stories demonstrate all features
- [ ] Exported from appropriate modules

## Additional Resources

**Component-specific guides** (read as needed for your specific implementation):

- [styling-guide.md](./styling-guide.md) - Tailwind Fuse styling patterns and
  variant configuration
- [patterns.md](./patterns.md) - Code translation patterns from React to Leptos
- [troubleshooting.md](./troubleshooting.md) - Common compilation errors and
  solutions

**External documentation**:

- Shadcn UI: https://ui.shadcn.com/
- Radix UI: https://www.radix-ui.com/
- Leptos Book: https://leptos.dev/
- Tailwind CSS: https://tailwindcss.com/
