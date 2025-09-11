use leptos::prelude::*;

/// Checkbox behavior context that manages state and interactions
#[derive(Clone)]
pub struct CheckboxContext {
    pub checked: RwSignal<bool>,
    pub disabled: Signal<bool>,
}

impl CheckboxContext {
    pub fn new(checked: RwSignal<bool>, disabled: Signal<bool>) -> Self {
        Self { checked, disabled }
    }

    pub fn is_checked(&self) -> bool {
        self.checked.get()
    }

    pub fn is_disabled(&self) -> bool {
        self.disabled.get()
    }

    pub fn toggle(&self) {
        if !self.is_disabled() {
            self.checked.update(|c| *c = !*c);
        }
    }
}

/// Root checkbox primitive that provides context and handles the underlying input
#[component]
pub fn CheckboxRoot(
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional)] checked: RwSignal<bool>,
    #[prop(into, default = Signal::stored(false))] disabled: Signal<bool>,
    #[prop(optional_no_strip, into)] id: Option<&'static str>,
    #[prop(optional_no_strip, into)] name: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    let context = CheckboxContext::new(checked, disabled);
    provide_context(context);

    view! {
        <input
            type="checkbox"
            class=class
            bind:checked=checked
            disabled=disabled
            data-state=if checked.get() { "checked" } else { "unchecked" }
            id=id
            name=name
        />
        {children()}
    }
}

/// Hook to access checkbox context
pub fn use_checkbox() -> CheckboxContext {
    use_context::<CheckboxContext>().expect("use_checkbox must be called within CheckboxRoot")
}

#[component]
pub fn CheckboxIndicator(
    #[prop(optional, into)] class: Signal<String>,
    children: ChildrenFn,
) -> impl IntoView {
    let context = use_checkbox();

    view! {
        <Show when=move || context.checked.get()>
            <div class=class>{children()}</div>
        </Show>
    }
}
