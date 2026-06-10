use leptos::prelude::*;

/// Radio group behavior context that manages the selected value
#[derive(Clone)]
pub struct RadioGroupContext {
    pub value: RwSignal<Option<String>>,
    pub disabled: Signal<bool>,
    pub on_change: Option<Callback<String>>,
}

impl RadioGroupContext {
    pub fn new(
        value: RwSignal<Option<String>>,
        disabled: Signal<bool>,
        on_change: Option<Callback<String>>,
    ) -> Self {
        Self {
            value,
            disabled,
            on_change,
        }
    }

    pub fn selected_value(&self) -> Option<String> {
        self.value.get()
    }

    pub fn is_disabled(&self) -> bool {
        self.disabled.get()
    }

    pub fn select(&self, new_value: String) {
        if !self.is_disabled() {
            self.value.set(Some(new_value.clone()));
            if let Some(cb) = &self.on_change {
                cb.run(new_value);
            }
        }
    }

    pub fn is_selected(&self, item_value: &str) -> bool {
        self.value.get().as_deref() == Some(item_value)
    }
}

/// Root radio group primitive that provides context and handles the container
#[component]
pub fn RadioGroupRoot(
    #[prop(optional, into)] class: String,
    #[prop(optional)] value: RwSignal<Option<String>>,
    #[prop(into, default = Signal::stored(false))] disabled: Signal<bool>,
    #[prop(optional_no_strip)] on_change: Option<Callback<String>>,
    children: Children,
) -> impl IntoView {
    let context = RadioGroupContext::new(value, disabled, on_change);
    provide_context(context);

    view! {
        <div role="radiogroup" class=class>
            {children()}
        </div>
    }
}

/// Hook to access radio group context
pub fn use_radio_group() -> RadioGroupContext {
    use_context::<RadioGroupContext>()
        .expect("use_radio_group must be called within RadioGroupRoot")
}

/// Individual radio button item within a radio group
#[component]
pub fn RadioGroupItemRoot(
    #[prop(optional, into)] class: String,
    #[prop(into)] value: String,
    #[prop(into, default = Signal::stored(false))] disabled: Signal<bool>,
    children: Children,
) -> impl IntoView {
    let context = use_radio_group();
    let item_value = value.clone();
    let item_value_click = value.clone();
    let ctx_click = context.clone();
    let ctx_disabled = context.clone();
    let ctx_state = context.clone();

    view! {
        <button
            type="button"
            role="radio"
            aria-checked=move || context.is_selected(&item_value)
            class=class
            on:click=move |_| {
                if !disabled.get() {
                    ctx_click.select(item_value_click.clone());
                }
            }
            disabled=move || disabled.get() || ctx_disabled.is_disabled()
            data-state=move || {
                if ctx_state.is_selected(&value) { "checked" } else { "unchecked" }
            }
        >
            {children()}
        </button>
    }
}

/// Indicator that shows when the radio item is selected
#[component]
pub fn RadioGroupIndicator(
    #[prop(optional, into)] class: String,
    #[prop(into)] value: String,
    children: ChildrenFn,
) -> impl IntoView {
    let context = use_radio_group();

    view! {
        <Show when=move || context.is_selected(&value)>
            <span class=class.clone()>{children()}</span>
        </Show>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{reactive_scope, track_callback};

    #[test]
    fn class_prop_accepts_str_and_string() {
        assert_class_prop!(
            RadioGroupRootProps,
            RadioGroupItemRootProps,
            RadioGroupIndicatorProps
        );
    }

    #[test]
    fn select_updates_value() {
        reactive_scope(|| {
            let value = RwSignal::new(None::<String>);
            let disabled = Signal::stored(false);

            let context = RadioGroupContext::new(value, disabled, None);

            context.select("option1".to_string());
            assert_eq!(value.get(), Some("option1".to_string()));

            context.select("option2".to_string());
            assert_eq!(value.get(), Some("option2".to_string()));
        });
    }

    #[test]
    fn on_change_fires_on_select() {
        reactive_scope(|| {
            let (on_change, last) = track_callback::<String>();
            let value = RwSignal::new(None::<String>);
            let disabled = Signal::stored(false);

            let context = RadioGroupContext::new(value, disabled, Some(on_change));

            context.select("option1".to_string());
            assert_eq!(value.get(), Some("option1".to_string()));
            assert_eq!(last.get(), Some("option1".to_string()));
        });
    }

    #[test]
    fn on_change_not_fired_when_disabled() {
        reactive_scope(|| {
            let (on_change, last) = track_callback::<String>();
            let value = RwSignal::new(None::<String>);
            let disabled = Signal::stored(true);

            let context = RadioGroupContext::new(value, disabled, Some(on_change));

            context.select("option1".to_string());
            assert_eq!(value.get(), None);
            assert_eq!(last.get(), None);
        });
    }

    #[test]
    fn is_selected_returns_correct_value() {
        reactive_scope(|| {
            let value = RwSignal::new(Some("option1".to_string()));
            let disabled = Signal::stored(false);

            let context = RadioGroupContext::new(value, disabled, None);

            assert!(context.is_selected("option1"));
            assert!(!context.is_selected("option2"));
        });
    }

    #[test]
    fn works_without_on_change() {
        reactive_scope(|| {
            let value = RwSignal::new(None::<String>);
            let disabled = Signal::stored(false);

            let context = RadioGroupContext::new(value, disabled, None);

            context.select("option1".to_string());
            assert_eq!(value.get(), Some("option1".to_string()));
        });
    }
}
