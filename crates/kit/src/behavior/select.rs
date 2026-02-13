use crate::floating::{Align, FloatingOptions, Side, use_floating};
use leptos::html::Div;
use leptos::portal::Portal;
use leptos::prelude::*;

/// Select behavior context that manages state and interactions
#[derive(Clone)]
pub struct SelectContext {
    pub value: RwSignal<Option<String>>,
    pub open: RwSignal<bool>,
    pub disabled: Signal<bool>,
    pub trigger_ref: NodeRef<leptos::html::Button>,
    pub on_change: Option<Callback<Option<String>>>,
}

impl SelectContext {
    pub fn new(
        value: RwSignal<Option<String>>,
        disabled: Signal<bool>,
        on_change: Option<Callback<Option<String>>>,
    ) -> Self {
        Self {
            value,
            open: RwSignal::new(false),
            disabled,
            trigger_ref: NodeRef::new(),
            on_change,
        }
    }

    pub fn is_open(&self) -> bool {
        self.open.get()
    }

    pub fn is_disabled(&self) -> bool {
        self.disabled.get()
    }

    pub fn toggle(&self) {
        if !self.is_disabled() {
            self.open.update(|open| *open = !*open);
        }
    }

    pub fn close(&self) {
        self.open.set(false);
    }

    pub fn select_value(&self, new_value: String) {
        if !self.is_disabled() {
            self.value.set(Some(new_value));
            if let Some(cb) = &self.on_change {
                cb.run(self.value.get());
            }
            self.close();
        }
    }

    pub fn get_value(&self) -> Option<String> {
        self.value.get()
    }
}

/// Root select primitive that provides context
#[component]
pub fn SelectRoot(
    #[prop(optional)] value: RwSignal<Option<String>>,
    #[prop(into, default = Signal::stored(false))] disabled: Signal<bool>,
    #[prop(optional_no_strip)] on_change: Option<Callback<Option<String>>>,
    children: Children,
) -> impl IntoView {
    let context = SelectContext::new(value, disabled, on_change);
    let context_state = context.clone();

    provide_context(context);

    view! {
        <div
            class="relative"
            data-state=move || if context_state.open.get() { "open" } else { "closed" }
        >
            {children()}
        </div>
    }
}

/// Hook to access select context
pub fn use_select() -> SelectContext {
    use_context::<SelectContext>().expect("use_select must be called within SelectRoot")
}

/// Trigger button that opens/closes the select
#[component]
pub fn SelectTrigger(
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional_no_strip, into)] id: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    let context = use_select();
    let context_disabled = context.clone();
    let context_state = context.clone();

    view! {
        <button
            type="button"
            role="combobox"
            aria-expanded=move || context.open.get()
            aria-haspopup="listbox"
            class=class
            id=id
            node_ref=context.trigger_ref
            on:click=move |_| context.toggle()
            disabled=move || context_disabled.disabled.get()
            data-state=move || if context_state.open.get() { "open" } else { "closed" }
        >
            {children()}
        </button>
    }
}

/// Content area that shows when select is open
#[component]
pub fn SelectContent(
    #[prop(optional, into)] class: Option<String>,
    #[prop(into, default = Side::Bottom)] side: Side,
    #[prop(into, default = Align::Start)] align: Align,
    #[prop(into, default = 4.0)] side_offset: f64,
    children: ChildrenFn,
) -> impl IntoView {
    let context = use_select();
    let children = StoredValue::new(children);
    let content_ref = NodeRef::<Div>::new();

    // Set up floating positioning
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
                    role="listbox"
                    class=class.clone()
                    data-state="open"
                    node_ref=content_ref
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

/// Individual select item
#[component]
pub fn SelectItem(
    #[prop(into)] value: String,
    #[prop(optional, into)] class: Option<String>,
    #[prop(into, default = Signal::stored(false))] disabled: Signal<bool>,
    children: Children,
) -> impl IntoView {
    let context = use_select();
    let item_value = value.clone();
    let select_value = value.clone();

    let context_selected = context.clone();
    let is_selected = Signal::derive(move || {
        context_selected
            .get_value()
            .is_some_and(|v| v == item_value)
    });

    view! {
        <div
            role="option"
            aria-selected=move || is_selected.get()
            class=class
            on:click=move |_| {
                if !disabled.get() {
                    context.select_value(select_value.clone());
                }
            }
            data-disabled=move || disabled.get()
            data-state=move || if is_selected.get() { "checked" } else { "unchecked" }
        >
            {children()}
        </div>
    }
}

/// Component to display current value or placeholder
#[component]
pub fn SelectValue(
    #[prop(optional_no_strip, into)] placeholder: Option<String>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let context = use_select();
    let placeholder_text = placeholder.unwrap_or_default();

    view! {
        <span class=class>
            {move || context.get_value().unwrap_or_else(|| placeholder_text.clone())}
        </span>
    }
}
