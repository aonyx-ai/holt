use leptos::prelude::*;

/// Context for Collapsible components managing the open/closed state
#[derive(Clone)]
pub struct CollapsibleContext {
    pub open: RwSignal<bool>,
}

impl CollapsibleContext {
    pub fn toggle(&self) {
        self.open.update(|o| *o = !*o);
    }
}

/// Root collapsible primitive that provides context
#[component]
pub fn CollapsibleRoot(
    #[prop(optional, into)] class: Option<String>,
    open: RwSignal<bool>,
    children: Children,
) -> impl IntoView {
    let ctx = CollapsibleContext { open };
    provide_context(ctx.clone());

    view! {
        <div class=class data-state=move || if ctx.open.get() { "open" } else { "closed" }>
            {children()}
        </div>
    }
}

/// Access the collapsible context
pub fn use_collapsible() -> CollapsibleContext {
    use_context::<CollapsibleContext>()
        .expect("use_collapsible must be called within CollapsibleRoot")
}

/// A trigger button that toggles the collapsible state
#[component]
pub fn CollapsibleTrigger(
    #[prop(optional, into)] class: Option<String>,
    #[prop(into, default = Signal::stored(false))] disabled: Signal<bool>,
    children: Children,
) -> impl IntoView {
    let ctx = use_collapsible();
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
            aria-expanded=move || ctx.open.get()
            data-state=move || if ctx.open.get() { "open" } else { "closed" }
            disabled=disabled
        >
            {children()}
        </button>
    }
}

/// Content area that is shown/hidden based on state
#[component]
pub fn CollapsibleContent(
    #[prop(optional, into)] class: Option<String>,
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = use_collapsible();
    view! {
        <div
            class=class
            hidden=move || !ctx.open.get()
            data-state=move || if ctx.open.get() { "open" } else { "closed" }
        >
            {children()}
        </div>
    }
    .into_any()
}
