use leptos::prelude::*;

/// Context for Collapsible components managing the open/closed state
///
/// # Examples
///
/// ```
/// use holt_kit::behavior::CollapsibleContext;
/// use holt_kit::testing::reactive_scope;
/// use leptos::prelude::*;
///
/// reactive_scope(|| {
///     let open = RwSignal::new(false);
///     let context = CollapsibleContext::builder().open(open).build();
///
///     context.toggle();
///
///     assert!(open.get());
/// });
/// ```
#[derive(Clone, bon::Builder, getset::CopyGetters)]
pub struct CollapsibleContext {
    #[getset(get_copy = "pub")]
    open: RwSignal<bool>,
}

impl CollapsibleContext {
    pub fn toggle(&self) {
        self.open.update(|o| *o = !*o);
    }
}

/// Renders the root collapsible primitive that provides context
///
/// # Examples
///
/// ```
/// use holt_kit::behavior::CollapsibleRoot;
/// use holt_kit::testing::reactive_scope;
/// use leptos::prelude::*;
///
/// reactive_scope(|| {
///     let open = RwSignal::new(false);
///     let _view = view! { <CollapsibleRoot open=open>"Content"</CollapsibleRoot> };
/// });
/// ```
#[component]
pub fn CollapsibleRoot(
    #[prop(optional, into)] class: String,
    open: RwSignal<bool>,
    children: Children,
) -> impl IntoView {
    let ctx = CollapsibleContext::builder().open(open).build();
    provide_context(ctx.clone());

    view! {
        <div class=class data-state=move || if ctx.open().get() { "open" } else { "closed" }>
            {children()}
        </div>
    }
}

/// Access the collapsible context
///
/// # Examples
///
/// ```
/// use holt_kit::behavior::{CollapsibleContext, use_collapsible};
/// use holt_kit::testing::reactive_scope;
/// use leptos::prelude::*;
///
/// reactive_scope(|| {
///     let open = RwSignal::new(true);
///     provide_context(CollapsibleContext::builder().open(open).build());
///
///     assert!(use_collapsible().open().get());
/// });
/// ```
pub fn use_collapsible() -> CollapsibleContext {
    use_context::<CollapsibleContext>()
        .expect("use_collapsible must be called within CollapsibleRoot")
}

/// Renders a trigger button that toggles the collapsible state
///
/// # Examples
///
/// ```
/// use holt_kit::behavior::{CollapsibleRoot, CollapsibleTrigger};
/// use holt_kit::testing::reactive_scope;
/// use leptos::prelude::*;
///
/// reactive_scope(|| {
///     let open = RwSignal::new(false);
///     let _view = view! {
///         <CollapsibleRoot open=open>
///             <CollapsibleTrigger>"Toggle"</CollapsibleTrigger>
///         </CollapsibleRoot>
///     };
/// });
/// ```
#[component]
pub fn CollapsibleTrigger(
    #[prop(optional, into)] class: String,
    #[prop(into, default = Signal::stored(false))] disabled: Signal<bool>,
    children: Children,
) -> impl IntoView {
    let ctx = use_collapsible();
    let open = ctx.open();
    let ctx_click = ctx.clone();

    let on_click = move |_| {
        if !disabled.get() {
            ctx_click.toggle();
        }
    };

    view! {
        <button
            type="button"
            on:click=on_click
            class=class
            aria-expanded=move || open.get()
            data-state=move || if open.get() { "open" } else { "closed" }
            disabled=disabled
        >
            {children()}
        </button>
    }
}

/// Renders the content area that is shown or hidden based on state
///
/// # Examples
///
/// ```
/// use holt_kit::behavior::{CollapsibleContent, CollapsibleRoot};
/// use holt_kit::testing::reactive_scope;
/// use leptos::prelude::*;
///
/// reactive_scope(|| {
///     let open = RwSignal::new(false);
///     let _view = view! {
///         <CollapsibleRoot open=open>
///             <CollapsibleContent>"Content"</CollapsibleContent>
///         </CollapsibleRoot>
///     };
/// });
/// ```
#[component]
pub fn CollapsibleContent(
    #[prop(optional, into)] class: String,
    children: ChildrenFn,
) -> impl IntoView {
    let open = use_collapsible().open();
    view! {
        <div
            class=class
            hidden=move || !open.get()
            data-state=move || if open.get() { "open" } else { "closed" }
        >
            {children()}
        </div>
    }
    .into_any()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn class_prop_accepts_str_and_string() {
        assert_class_prop!(
            CollapsibleRootProps,
            CollapsibleTriggerProps,
            CollapsibleContentProps,
        );
    }

    #[test]
    fn collapsible_context_is_send_sync_and_unpin() {
        fn assert_send_sync_and_unpin<T: Send + Sync + Unpin>() {}

        assert_send_sync_and_unpin::<CollapsibleContext>();
    }
}
