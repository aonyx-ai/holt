# UI Component Architecture

Components use behavior/visual separation: behavior components handle logic,
visual components add styling.

## Structure

- **`behavior/`** - State management and interactions (Leptos primitives)
- **`visual/`** - Tailwind styling wrappers
- **`leptos-floating`** (external crate) - Positioning for dropdowns, tooltips,
  popovers

## Development Pattern

1. Create behavior component in `behavior/`
2. Create visual wrapper in `visual/` with Tailwind classes
3. Export both from respective `mod.rs`
4. Add story in `crates/ui-book/src/stories/`

## Testing

- Unit tests for logic
- `just kit test-e2e` to run browser tests
