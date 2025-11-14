# Troubleshooting

Common compilation errors and their solutions when generating Leptos components.

## Compilation Errors

**Missing imports**: Ensure all types are imported:

```rust
use leptos::prelude::*;
use leptos::children::Children;
use tailwind_fuse::*;
```

**Module is private** (`E0603`):

```
error[E0603]: module `switch` is private
use crate::behavior::switch::{SwitchRoot}
```

Fix: Import from re-exported module instead:

```rust
use crate::behavior::{SwitchRoot, SwitchThumb};
```

**Use of moved value** (`E0382`):

```
error[E0382]: use of moved value: `context_click`
```

Fix: Clone context separately for each closure:

```rust
let context_keydown = context.clone();
let context_click = context.clone();
```

**Type conversion error** (`E0277`):

```
error[E0277]: `String` does not implement `From<Option<String>>`
class=Some(switch_class)  // ❌
```

Fix: When prop is `#[prop(optional, into)] class: Option<String>`, pass value
directly:

```rust
class=switch_class  // ✅
```

**Method not found** (`E0599`):

```
error[E0599]: no method named `for_` found
<Label for_="input-id">  // ❌
```

Fix: Use raw identifier for Rust keywords:

```rust
<Label r#for="input-id">  // ✅
```

**Trait bounds**: Use `impl IntoView` for return types, not concrete types

**Lifetime issues**: Use `&'static str` for class props

## Styling Not Working

**Classes not applied**: Verify `tw_merge!` or `.with_class()` is used

**Variants not showing**: Check `#[tw(default)]` is set on one variant

**Tailwind not compiling**: Ensure classes are in the configured content paths

## Stories Not Appearing

**Module not exported**: Check story module is listed in `mod.rs`

**Story macro issues**: Verify `#[story]` attribute is present

## Reference Examples

See existing components for patterns:

- Simple: `visual/card.rs`, `visual/label.rs`
- With variants: `visual/button.rs`, `visual/badge.rs`
- Compound: `visual/select.rs`, `visual/collapsible.rs`
- Complex behavior: `behavior/select.rs`, `behavior/dialog.rs`
