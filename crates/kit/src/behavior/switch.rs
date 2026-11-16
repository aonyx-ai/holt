use leptos::prelude::*;

/// Switch behavior context that manages state and interactions
///
/// # Examples
///
/// ```
/// use holt_kit::behavior::SwitchContext;
/// use holt_kit::testing::reactive_scope;
/// use leptos::prelude::*;
///
/// reactive_scope(|| {
///     let context = SwitchContext::builder()
///         .checked(RwSignal::new(false))
///         .disabled(Signal::stored(false))
///         .build();
///
///     context.toggle();
///
///     assert!(context.is_checked());
/// });
/// ```
#[derive(Clone, bon::Builder, getset::CopyGetters)]
pub struct SwitchContext {
    #[getset(get_copy = "pub")]
    checked: RwSignal<bool>,
    #[getset(get_copy = "pub")]
    disabled: Signal<bool>,
    #[getset(get_copy = "pub")]
    on_change: Option<Callback<bool>>,
}

impl SwitchContext {
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

/// Renders the root switch primitive that provides context and handles the underlying button
///
/// # Examples
///
/// ```
/// use holt_kit::behavior::SwitchRoot;
/// use holt_kit::testing::reactive_scope;
/// use leptos::prelude::*;
///
/// reactive_scope(|| {
///     let _view = view! { <SwitchRoot>"Label"</SwitchRoot> };
/// });
/// ```
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
    let context = SwitchContext::builder()
        .checked(checked)
        .disabled(disabled)
        .maybe_on_change(on_change)
        .build();
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

/// Returns the switch context
///
/// # Examples
///
/// ```
/// use holt_kit::behavior::{SwitchContext, use_switch};
/// use holt_kit::testing::reactive_scope;
/// use leptos::prelude::*;
///
/// reactive_scope(|| {
///     let context = SwitchContext::builder()
///         .checked(RwSignal::new(true))
///         .disabled(Signal::stored(false))
///         .build();
///     provide_context(context);
///
///     assert!(use_switch().is_checked());
/// });
/// ```
pub fn use_switch() -> SwitchContext {
    use_context::<SwitchContext>().expect("use_switch must be called within SwitchRoot")
}

/// Renders the switch thumb that translates based on the checked state
///
/// # Examples
///
/// ```
/// use holt_kit::behavior::{SwitchRoot, SwitchThumb};
/// use holt_kit::testing::reactive_scope;
/// use leptos::prelude::*;
///
/// reactive_scope(|| {
///     let _view = view! { <SwitchRoot><SwitchThumb /></SwitchRoot> };
/// });
/// ```
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
    fn on_change_fires_on_toggle_first_toggle_reports_true() {
        reactive_scope(|| {
            let (on_change, last) = track_callback::<bool>();
            let context = SwitchContext::builder()
                .checked(RwSignal::new(false))
                .disabled(Signal::stored(false))
                .on_change(on_change)
                .build();

            context.toggle();

            assert_eq!(last.get(), Some(true));
        });
    }

    #[test]
    fn on_change_fires_on_toggle_first_toggle_sets_checked() {
        reactive_scope(|| {
            let (on_change, _last) = track_callback::<bool>();
            let checked = RwSignal::new(false);
            let context = SwitchContext::builder()
                .checked(checked)
                .disabled(Signal::stored(false))
                .on_change(on_change)
                .build();

            context.toggle();

            assert!(checked.get());
        });
    }

    #[test]
    fn on_change_fires_on_toggle_second_toggle_clears_checked() {
        reactive_scope(|| {
            let (on_change, _last) = track_callback::<bool>();
            let checked = RwSignal::new(false);
            let context = SwitchContext::builder()
                .checked(checked)
                .disabled(Signal::stored(false))
                .on_change(on_change)
                .build();

            context.toggle();
            context.toggle();

            assert!(!checked.get());
        });
    }

    #[test]
    fn on_change_fires_on_toggle_second_toggle_reports_false() {
        reactive_scope(|| {
            let (on_change, last) = track_callback::<bool>();
            let context = SwitchContext::builder()
                .checked(RwSignal::new(false))
                .disabled(Signal::stored(false))
                .on_change(on_change)
                .build();

            context.toggle();
            context.toggle();

            assert_eq!(last.get(), Some(false));
        });
    }

    #[test]
    fn on_change_not_fired_when_disabled_keeps_checked_unset() {
        reactive_scope(|| {
            let (on_change, _last) = track_callback::<bool>();
            let checked = RwSignal::new(false);
            let context = SwitchContext::builder()
                .checked(checked)
                .disabled(Signal::stored(true))
                .on_change(on_change)
                .build();

            context.toggle();

            assert!(!checked.get());
        });
    }

    #[test]
    fn on_change_not_fired_when_disabled_keeps_last_empty() {
        reactive_scope(|| {
            let (on_change, last) = track_callback::<bool>();
            let context = SwitchContext::builder()
                .checked(RwSignal::new(false))
                .disabled(Signal::stored(true))
                .on_change(on_change)
                .build();

            context.toggle();

            assert_eq!(last.get(), None);
        });
    }

    #[test]
    fn switch_context_is_send_sync_and_unpin() {
        fn assert_send_sync_and_unpin<T: Send + Sync + Unpin>() {}

        assert_send_sync_and_unpin::<SwitchContext>();
    }
}
