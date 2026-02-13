use leptos::prelude::*;

/// Toggle behavior context that manages state and interactions
#[derive(Clone)]
pub struct ToggleContext {
    pub pressed: RwSignal<bool>,
    pub disabled: Signal<bool>,
    pub on_change: Option<Callback<bool>>,
}

impl ToggleContext {
    pub fn new(
        pressed: RwSignal<bool>,
        disabled: Signal<bool>,
        on_change: Option<Callback<bool>>,
    ) -> Self {
        Self {
            pressed,
            disabled,
            on_change,
        }
    }

    pub fn is_pressed(&self) -> bool {
        self.pressed.get()
    }

    pub fn is_disabled(&self) -> bool {
        self.disabled.get()
    }

    pub fn toggle(&self) {
        if !self.is_disabled() {
            self.pressed.update(|p| *p = !*p);
            if let Some(cb) = &self.on_change {
                cb.run(self.pressed.get());
            }
        }
    }
}

/// Root toggle primitive that provides context and handles the underlying button
#[component]
pub fn ToggleRoot(
    #[prop(optional, into)] class: String,
    #[prop(optional)] pressed: RwSignal<bool>,
    #[prop(into, default = Signal::stored(false))] disabled: Signal<bool>,
    #[prop(optional_no_strip, into)] aria_label: Option<&'static str>,
    #[prop(optional_no_strip)] on_change: Option<Callback<bool>>,
    children: Children,
) -> impl IntoView {
    let context = ToggleContext::new(pressed, disabled, on_change);
    let context_on_click = context.clone();
    provide_context(context);

    view! {
        <button
            type="button"
            role="button"
            aria-pressed=move || pressed.get()
            class=class
            on:click=move |_| {
                if !context_on_click.is_disabled() {
                    context_on_click.toggle();
                }
            }
            disabled=disabled
            data-state=move || if pressed.get() { "on" } else { "off" }
            aria-label=aria_label
        >
            {children()}
        </button>
    }
}

/// Hook to access toggle context
pub fn use_toggle() -> ToggleContext {
    use_context::<ToggleContext>().expect("use_toggle must be called within ToggleRoot")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn class_prop_accepts_str_and_string() {
        assert_class_prop!(ToggleRootProps);
    }
}
