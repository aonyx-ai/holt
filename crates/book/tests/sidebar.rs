use holt_book::SidebarContext;
use holt_kit::testing::reactive_scope;
use leptos::prelude::*;

#[test]
fn sidebar_starts_open_by_default() {
    reactive_scope(|| {
        let (is_open, _set_open) = signal(true);
        let (_is_mobile, _set_mobile) = signal(false);

        let context = SidebarContext::builder()
            .is_open(is_open)
            .set_open(_set_open)
            .is_mobile(_is_mobile)
            .build();

        assert!(context.is_open());
    });
}

#[test]
fn sidebar_toggle_closes_open_state() {
    reactive_scope(|| {
        let (is_open, set_open) = signal(true);
        let (is_mobile, _set_mobile) = signal(false);
        let context = SidebarContext::builder()
            .is_open(is_open)
            .set_open(set_open)
            .is_mobile(is_mobile)
            .build();

        context.toggle();

        assert!(!context.is_open());
    });
}

#[test]
fn sidebar_toggle_twice_restores_open_state() {
    reactive_scope(|| {
        let (is_open, set_open) = signal(true);
        let (is_mobile, _set_mobile) = signal(false);
        let context = SidebarContext::builder()
            .is_open(is_open)
            .set_open(set_open)
            .is_mobile(is_mobile)
            .build();

        context.toggle();
        context.toggle();

        assert!(context.is_open());
    });
}
