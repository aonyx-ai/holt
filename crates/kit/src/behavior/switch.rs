use leptos::prelude::*;

/// Switch behavior context that manages state and interactions
#[derive(Clone)]
pub struct SwitchContext {
    pub checked: RwSignal<bool>,
    pub disabled: Signal<bool>,
    pub on_change: Option<Callback<bool>>,
}

impl SwitchContext {
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

/// Root switch primitive that provides context and handles the underlying button
#[component]
pub fn SwitchRoot(
    #[prop(optional, into)] class: String,
    #[prop(optional)] checked: RwSignal<bool>,
    #[prop(into, default = Signal::stored(false))] disabled: Signal<bool>,
    #[prop(optional_no_strip, into)] id: Option<&'static str>,
    #[prop(optional_no_strip, into)] name: Option<&'static str>,
    #[prop(optional_no_strip)] on_change: Option<Callback<bool>>,
    children: Children,
) -> impl IntoView {
    let context = SwitchContext::new(checked, disabled, on_change);
    let context_on_click = context.clone();
    provide_context(context);

    view! {
        <button
            type="button"
            role="switch"
            aria-checked=move || checked.get()
            class=class
            on:click=move |_| context_on_click.toggle()
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
pub fn SwitchThumb(#[prop(optional, into)] class: String) -> impl IntoView {
    let context = use_switch();

    view! {
        <span
            class=class
            data-state=move || if context.is_checked() { "checked" } else { "unchecked" }
        />
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{reactive_scope, track_callback};

    #[test]
    fn class_prop_accepts_str_and_string() {
        assert_class_prop!(SwitchRootProps, SwitchThumbProps);
    }

    #[test]
    fn on_change_fires_on_toggle() {
        reactive_scope(|| {
            let (on_change, last) = track_callback::<bool>();
            let checked = RwSignal::new(false);
            let disabled = Signal::stored(false);

            let context = SwitchContext::new(checked, disabled, Some(on_change));

            context.toggle();
            assert!(checked.get());
            assert_eq!(last.get(), Some(true));

            context.toggle();
            assert!(!checked.get());
            assert_eq!(last.get(), Some(false));
        });
    }

    #[test]
    fn on_change_not_fired_when_disabled() {
        reactive_scope(|| {
            let (on_change, last) = track_callback::<bool>();
            let checked = RwSignal::new(false);
            let disabled = Signal::stored(true);

            let context = SwitchContext::new(checked, disabled, Some(on_change));

            context.toggle();
            assert!(!checked.get());
            assert_eq!(last.get(), None);
        });
    }
}
