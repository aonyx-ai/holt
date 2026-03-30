use leptos::prelude::*;

/// Whether the toggle group allows single or multiple selections.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ToggleGroupType {
    #[default]
    Single,
    Multiple,
}

/// Context shared between `ToggleGroupRoot` and its `ToggleGroupItem` children.
#[derive(Clone)]
pub struct ToggleGroupContext {
    pub value: RwSignal<Vec<String>>,
    pub group_type: ToggleGroupType,
    pub disabled: Signal<bool>,
    pub on_value_change: Option<Callback<Vec<String>>>,
}

impl ToggleGroupContext {
    pub fn new(
        value: RwSignal<Vec<String>>,
        group_type: ToggleGroupType,
        disabled: Signal<bool>,
        on_value_change: Option<Callback<Vec<String>>>,
    ) -> Self {
        Self {
            value,
            group_type,
            disabled,
            on_value_change,
        }
    }

    pub fn is_disabled(&self) -> bool {
        self.disabled.get()
    }

    pub fn is_pressed(&self, item_value: &str) -> bool {
        self.value.get().contains(&item_value.to_string())
    }

    pub fn toggle(&self, item_value: &str) {
        if self.is_disabled() {
            return;
        }

        self.value.update(|values| {
            if let Some(pos) = values.iter().position(|v| v == item_value) {
                values.remove(pos);
            } else {
                match self.group_type {
                    ToggleGroupType::Single => {
                        values.clear();
                        values.push(item_value.to_string());
                    }
                    ToggleGroupType::Multiple => {
                        values.push(item_value.to_string());
                    }
                }
            }
        });

        if let Some(cb) = &self.on_value_change {
            cb.run(self.value.get());
        }
    }
}

/// Root container that provides toggle group context to its children.
#[component]
pub fn ToggleGroupRoot(
    #[prop(optional, into)] class: String,
    #[prop(optional)] value: RwSignal<Vec<String>>,
    #[prop(optional)] group_type: ToggleGroupType,
    #[prop(into, default = Signal::stored(false))] disabled: Signal<bool>,
    #[prop(optional_no_strip)] on_value_change: Option<Callback<Vec<String>>>,
    children: Children,
) -> impl IntoView {
    let context = ToggleGroupContext::new(value, group_type, disabled, on_value_change);
    provide_context(context);

    view! {
        <div role="group" class=class>
            {children()}
        </div>
    }
}

/// An individual toggle button within a toggle group.
#[component]
pub fn ToggleGroupItem(
    #[prop(optional, into)] class: String,
    #[prop(into)] value: String,
    #[prop(into, default = Signal::stored(false))] disabled: Signal<bool>,
    #[prop(optional_no_strip, into)] aria_label: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    let context = use_toggle_group();
    let item_value = value.clone();
    let item_value_click = value.clone();
    let item_value_state = value.clone();
    let ctx_click = context.clone();
    let ctx_disabled = context.clone();
    let ctx_state = context.clone();

    view! {
        <button
            type="button"
            role="button"
            aria-pressed=move || context.is_pressed(&item_value)
            class=class
            on:click=move |_| {
                if !disabled.get() {
                    ctx_click.toggle(&item_value_click);
                }
            }
            disabled=move || disabled.get() || ctx_disabled.is_disabled()
            data-state=move || {
                if ctx_state.is_pressed(&item_value_state) { "on" } else { "off" }
            }
            aria-label=aria_label
        >
            {children()}
        </button>
    }
}

/// Hook to access toggle group context.
pub fn use_toggle_group() -> ToggleGroupContext {
    use_context::<ToggleGroupContext>()
        .expect("use_toggle_group must be called within ToggleGroupRoot")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{reactive_scope, track_callback};

    #[test]
    fn class_prop_accepts_str_and_string() {
        assert_class_prop!(ToggleGroupRootProps, ToggleGroupItemProps);
    }

    #[test]
    fn single_mode_selects_one_at_a_time() {
        reactive_scope(|| {
            let value = RwSignal::new(vec![]);
            let context = ToggleGroupContext::new(
                value,
                ToggleGroupType::Single,
                Signal::stored(false),
                None,
            );

            context.toggle("a");
            assert_eq!(value.get(), vec!["a"]);

            context.toggle("b");
            assert_eq!(value.get(), vec!["b"]);
        });
    }

    #[test]
    fn single_mode_deselects_current() {
        reactive_scope(|| {
            let value = RwSignal::new(vec![]);
            let context = ToggleGroupContext::new(
                value,
                ToggleGroupType::Single,
                Signal::stored(false),
                None,
            );

            context.toggle("a");
            assert_eq!(value.get(), vec!["a"]);

            context.toggle("a");
            assert!(value.get().is_empty());
        });
    }

    #[test]
    fn multiple_mode_selects_many() {
        reactive_scope(|| {
            let value = RwSignal::new(vec![]);
            let context = ToggleGroupContext::new(
                value,
                ToggleGroupType::Multiple,
                Signal::stored(false),
                None,
            );

            context.toggle("a");
            context.toggle("b");
            assert_eq!(value.get(), vec!["a", "b"]);

            context.toggle("a");
            assert_eq!(value.get(), vec!["b"]);
        });
    }

    #[test]
    fn disabled_prevents_toggle() {
        reactive_scope(|| {
            let value = RwSignal::new(vec![]);
            let context =
                ToggleGroupContext::new(value, ToggleGroupType::Single, Signal::stored(true), None);

            context.toggle("a");
            assert!(value.get().is_empty());
        });
    }

    #[test]
    fn on_value_change_fires() {
        reactive_scope(|| {
            let (on_change, last) = track_callback::<Vec<String>>();
            let value = RwSignal::new(vec![]);
            let context = ToggleGroupContext::new(
                value,
                ToggleGroupType::Single,
                Signal::stored(false),
                Some(on_change),
            );

            context.toggle("a");
            assert_eq!(last.get(), Some(vec!["a".to_string()]));
        });
    }

    #[test]
    fn on_value_change_not_fired_when_disabled() {
        reactive_scope(|| {
            let (on_change, last) = track_callback::<Vec<String>>();
            let value = RwSignal::new(vec![]);
            let context = ToggleGroupContext::new(
                value,
                ToggleGroupType::Single,
                Signal::stored(true),
                Some(on_change),
            );

            context.toggle("a");
            assert_eq!(last.get(), None);
        });
    }
}
