use leptos::portal::Portal;
use leptos::prelude::*;

/// Alert dialog context that manages the open/closed state
#[derive(Copy, Clone)]
pub struct AlertDialogContext {
    pub open: RwSignal<bool>,
}

impl AlertDialogContext {
    pub fn new(open: RwSignal<bool>) -> Self {
        Self { open }
    }

    pub fn is_open(&self) -> bool {
        self.open.get()
    }

    pub fn open(&self) {
        self.open.set(true);
    }

    pub fn close(&self) {
        self.open.set(false);
    }
}

/// Root alert dialog primitive that provides context
#[component]
pub fn AlertDialogRoot(
    #[prop(optional)] open: RwSignal<bool>,
    children: Children,
) -> impl IntoView {
    let context = AlertDialogContext::new(open);
    provide_context(context);

    view! { <>{children()}</> }
}

/// Hook to access alert dialog context
pub fn use_alert_dialog() -> AlertDialogContext {
    use_context::<AlertDialogContext>()
        .expect("use_alert_dialog must be called within AlertDialogRoot")
}

/// Trigger button that opens the alert dialog
#[component]
pub fn AlertDialogTrigger(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let context = use_alert_dialog();

    view! {
        <button type="button" class=class on:click=move |_| context.open()>
            {children()}
        </button>
    }
}

/// Overlay backdrop and centered content container rendered in a Portal
#[component]
pub fn AlertDialogContent(
    #[prop(optional, into)] class: String,
    children: ChildrenFn,
) -> impl IntoView {
    let context = use_alert_dialog();
    let children = StoredValue::new(children);

    view! {
        <Show when=move || context.is_open() clone:class>
            <Portal clone:class>
                <div
                    class="fixed inset-0 z-50 bg-black/80"
                    on:click=move |_| context.close()
                    role="presentation"
                    data-state="open"
                />
                <div
                    class=format!(
                        "fixed left-1/2 top-1/2 z-50 -translate-x-1/2 -translate-y-1/2 {}",
                        class,
                    )
                    role="alertdialog"
                    aria-modal="true"
                >
                    {children.read_value()()}
                </div>
            </Portal>
        </Show>
    }
}

/// Action button that confirms and closes the alert dialog
#[component]
pub fn AlertDialogAction(
    #[prop(optional, into)] class: String,
    #[prop(optional_no_strip)] on_click: Option<Callback<()>>,
    children: Children,
) -> impl IntoView {
    let context = use_alert_dialog();

    view! {
        <button
            type="button"
            class=class
            on:click=move |_| {
                if let Some(cb) = &on_click {
                    cb.run(());
                }
                context.close();
            }
        >
            {children()}
        </button>
    }
}

/// Cancel button that closes the alert dialog without action
#[component]
pub fn AlertDialogCancel(
    #[prop(optional, into)] class: String,
    #[prop(optional_no_strip)] on_click: Option<Callback<()>>,
    children: Children,
) -> impl IntoView {
    let context = use_alert_dialog();

    view! {
        <button
            type="button"
            class=class
            on:click=move |_| {
                if let Some(cb) = &on_click {
                    cb.run(());
                }
                context.close();
            }
        >
            {children()}
        </button>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::reactive_scope;

    #[test]
    fn class_prop_accepts_str_and_string() {
        assert_class_prop!(
            AlertDialogTriggerProps,
            AlertDialogContentProps,
            AlertDialogActionProps,
            AlertDialogCancelProps,
        );
    }

    #[test]
    fn context_open_close() {
        reactive_scope(|| {
            let open = RwSignal::new(false);
            let context = AlertDialogContext::new(open);

            assert!(!context.is_open());
            context.open();
            assert!(context.is_open());
            context.close();
            assert!(!context.is_open());
        });
    }

    #[test]
    fn context_starts_open() {
        reactive_scope(|| {
            let open = RwSignal::new(true);
            let context = AlertDialogContext::new(open);

            assert!(context.is_open());
            context.close();
            assert!(!context.is_open());
        });
    }
}
