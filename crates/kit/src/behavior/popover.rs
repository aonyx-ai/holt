use leptos::html::Div;
use leptos::portal::Portal;
use leptos::prelude::*;
use leptos::web_sys::KeyboardEvent;
use leptos_floating::{Align, FloatingOptions, Side, use_floating};

/// Popover behavior context that manages open state and trigger reference
#[derive(Copy, Clone)]
pub struct PopoverContext {
    pub open: RwSignal<bool>,
    pub trigger_ref: NodeRef<leptos::html::Button>,
}

impl PopoverContext {
    pub fn new(open: RwSignal<bool>) -> Self {
        Self {
            open,
            trigger_ref: NodeRef::new(),
        }
    }

    pub fn is_open(&self) -> bool {
        self.open.get()
    }

    pub fn toggle(&self) {
        self.open.update(|open| *open = !*open);
    }

    pub fn close(&self) {
        self.open.set(false);
    }

    pub fn open(&self) {
        self.open.set(true);
    }

    pub fn focus_trigger(&self) {
        if let Some(el) = self.trigger_ref.get() {
            let _ = el.focus();
        }
    }
}

/// Root popover primitive that provides context
#[component]
pub fn PopoverRoot(#[prop(optional)] open: RwSignal<bool>, children: Children) -> impl IntoView {
    let context = PopoverContext::new(open);

    provide_context(context);

    view! {
        <div class="relative" data-state=move || if context.open.get() { "open" } else { "closed" }>
            {children()}
        </div>
    }
}

/// Hook to access popover context
pub fn use_popover() -> PopoverContext {
    use_context::<PopoverContext>().expect("use_popover must be called within PopoverRoot")
}

/// Trigger button that toggles the popover
#[component]
pub fn PopoverTrigger(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let context = use_popover();

    view! {
        <button
            type="button"
            aria-expanded=move || context.open.get()
            aria-haspopup="dialog"
            class=class
            node_ref=context.trigger_ref
            on:click=move |_| context.toggle()
            data-state=move || if context.open.get() { "open" } else { "closed" }
        >
            {children()}
        </button>
    }
}

/// Content area that shows when popover is open
#[component]
pub fn PopoverContent(
    #[prop(optional, into)] class: String,
    #[prop(into, default = Side::Bottom)] side: Side,
    #[prop(into, default = Align::Start)] align: Align,
    #[prop(into, default = 4.0)] side_offset: f64,
    children: ChildrenFn,
) -> impl IntoView {
    let context = use_popover();
    let children = StoredValue::new(children);
    let content_ref = NodeRef::<Div>::new();

    let floating_options = FloatingOptions {
        side,
        align,
        side_offset,
        align_offset: 0.0,
    };

    let floating = use_floating(context.trigger_ref, content_ref, floating_options);

    view! {
        <Show when=move || context.open.get() clone:class>
            <Portal clone:class>
                <div
                    role="dialog"
                    class=class.clone()
                    data-state="open"
                    node_ref=content_ref
                    on:keydown=move |ev: KeyboardEvent| {
                        let action = handle_content_keydown(&ev.key());
                        if action == PopoverKeyAction::Close {
                            ev.prevent_default();
                            context.close();
                            context.focus_trigger();
                        }
                    }
                    style:position="fixed"
                    style:left=move || format!("{}px", floating.x.get())
                    style:top=move || format!("{}px", floating.y.get())
                    style:z-index="50"
                    data-side=move || format!("{:?}", floating.side.get()).to_lowercase()
                >
                    {children.read_value()()}
                </div>
            </Portal>
        </Show>
    }
}

/// Actions that can result from a keydown within popover content
#[derive(Clone, Eq, PartialEq, Debug)]
enum PopoverKeyAction {
    None,
    Close,
}

fn handle_content_keydown(key: &str) -> PopoverKeyAction {
    match key {
        "Escape" => PopoverKeyAction::Close,
        _ => PopoverKeyAction::None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::reactive_scope;

    #[test]
    fn class_prop_accepts_str_and_string() {
        assert_class_prop!(PopoverTriggerProps, PopoverContentProps,);
    }

    #[test]
    fn toggle_flips_open_state() {
        reactive_scope(|| {
            let open = RwSignal::new(false);
            let context = PopoverContext::new(open);

            assert!(!context.is_open());
            context.toggle();
            assert!(context.is_open());
            context.toggle();
            assert!(!context.is_open());
        });
    }

    #[test]
    fn close_sets_open_false() {
        reactive_scope(|| {
            let open = RwSignal::new(true);
            let context = PopoverContext::new(open);

            assert!(context.is_open());
            context.close();
            assert!(!context.is_open());
        });
    }

    #[test]
    fn open_sets_open_true() {
        reactive_scope(|| {
            let open = RwSignal::new(false);
            let context = PopoverContext::new(open);

            assert!(!context.is_open());
            context.open();
            assert!(context.is_open());
        });
    }

    #[test]
    fn escape_closes_popover() {
        assert_eq!(handle_content_keydown("Escape"), PopoverKeyAction::Close);
    }

    #[test]
    fn unhandled_keys_are_noop() {
        assert_eq!(handle_content_keydown("Tab"), PopoverKeyAction::None);
        assert_eq!(handle_content_keydown("Enter"), PopoverKeyAction::None);
        assert_eq!(handle_content_keydown("a"), PopoverKeyAction::None);
    }
}
