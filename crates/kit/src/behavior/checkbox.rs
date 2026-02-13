use leptos::prelude::*;

/// Checkbox behavior context that manages state and interactions
#[derive(Clone)]
pub struct CheckboxContext {
    pub checked: RwSignal<bool>,
    pub disabled: Signal<bool>,
    pub on_change: Option<Callback<bool>>,
}

impl CheckboxContext {
    pub fn new(
        checked: RwSignal<bool>,
        disabled: Signal<bool>,
        on_change: Option<Callback<bool>>,
    ) -> Self {
        Self {
            checked,
            disabled,
            on_change,
        }
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
            if let Some(cb) = &self.on_change {
                cb.run(self.checked.get());
            }
        }
    }
}

/// Root checkbox primitive that provides context and handles the underlying button
#[component]
pub fn CheckboxRoot(
    #[prop(optional, into)] class: String,
    #[prop(optional)] checked: RwSignal<bool>,
    #[prop(into, default = Signal::stored(false))] disabled: Signal<bool>,
    #[prop(optional_no_strip, into)] id: Option<&'static str>,
    #[prop(optional_no_strip, into)] name: Option<&'static str>,
    #[prop(optional_no_strip)] on_change: Option<Callback<bool>>,
    children: Children,
) -> impl IntoView {
    let context = CheckboxContext::new(checked, disabled, on_change);
    let context_on_click = context.clone();
    provide_context(context);

    view! {
        <button
            type="button"
            role="checkbox"
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

/// Hook to access checkbox context
pub fn use_checkbox() -> CheckboxContext {
    use_context::<CheckboxContext>().expect("use_checkbox must be called within CheckboxRoot")
}

#[component]
pub fn CheckboxIndicator(
    #[prop(optional, into)] class: String,
    children: ChildrenFn,
) -> impl IntoView {
    let context = use_checkbox();

    view! {
        <Show when=move || context.checked.get()>
            <div class=class.clone()>{children()}</div>
        </Show>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn class_prop_accepts_str_and_string() {
        assert_class_prop!(CheckboxRootProps, CheckboxIndicatorProps);
    }
}
