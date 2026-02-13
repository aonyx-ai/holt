use holt_kit::behavior::{CheckboxContext, SelectContext, SwitchContext, ToggleContext};
use leptos::prelude::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

// ── Checkbox ────────────────────────────────────────────────────────────────

#[wasm_bindgen_test(unsupported = test)]
#[cfg_attr(not(target_family = "wasm"), ignore)]
fn checkbox_on_change_fires_on_toggle() {
    let owner = Owner::new();
    owner.with(|| {
        let callback_value = RwSignal::new(None::<bool>);
        let checked = RwSignal::new(false);
        let disabled = Signal::stored(false);
        let on_change = Callback::new(move |val: bool| {
            callback_value.set(Some(val));
        });

        let context = CheckboxContext::new(checked, disabled, Some(on_change));

        context.toggle();
        assert!(checked.get());
        assert_eq!(callback_value.get(), Some(true));

        context.toggle();
        assert!(!checked.get());
        assert_eq!(callback_value.get(), Some(false));
    });
}

#[wasm_bindgen_test(unsupported = test)]
#[cfg_attr(not(target_family = "wasm"), ignore)]
fn checkbox_on_change_not_fired_when_disabled() {
    let owner = Owner::new();
    owner.with(|| {
        let callback_value = RwSignal::new(None::<bool>);
        let checked = RwSignal::new(false);
        let disabled = Signal::stored(true);
        let on_change = Callback::new(move |val: bool| {
            callback_value.set(Some(val));
        });

        let context = CheckboxContext::new(checked, disabled, Some(on_change));

        context.toggle();
        assert!(!checked.get());
        assert_eq!(callback_value.get(), None);
    });
}

#[wasm_bindgen_test(unsupported = test)]
#[cfg_attr(not(target_family = "wasm"), ignore)]
fn checkbox_works_without_on_change() {
    let owner = Owner::new();
    owner.with(|| {
        let checked = RwSignal::new(false);
        let disabled = Signal::stored(false);

        let context = CheckboxContext::new(checked, disabled, None);

        context.toggle();
        assert!(checked.get());
    });
}

// ── Switch ──────────────────────────────────────────────────────────────────

#[wasm_bindgen_test(unsupported = test)]
#[cfg_attr(not(target_family = "wasm"), ignore)]
fn switch_on_change_fires_on_toggle() {
    let owner = Owner::new();
    owner.with(|| {
        let callback_value = RwSignal::new(None::<bool>);
        let checked = RwSignal::new(false);
        let disabled = Signal::stored(false);
        let on_change = Callback::new(move |val: bool| {
            callback_value.set(Some(val));
        });

        let context = SwitchContext::new(checked, disabled, Some(on_change));

        context.toggle();
        assert!(checked.get());
        assert_eq!(callback_value.get(), Some(true));

        context.toggle();
        assert!(!checked.get());
        assert_eq!(callback_value.get(), Some(false));
    });
}

#[wasm_bindgen_test(unsupported = test)]
#[cfg_attr(not(target_family = "wasm"), ignore)]
fn switch_on_change_not_fired_when_disabled() {
    let owner = Owner::new();
    owner.with(|| {
        let callback_value = RwSignal::new(None::<bool>);
        let checked = RwSignal::new(false);
        let disabled = Signal::stored(true);
        let on_change = Callback::new(move |val: bool| {
            callback_value.set(Some(val));
        });

        let context = SwitchContext::new(checked, disabled, Some(on_change));

        context.toggle();
        assert!(!checked.get());
        assert_eq!(callback_value.get(), None);
    });
}

// ── Toggle ──────────────────────────────────────────────────────────────────

#[wasm_bindgen_test(unsupported = test)]
#[cfg_attr(not(target_family = "wasm"), ignore)]
fn toggle_on_change_fires_on_toggle() {
    let owner = Owner::new();
    owner.with(|| {
        let callback_value = RwSignal::new(None::<bool>);
        let pressed = RwSignal::new(false);
        let disabled = Signal::stored(false);
        let on_change = Callback::new(move |val: bool| {
            callback_value.set(Some(val));
        });

        let context = ToggleContext::new(pressed, disabled, Some(on_change));

        context.toggle();
        assert!(pressed.get());
        assert_eq!(callback_value.get(), Some(true));

        context.toggle();
        assert!(!pressed.get());
        assert_eq!(callback_value.get(), Some(false));
    });
}

#[wasm_bindgen_test(unsupported = test)]
#[cfg_attr(not(target_family = "wasm"), ignore)]
fn toggle_on_change_not_fired_when_disabled() {
    let owner = Owner::new();
    owner.with(|| {
        let callback_value = RwSignal::new(None::<bool>);
        let pressed = RwSignal::new(false);
        let disabled = Signal::stored(true);
        let on_change = Callback::new(move |val: bool| {
            callback_value.set(Some(val));
        });

        let context = ToggleContext::new(pressed, disabled, Some(on_change));

        context.toggle();
        assert!(!pressed.get());
        assert_eq!(callback_value.get(), None);
    });
}

// ── Select ──────────────────────────────────────────────────────────────────

#[wasm_bindgen_test(unsupported = test)]
#[cfg_attr(not(target_family = "wasm"), ignore)]
fn select_on_change_fires_on_select_value() {
    let owner = Owner::new();
    owner.with(|| {
        let callback_value = RwSignal::new(None::<Option<String>>);
        let value = RwSignal::new(None::<String>);
        let disabled = Signal::stored(false);
        let on_change = Callback::new(move |val: Option<String>| {
            callback_value.set(Some(val));
        });

        let context = SelectContext::new(value, disabled, Some(on_change));

        context.select_value("option1".to_string());
        assert_eq!(value.get(), Some("option1".to_string()));
        assert_eq!(callback_value.get(), Some(Some("option1".to_string())));

        context.select_value("option2".to_string());
        assert_eq!(value.get(), Some("option2".to_string()));
        assert_eq!(callback_value.get(), Some(Some("option2".to_string())));
    });
}

#[wasm_bindgen_test(unsupported = test)]
#[cfg_attr(not(target_family = "wasm"), ignore)]
fn select_on_change_not_fired_when_disabled() {
    let owner = Owner::new();
    owner.with(|| {
        let callback_value = RwSignal::new(None::<Option<String>>);
        let value = RwSignal::new(None::<String>);
        let disabled = Signal::stored(true);
        let on_change = Callback::new(move |val: Option<String>| {
            callback_value.set(Some(val));
        });

        let context = SelectContext::new(value, disabled, Some(on_change));

        context.select_value("option1".to_string());
        assert_eq!(value.get(), None);
        assert_eq!(callback_value.get(), None);
    });
}

#[wasm_bindgen_test(unsupported = test)]
#[cfg_attr(not(target_family = "wasm"), ignore)]
fn select_works_without_on_change() {
    let owner = Owner::new();
    owner.with(|| {
        let value = RwSignal::new(None::<String>);
        let disabled = Signal::stored(false);

        let context = SelectContext::new(value, disabled, None);

        context.select_value("test".to_string());
        assert_eq!(value.get(), Some("test".to_string()));
    });
}
