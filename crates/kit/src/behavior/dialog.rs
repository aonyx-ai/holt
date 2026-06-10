use leptos::portal::Portal;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use leptos::web_sys::KeyboardEvent;

/// Context for Dialog components managing the open/closed state
#[derive(Copy, Clone)]
pub struct DialogContext {
    pub open: RwSignal<bool>,
}

impl DialogContext {
    pub fn close(&self) {
        self.open.set(false);
    }

    pub fn show(&self) {
        self.open.set(true);
    }
}

/// Root dialog primitive that provides context
#[component]
pub fn DialogRoot(#[prop(optional)] open: RwSignal<bool>, children: Children) -> impl IntoView {
    let ctx = DialogContext { open };
    provide_context(ctx);

    view! { <>{children()}</> }
}

/// Access the dialog context
pub fn use_dialog() -> DialogContext {
    use_context::<DialogContext>().expect("use_dialog must be called within DialogRoot")
}

/// A trigger button that opens the dialog
#[component]
pub fn DialogTrigger(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let ctx = use_dialog();

    view! {
        <button
            type="button"
            class=class
            on:click=move |_| ctx.show()
            aria-haspopup="dialog"
            data-state=move || if ctx.open.get() { "open" } else { "closed" }
        >
            {children()}
        </button>
    }
}

/// Content area displayed as a modal overlay via Portal
#[component]
pub fn DialogContent(#[prop(optional, into)] class: String, children: ChildrenFn) -> impl IntoView {
    let ctx = use_dialog();
    let children = StoredValue::new(children);
    let content_ref = NodeRef::<leptos::html::Div>::new();

    view! {
        <Show when=move || ctx.open.get() clone:class>
            <Portal clone:class>
                // Backdrop overlay — clicking it closes the dialog
                <div
                    class="dialog-backdrop"
                    on:click=move |_| ctx.close()
                    aria-hidden="true"
                ></div>
                <div
                    role="dialog"
                    aria-modal="true"
                    class=class.clone()
                    node_ref=content_ref
                    tabindex="-1"
                    on:keydown=move |ev: KeyboardEvent| {
                        if ev.key() == "Escape" {
                            ev.prevent_default();
                            ctx.close();
                        }
                    }
                    data-state="open"
                >
                    {children.read_value()()}
                </div>
            </Portal>
        </Show>
    }
    .into_any()
}

/// A button that closes the dialog
#[component]
pub fn DialogClose(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let ctx = use_dialog();

    view! {
        <button type="button" class=class on:click=move |_| ctx.close()>
            {children()}
        </button>
    }
}

/// Actions that can result from a keydown within the dialog
#[derive(Clone, Eq, PartialEq, Debug)]
enum DialogKeyAction {
    None,
    Close,
}

fn handle_dialog_keydown(key: &str) -> DialogKeyAction {
    match key {
        "Escape" => DialogKeyAction::Close,
        _ => DialogKeyAction::None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn class_prop_accepts_str_and_string() {
        assert_class_prop!(DialogTriggerProps, DialogContentProps, DialogCloseProps,);
    }

    #[test]
    fn escape_closes_dialog() {
        assert_eq!(handle_dialog_keydown("Escape"), DialogKeyAction::Close);
    }

    #[test]
    fn unhandled_keys_are_noop() {
        assert_eq!(handle_dialog_keydown("Enter"), DialogKeyAction::None);
        assert_eq!(handle_dialog_keydown("Tab"), DialogKeyAction::None);
        assert_eq!(handle_dialog_keydown("a"), DialogKeyAction::None);
    }
}
