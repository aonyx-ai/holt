use leptos::prelude::*;

/// Context for Tabs components managing the active tab value
#[derive(Clone)]
pub struct TabsContext {
    pub active_value: RwSignal<String>,
}

/// Root tabs primitive that provides context
#[component]
pub fn TabsRoot(
    #[prop(optional, into)] class: String,
    #[prop(into)] default_value: String,
    children: Children,
) -> impl IntoView {
    let active_value = RwSignal::new(default_value);
    let ctx = TabsContext { active_value };
    provide_context(ctx);

    view! { <div class=class>{children()}</div> }
}

/// Access the tabs context
pub fn use_tabs() -> TabsContext {
    use_context::<TabsContext>().expect("use_tabs must be called within TabsRoot")
}

/// Container for tab triggers
#[component]
pub fn TabsList(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! {
        <div role="tablist" class=class>
            {children()}
        </div>
    }
}

/// A trigger button that activates a tab panel
#[component]
pub fn TabsTrigger(
    #[prop(optional, into)] class: String,
    #[prop(into)] value: String,
    #[prop(into, default = Signal::stored(false))] disabled: Signal<bool>,
    children: Children,
) -> impl IntoView {
    let ctx = use_tabs();
    let value_click = value.clone();
    let value_selected = value.clone();

    let on_click = move |_| {
        if !disabled.get() {
            ctx.active_value.set(value_click.clone());
        }
    };

    view! {
        <button
            type="button"
            role="tab"
            on:click=on_click
            class=class
            aria-selected=move || ctx.active_value.get() == value_selected
            data-state=move || {
                if ctx.active_value.get() == value { "active" } else { "inactive" }
            }
            disabled=disabled
        >
            {children()}
        </button>
    }
}

/// Content panel shown when its value matches the active tab
#[component]
pub fn TabsContent(
    #[prop(optional, into)] class: String,
    #[prop(into)] value: String,
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = use_tabs();

    view! {
        <div
            role="tabpanel"
            class=class
            hidden=move || ctx.active_value.get() != value
            data-state=move || {
                if ctx.active_value.get() == value { "active" } else { "inactive" }
            }
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
            TabsRootProps,
            TabsListProps,
            TabsTriggerProps,
            TabsContentProps,
        );
    }
}
