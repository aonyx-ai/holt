use leptos::portal::Portal;
use leptos::prelude::*;
use leptos::web_sys::KeyboardEvent;

/// Context for Sheet components managing the open/closed state
#[derive(Clone, Copy)]
pub struct SheetContext {
    pub open: RwSignal<bool>,
}

impl SheetContext {
    pub fn is_open(&self) -> bool {
        self.open.get()
    }

    pub fn toggle(&self) {
        self.open.update(|o| *o = !*o);
    }

    pub fn close(&self) {
        self.open.set(false);
    }
}

/// Root sheet primitive that provides context
#[component]
pub fn SheetRoot(#[prop(optional)] open: RwSignal<bool>, children: Children) -> impl IntoView {
    let ctx = SheetContext { open };
    provide_context(ctx);

    view! { <>{children()}</> }
}

/// Access the sheet context
pub fn use_sheet() -> SheetContext {
    use_context::<SheetContext>().expect("use_sheet must be called within SheetRoot")
}

/// A trigger button that opens the sheet
#[component]
pub fn SheetTrigger(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let ctx = use_sheet();

    view! {
        <button
            type="button"
            on:click=move |_| ctx.open.set(true)
            class=class
            data-state=move || if ctx.is_open() { "open" } else { "closed" }
        >
            {children()}
        </button>
    }
}

/// Content area that shows as an overlay panel
#[component]
pub fn SheetContent(#[prop(optional, into)] class: String, children: ChildrenFn) -> impl IntoView {
    let ctx = use_sheet();
    let children = StoredValue::new(children);

    let on_overlay_click = move |_| {
        ctx.close();
    };

    let on_keydown = move |ev: KeyboardEvent| {
        let action = handle_sheet_keydown(&ev.key());
        if action == SheetKeyAction::Close {
            ev.prevent_default();
            ctx.close();
        }
    };

    view! {
        <Show when=move || ctx.is_open() clone:class>
            <Portal clone:class>
                <div
                    class="sheet-overlay"
                    on:click=on_overlay_click
                    on:keydown=on_keydown
                    data-state="open"
                    role="dialog"
                    aria-modal="true"
                >
                    <div class=class.clone() on:click=move |ev| ev.stop_propagation()>
                        {children.read_value()()}
                    </div>
                </div>
            </Portal>
        </Show>
    }
    .into_any()
}

/// A close button for the sheet
#[component]
pub fn SheetClose(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let ctx = use_sheet();

    view! {
        <button type="button" on:click=move |_| ctx.close() class=class>
            {children()}
        </button>
    }
}

/// Actions from keyboard events within the sheet
#[derive(Clone, Eq, PartialEq, Debug)]
enum SheetKeyAction {
    None,
    Close,
}

fn handle_sheet_keydown(key: &str) -> SheetKeyAction {
    match key {
        "Escape" => SheetKeyAction::Close,
        _ => SheetKeyAction::None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::reactive_scope;

    #[test]
    fn class_prop_accepts_str_and_string() {
        assert_class_prop!(SheetTriggerProps, SheetContentProps, SheetCloseProps);
    }

    #[test]
    fn context_toggle() {
        reactive_scope(|| {
            let open = RwSignal::new(false);
            let ctx = SheetContext { open };

            assert!(!ctx.is_open());
            ctx.toggle();
            assert!(ctx.is_open());
            ctx.toggle();
            assert!(!ctx.is_open());
        });
    }

    #[test]
    fn context_close() {
        reactive_scope(|| {
            let open = RwSignal::new(true);
            let ctx = SheetContext { open };

            assert!(ctx.is_open());
            ctx.close();
            assert!(!ctx.is_open());
        });
    }

    #[test]
    fn escape_closes_sheet() {
        assert_eq!(handle_sheet_keydown("Escape"), SheetKeyAction::Close);
    }

    #[test]
    fn other_keys_are_noop() {
        assert_eq!(handle_sheet_keydown("Enter"), SheetKeyAction::None);
        assert_eq!(handle_sheet_keydown("Tab"), SheetKeyAction::None);
        assert_eq!(handle_sheet_keydown("a"), SheetKeyAction::None);
    }
}
