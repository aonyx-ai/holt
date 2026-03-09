use holt_book::SidebarContext;
use holt_kit::testing::reactive_scope;
use leptos::prelude::*;

#[test]
fn sidebar_starts_open_by_default() {
    reactive_scope(|| {
        let (is_open, _set_open) = signal(true);
        let (_is_mobile, _set_mobile) = signal(false);

        let context = SidebarContext {
            is_open,
            set_open: _set_open,
            is_mobile: _is_mobile,
        };

        assert!(context.is_open());
    });
}

#[test]
fn sidebar_toggle_flips_open_state() {
    reactive_scope(|| {
        let (is_open, set_open) = signal(true);
        let (is_mobile, _set_mobile) = signal(false);

        let context = SidebarContext {
            is_open,
            set_open,
            is_mobile,
        };

        assert!(context.is_open());

        context.toggle();
        assert!(!context.is_open());

        context.toggle();
        assert!(context.is_open());
    });
}
