use holt_kit::behavior::{CheckboxContext, SelectContext, SwitchContext, ToggleContext};
use holt_kit::testing::{reactive_scope, track_callback};
use leptos::prelude::*;

// ── Checkbox ────────────────────────────────────────────────────────────────

#[test]
fn checkbox_on_change_fires_on_toggle() {
    reactive_scope(|| {
        let (on_change, last) = track_callback::<bool>();
        let checked = RwSignal::new(false);
        let disabled = Signal::stored(false);

        let context = CheckboxContext::new(checked, disabled, Some(on_change));

        context.toggle();
        assert!(checked.get());
        assert_eq!(last.get(), Some(true));

        context.toggle();
        assert!(!checked.get());
        assert_eq!(last.get(), Some(false));
    });
}

#[test]
fn checkbox_on_change_not_fired_when_disabled() {
    reactive_scope(|| {
        let (on_change, last) = track_callback::<bool>();
        let checked = RwSignal::new(false);
        let disabled = Signal::stored(true);

        let context = CheckboxContext::new(checked, disabled, Some(on_change));

        context.toggle();
        assert!(!checked.get());
        assert_eq!(last.get(), None);
    });
}

#[test]
fn checkbox_works_without_on_change() {
    reactive_scope(|| {
        let checked = RwSignal::new(false);
        let disabled = Signal::stored(false);

        let context = CheckboxContext::new(checked, disabled, None);

        context.toggle();
        assert!(checked.get());
    });
}

// ── Switch ──────────────────────────────────────────────────────────────────

#[test]
fn switch_on_change_fires_on_toggle() {
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
fn switch_on_change_not_fired_when_disabled() {
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

// ── Toggle ──────────────────────────────────────────────────────────────────

#[test]
fn toggle_on_change_fires_on_toggle() {
    reactive_scope(|| {
        let (on_change, last) = track_callback::<bool>();
        let pressed = RwSignal::new(false);
        let disabled = Signal::stored(false);

        let context = ToggleContext::new(pressed, disabled, Some(on_change));

        context.toggle();
        assert!(pressed.get());
        assert_eq!(last.get(), Some(true));

        context.toggle();
        assert!(!pressed.get());
        assert_eq!(last.get(), Some(false));
    });
}

#[test]
fn toggle_on_change_not_fired_when_disabled() {
    reactive_scope(|| {
        let (on_change, last) = track_callback::<bool>();
        let pressed = RwSignal::new(false);
        let disabled = Signal::stored(true);

        let context = ToggleContext::new(pressed, disabled, Some(on_change));

        context.toggle();
        assert!(!pressed.get());
        assert_eq!(last.get(), None);
    });
}

// ── Select ──────────────────────────────────────────────────────────────────

#[test]
fn select_on_change_fires_on_select_value() {
    reactive_scope(|| {
        let (on_change, last) = track_callback::<Option<String>>();
        let value = RwSignal::new(None::<String>);
        let disabled = Signal::stored(false);

        let context = SelectContext::new(value, disabled, Some(on_change));

        context.select_value("option1".to_string());
        assert_eq!(value.get(), Some("option1".to_string()));
        assert_eq!(last.get(), Some(Some("option1".to_string())));

        context.select_value("option2".to_string());
        assert_eq!(value.get(), Some("option2".to_string()));
        assert_eq!(last.get(), Some(Some("option2".to_string())));
    });
}

#[test]
fn select_on_change_not_fired_when_disabled() {
    reactive_scope(|| {
        let (on_change, last) = track_callback::<Option<String>>();
        let value = RwSignal::new(None::<String>);
        let disabled = Signal::stored(true);

        let context = SelectContext::new(value, disabled, Some(on_change));

        context.select_value("option1".to_string());
        assert_eq!(value.get(), None);
        assert_eq!(last.get(), None);
    });
}

#[test]
fn select_works_without_on_change() {
    reactive_scope(|| {
        let value = RwSignal::new(None::<String>);
        let disabled = Signal::stored(false);

        let context = SelectContext::new(value, disabled, None);

        context.select_value("test".to_string());
        assert_eq!(value.get(), Some("test".to_string()));
    });
}
