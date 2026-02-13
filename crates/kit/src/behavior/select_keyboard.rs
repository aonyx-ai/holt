use leptos::wasm_bindgen::JsCast;
use leptos::web_sys;

/// Actions that can result from a keydown on the select trigger
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyAction {
    /// No action for this key
    None,
    /// Toggle the dropdown open/closed
    Toggle,
    /// Close the dropdown
    Close,
    /// Open dropdown and focus the first (or selected) item
    OpenAndFocusFirst,
}

/// Actions that can result from a keydown within the select content/listbox
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContentKeyAction {
    /// No action for this key
    None,
    /// Focus the next item in the list
    FocusNext,
    /// Focus the previous item in the list
    FocusPrevious,
    /// Select the currently focused item
    SelectFocused,
    /// Close the dropdown and return focus to the trigger
    CloseAndFocusTrigger,
}

/// Determine the action for a keydown event on the select trigger.
///
/// `key` is the value of `KeyboardEvent::key()`.
/// `is_open` indicates whether the dropdown is currently open.
pub fn handle_trigger_keydown(key: &str, is_open: bool) -> KeyAction {
    match key {
        "Escape" if is_open => KeyAction::Close,
        "ArrowDown" => KeyAction::OpenAndFocusFirst,
        "Enter" | " " => KeyAction::Toggle,
        _ => KeyAction::None,
    }
}

/// Determine the action for a keydown event within the select content.
///
/// `key` is the value of `KeyboardEvent::key()`.
pub fn handle_content_keydown(key: &str) -> ContentKeyAction {
    match key {
        "ArrowDown" => ContentKeyAction::FocusNext,
        "ArrowUp" => ContentKeyAction::FocusPrevious,
        "Enter" | " " => ContentKeyAction::SelectFocused,
        "Escape" => ContentKeyAction::CloseAndFocusTrigger,
        _ => ContentKeyAction::None,
    }
}

/// Focus the next option item within a listbox container.
/// Returns true if focus was moved.
pub fn focus_next_item(container: &web_sys::Element) -> bool {
    focus_sibling_item(container, true)
}

/// Focus the previous option item within a listbox container.
/// Returns true if focus was moved.
pub fn focus_previous_item(container: &web_sys::Element) -> bool {
    focus_sibling_item(container, false)
}

/// Focus the first option item within a listbox container.
/// Returns true if an item was focused.
pub fn focus_first_item(container: &web_sys::Element) -> bool {
    let items = container.query_selector_all("[role='option']");
    let Ok(items) = items else { return false };
    if items.length() == 0 {
        return false;
    }
    if let Some(node) = items.item(0)
        && let Ok(el) = node.dyn_into::<web_sys::HtmlElement>()
    {
        let _ = el.focus();
        return true;
    }
    false
}

fn focus_sibling_item(container: &web_sys::Element, forward: bool) -> bool {
    let items = container.query_selector_all("[role='option']");
    let Ok(items) = items else { return false };

    let count = items.length();
    if count == 0 {
        return false;
    }

    // Find which item is currently focused
    let document = web_sys::window()
        .and_then(|w| w.document())
        .expect("should have document");
    let active = document.active_element();

    let mut current_index: Option<u32> = None;
    if let Some(ref active_el) = active {
        for i in 0..count {
            if let Some(node) = items.item(i)
                && let Ok(el) = node.dyn_into::<web_sys::Element>()
                && *active_el == el
            {
                current_index = Some(i);
                break;
            }
        }
    }

    let next_index = match current_index {
        Some(idx) => {
            if forward {
                if idx + 1 < count { idx + 1 } else { 0 }
            } else if idx > 0 {
                idx - 1
            } else {
                count - 1
            }
        }
        None => 0,
    };

    if let Some(node) = items.item(next_index)
        && let Ok(el) = node.dyn_into::<web_sys::HtmlElement>()
    {
        let _ = el.focus();
        return true;
    }
    false
}
