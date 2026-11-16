use leptos::prelude::*;

/// Toggle behavior context that manages state and interactions
///
/// # Examples
///
/// ```
/// use holt_kit::behavior::ToggleContext;
/// use holt_kit::testing::reactive_scope;
/// use leptos::prelude::*;
///
/// reactive_scope(|| {
///     let context = ToggleContext::builder()
///         .pressed(RwSignal::new(false))
///         .disabled(Signal::stored(false))
///         .build();
///
///     context.toggle();
///
///     assert!(context.is_pressed());
/// });
/// ```
#[derive(Clone, bon::Builder, getset::CopyGetters)]
pub struct ToggleContext {
    #[getset(get_copy = "pub")]
    pressed: RwSignal<bool>,
    #[getset(get_copy = "pub")]
    disabled: Signal<bool>,
    #[getset(get_copy = "pub")]
    on_change: Option<Callback<bool>>,
}

impl ToggleContext {
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

/// Renders the root toggle primitive that provides context and handles the underlying button
///
/// # Examples
///
/// ```
/// use holt_kit::behavior::ToggleRoot;
/// use holt_kit::testing::reactive_scope;
/// use leptos::prelude::*;
///
/// reactive_scope(|| {
///     let _view = view! { <ToggleRoot>"Label"</ToggleRoot> };
/// });
/// ```
#[component]
pub fn ToggleRoot(
    #[prop(optional, into)] class: String,
    #[prop(optional)] pressed: RwSignal<bool>,
    #[prop(into, default = Signal::stored(false))] disabled: Signal<bool>,
    #[prop(optional_no_strip, into)] aria_label: Option<&'static str>,
    #[prop(optional_no_strip)] on_change: Option<Callback<bool>>,
    children: Children,
) -> impl IntoView {
    let context = ToggleContext::builder()
        .pressed(pressed)
        .disabled(disabled)
        .maybe_on_change(on_change)
        .build();
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

/// Returns the toggle context
///
/// # Examples
///
/// ```
/// use holt_kit::behavior::{ToggleContext, use_toggle};
/// use holt_kit::testing::reactive_scope;
/// use leptos::prelude::*;
///
/// reactive_scope(|| {
///     let context = ToggleContext::builder()
///         .pressed(RwSignal::new(true))
///         .disabled(Signal::stored(false))
///         .build();
///     provide_context(context);
///
///     assert!(use_toggle().is_pressed());
/// });
/// ```
pub fn use_toggle() -> ToggleContext {
    use_context::<ToggleContext>().expect("use_toggle must be called within ToggleRoot")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{reactive_scope, track_callback};

    #[test]
    fn class_prop_accepts_str_and_string() {
        assert_class_prop!(ToggleRootProps);
    }

    #[test]
    fn on_change_fires_on_toggle_first_toggle_reports_true() {
        reactive_scope(|| {
            let (on_change, last) = track_callback::<bool>();
            let context = ToggleContext::builder()
                .pressed(RwSignal::new(false))
                .disabled(Signal::stored(false))
                .on_change(on_change)
                .build();

            context.toggle();

            assert_eq!(last.get(), Some(true));
        });
    }

    #[test]
    fn on_change_fires_on_toggle_first_toggle_sets_pressed() {
        reactive_scope(|| {
            let (on_change, _last) = track_callback::<bool>();
            let pressed = RwSignal::new(false);
            let context = ToggleContext::builder()
                .pressed(pressed)
                .disabled(Signal::stored(false))
                .on_change(on_change)
                .build();

            context.toggle();

            assert!(pressed.get());
        });
    }

    #[test]
    fn on_change_fires_on_toggle_second_toggle_clears_pressed() {
        reactive_scope(|| {
            let (on_change, _last) = track_callback::<bool>();
            let pressed = RwSignal::new(false);
            let context = ToggleContext::builder()
                .pressed(pressed)
                .disabled(Signal::stored(false))
                .on_change(on_change)
                .build();

            context.toggle();
            context.toggle();

            assert!(!pressed.get());
        });
    }

    #[test]
    fn on_change_fires_on_toggle_second_toggle_reports_false() {
        reactive_scope(|| {
            let (on_change, last) = track_callback::<bool>();
            let context = ToggleContext::builder()
                .pressed(RwSignal::new(false))
                .disabled(Signal::stored(false))
                .on_change(on_change)
                .build();

            context.toggle();
            context.toggle();

            assert_eq!(last.get(), Some(false));
        });
    }

    #[test]
    fn on_change_not_fired_when_disabled_keeps_last_empty() {
        reactive_scope(|| {
            let (on_change, last) = track_callback::<bool>();
            let context = ToggleContext::builder()
                .pressed(RwSignal::new(false))
                .disabled(Signal::stored(true))
                .on_change(on_change)
                .build();

            context.toggle();

            assert_eq!(last.get(), None);
        });
    }

    #[test]
    fn on_change_not_fired_when_disabled_keeps_pressed_unset() {
        reactive_scope(|| {
            let (on_change, _last) = track_callback::<bool>();
            let pressed = RwSignal::new(false);
            let context = ToggleContext::builder()
                .pressed(pressed)
                .disabled(Signal::stored(true))
                .on_change(on_change)
                .build();

            context.toggle();

            assert!(!pressed.get());
        });
    }

    #[test]
    fn toggle_context_is_send_sync_and_unpin() {
        fn assert_send_sync_and_unpin<T: Send + Sync + Unpin>() {}

        assert_send_sync_and_unpin::<ToggleContext>();
    }
}
