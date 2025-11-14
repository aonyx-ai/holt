use leptos::prelude::*;

/// Switch behavior context that manages state and interactions
#[derive(Clone)]
pub struct SwitchContext {
    pub checked: RwSignal<bool>,
    pub disabled: Signal<bool>,
}

impl SwitchContext {
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

/// Root switch primitive that provides context and handles the underlying button
#[component]
pub fn SwitchRoot(
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional)] checked: RwSignal<bool>,
    #[prop(into, default = Signal::stored(false))] disabled: Signal<bool>,
    #[prop(optional_no_strip, into)] id: Option<&'static str>,
    #[prop(optional_no_strip, into)] name: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    let context = SwitchContext::new(checked, disabled);
    let context_on_click = context.clone();
    provide_context(context);

    view! {
        <button
            type="button"
            role="switch"
            aria-checked=move || checked.get()
            class=class
            on:click=move |_| {
                if !context_on_click.is_disabled() {
                    context_on_click.toggle();
                }
            }
            disabled=disabled
            data-state=move || if checked.get() { "checked" } else { "unchecked" }
            id=id
            name=name
        >
            {children()}
        </button>
    }
}

/// Hook to access switch context
pub fn use_switch() -> SwitchContext {
    use_context::<SwitchContext>().expect("use_switch must be called within SwitchRoot")
}

/// Switch thumb component that translates based on checked state
#[component]
pub fn SwitchThumb(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    let context = use_switch();

    view! {
        <span
            class=class
            data-state=move || if context.is_checked() { "checked" } else { "unchecked" }
        />
    }
}
