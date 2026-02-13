use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

// ── Pure logic tests (run on both native and WASM) ──

/// Test that pressing Escape on an open select closes it
#[wasm_bindgen_test(unsupported = test)]
fn test_escape_closes_select() {
    use holt_kit::behavior::select_keyboard::{KeyAction, handle_trigger_keydown};

    let action = handle_trigger_keydown("Escape", true);
    assert_eq!(action, KeyAction::Close);
}

/// Test that Escape on closed select does nothing
#[wasm_bindgen_test(unsupported = test)]
fn test_escape_on_closed_select_is_noop() {
    use holt_kit::behavior::select_keyboard::{KeyAction, handle_trigger_keydown};

    let action = handle_trigger_keydown("Escape", false);
    assert_eq!(action, KeyAction::None);
}

/// Test that ArrowDown on trigger opens and focuses first item
#[wasm_bindgen_test(unsupported = test)]
fn test_arrow_down_on_trigger_opens_select() {
    use holt_kit::behavior::select_keyboard::{KeyAction, handle_trigger_keydown};

    let action = handle_trigger_keydown("ArrowDown", false);
    assert_eq!(action, KeyAction::OpenAndFocusFirst);
}

/// Test that ArrowDown when already open focuses first item
#[wasm_bindgen_test(unsupported = test)]
fn test_arrow_down_on_open_trigger_focuses_first() {
    use holt_kit::behavior::select_keyboard::{KeyAction, handle_trigger_keydown};

    let action = handle_trigger_keydown("ArrowDown", true);
    assert_eq!(action, KeyAction::OpenAndFocusFirst);
}

/// Test that Enter/Space toggles the select open/closed
#[wasm_bindgen_test(unsupported = test)]
fn test_enter_toggles_trigger() {
    use holt_kit::behavior::select_keyboard::{KeyAction, handle_trigger_keydown};

    let action = handle_trigger_keydown("Enter", false);
    assert_eq!(action, KeyAction::Toggle);

    let action = handle_trigger_keydown(" ", false);
    assert_eq!(action, KeyAction::Toggle);
}

/// Test ArrowDown within content moves to next item
#[wasm_bindgen_test(unsupported = test)]
fn test_arrow_down_in_content_focuses_next() {
    use holt_kit::behavior::select_keyboard::{ContentKeyAction, handle_content_keydown};

    let action = handle_content_keydown("ArrowDown");
    assert_eq!(action, ContentKeyAction::FocusNext);
}

/// Test ArrowUp within content moves to previous item
#[wasm_bindgen_test(unsupported = test)]
fn test_arrow_up_in_content_focuses_previous() {
    use holt_kit::behavior::select_keyboard::{ContentKeyAction, handle_content_keydown};

    let action = handle_content_keydown("ArrowUp");
    assert_eq!(action, ContentKeyAction::FocusPrevious);
}

/// Test Enter in content selects the focused item
#[wasm_bindgen_test(unsupported = test)]
fn test_enter_in_content_selects_item() {
    use holt_kit::behavior::select_keyboard::{ContentKeyAction, handle_content_keydown};

    let action = handle_content_keydown("Enter");
    assert_eq!(action, ContentKeyAction::SelectFocused);
}

/// Test Space in content selects the focused item
#[wasm_bindgen_test(unsupported = test)]
fn test_space_in_content_selects_item() {
    use holt_kit::behavior::select_keyboard::{ContentKeyAction, handle_content_keydown};

    let action = handle_content_keydown(" ");
    assert_eq!(action, ContentKeyAction::SelectFocused);
}

/// Test Escape in content closes and returns focus to trigger
#[wasm_bindgen_test(unsupported = test)]
fn test_escape_in_content_closes() {
    use holt_kit::behavior::select_keyboard::{ContentKeyAction, handle_content_keydown};

    let action = handle_content_keydown("Escape");
    assert_eq!(action, ContentKeyAction::CloseAndFocusTrigger);
}

/// Test unhandled keys return None action
#[wasm_bindgen_test(unsupported = test)]
fn test_unhandled_keys_are_noop() {
    use holt_kit::behavior::select_keyboard::{
        ContentKeyAction, KeyAction, handle_content_keydown, handle_trigger_keydown,
    };

    assert_eq!(handle_trigger_keydown("Tab", false), KeyAction::None);
    assert_eq!(handle_trigger_keydown("a", false), KeyAction::None);
    assert_eq!(handle_content_keydown("Tab"), ContentKeyAction::None);
    assert_eq!(handle_content_keydown("a"), ContentKeyAction::None);
}

// ── DOM-based tests (WASM only) ──

/// Test focus navigation within a DOM listbox
#[wasm_bindgen_test(unsupported = test)]
#[cfg_attr(not(target_family = "wasm"), ignore)]
fn test_focus_navigation_within_dom() {
    use leptos::wasm_bindgen::JsCast;
    use leptos::web_sys::*;

    let document = web_sys::window().unwrap().document().unwrap();
    let body = document.body().unwrap();

    // Create a listbox container with option items
    let listbox = document.create_element("div").unwrap();
    listbox.set_attribute("role", "listbox").unwrap();

    for i in 0..3 {
        let item = document
            .create_element("div")
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap();
        item.set_attribute("role", "option").unwrap();
        item.set_attribute("tabindex", "0").unwrap();
        item.set_attribute("data-value", &format!("item-{}", i))
            .unwrap();
        item.set_inner_text(&format!("Item {}", i));
        listbox.append_child(&item).unwrap();
    }

    body.append_child(&listbox).unwrap();

    // Query all option items
    let items = listbox.query_selector_all("[role='option']").unwrap();
    assert_eq!(items.length(), 3);

    // Verify items are focusable (have tabindex)
    let first_item = items.item(0).unwrap().dyn_into::<HtmlElement>().unwrap();
    assert_eq!(first_item.get_attribute("tabindex").unwrap(), "0");

    // Focus first item
    first_item.focus().unwrap();
    let active = document.active_element().unwrap();
    assert_eq!(
        active.get_attribute("data-value").unwrap(),
        "item-0",
        "First item should be focused"
    );

    // Simulate moving to next: focus second item
    let second_item = items.item(1).unwrap().dyn_into::<HtmlElement>().unwrap();
    second_item.focus().unwrap();
    let active = document.active_element().unwrap();
    assert_eq!(
        active.get_attribute("data-value").unwrap(),
        "item-1",
        "Second item should be focused"
    );

    // Simulate moving to previous: focus first item again
    first_item.focus().unwrap();
    let active = document.active_element().unwrap();
    assert_eq!(
        active.get_attribute("data-value").unwrap(),
        "item-0",
        "First item should be focused again"
    );

    // Clean up
    body.remove_child(&listbox).unwrap();
}
