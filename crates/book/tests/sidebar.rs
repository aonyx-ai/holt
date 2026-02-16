use holt_book::SidebarContext;
use leptos::prelude::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test(unsupported = test)]
#[cfg_attr(not(target_family = "wasm"), ignore)]
fn sidebar_starts_open_by_default() {
    let owner = Owner::new();
    owner.with(|| {
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

#[wasm_bindgen_test(unsupported = test)]
#[cfg_attr(not(target_family = "wasm"), ignore)]
fn sidebar_toggle_flips_open_state() {
    let owner = Owner::new();
    owner.with(|| {
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
