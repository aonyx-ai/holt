use leptos::html::Div;
use leptos::portal::Portal;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use leptos::web_sys::{KeyboardEvent, MouseEvent};

/// Context menu behavior context that manages open state and cursor position
#[derive(Copy, Clone)]
pub struct ContextMenuContext {
    pub open: RwSignal<bool>,
    pub cursor_x: RwSignal<f64>,
    pub cursor_y: RwSignal<f64>,
}

impl ContextMenuContext {
    pub fn new() -> Self {
        Self {
            open: RwSignal::new(false),
            cursor_x: RwSignal::new(0.0),
            cursor_y: RwSignal::new(0.0),
        }
    }

    pub fn is_open(&self) -> bool {
        self.open.get()
    }

    pub fn open_at(&self, x: f64, y: f64) {
        self.cursor_x.set(x);
        self.cursor_y.set(y);
        self.open.set(true);
    }

    pub fn close(&self) {
        self.open.set(false);
    }
}

/// Root context menu primitive that provides context
#[component]
pub fn ContextMenuRoot(children: Children) -> impl IntoView {
    let context = ContextMenuContext::new();
    provide_context(context);

    view! { <>{children()}</> }
}

/// Hook to access context menu context
pub fn use_context_menu() -> ContextMenuContext {
    use_context::<ContextMenuContext>()
        .expect("use_context_menu must be called within ContextMenuRoot")
}

/// Trigger area that opens the context menu on right-click
#[component]
pub fn ContextMenuTrigger(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let context = use_context_menu();

    let on_contextmenu = move |ev: MouseEvent| {
        ev.prevent_default();
        context.open_at(ev.client_x() as f64, ev.client_y() as f64);
    };

    view! {
        <div class=class on:contextmenu=on_contextmenu>
            {children()}
        </div>
    }
}

/// Content area that appears at the cursor position when the context menu is open
#[component]
pub fn ContextMenuContent(
    #[prop(optional, into)] class: String,
    children: ChildrenFn,
) -> impl IntoView {
    let context = use_context_menu();
    let children = StoredValue::new(children);
    let content_ref = NodeRef::<Div>::new();

    // Close on click outside
    Effect::new(move |_| {
        if context.is_open() {
            request_animation_frame(move || {
                if let Some(el) = content_ref.get() {
                    let el: &web_sys::Element = &el;
                    focus_first_item(el);
                }
            });
        }
    });

    view! {
        <Show when=move || context.open.get() clone:class>
            <Portal clone:class>
                <div
                    role="menu"
                    class=class.clone()
                    data-state="open"
                    node_ref=content_ref
                    on:keydown=move |ev: KeyboardEvent| {
                        let action = handle_content_keydown(&ev.key());
                        match action {
                            ContentKeyAction::None => {}
                            ContentKeyAction::FocusNext => {
                                ev.prevent_default();
                                if let Some(el) = content_ref.get() {
                                    let el: &web_sys::Element = &el;
                                    focus_next_item(el);
                                }
                            }
                            ContentKeyAction::FocusPrevious => {
                                ev.prevent_default();
                                if let Some(el) = content_ref.get() {
                                    let el: &web_sys::Element = &el;
                                    focus_previous_item(el);
                                }
                            }
                            ContentKeyAction::ActivateFocused => {
                                ev.prevent_default();
                                if let Some(active) = web_sys::window()
                                    .and_then(|w| w.document())
                                    .and_then(|d| d.active_element())
                                {
                                    if let Ok(el) = active.dyn_into::<web_sys::HtmlElement>() {
                                        el.click();
                                    }
                                }
                            }
                            ContentKeyAction::Close => {
                                ev.prevent_default();
                                context.close();
                            }
                        }
                    }
                    style:position="fixed"
                    style:left=move || format!("{}px", context.cursor_x.get())
                    style:top=move || format!("{}px", context.cursor_y.get())
                    style:z-index="50"
                >
                    {children.read_value()()}
                </div>
            </Portal>
        </Show>
    }
}

/// Individual context menu item
#[component]
pub fn ContextMenuItem(
    #[prop(optional, into)] class: String,
    #[prop(into, default = Signal::stored(false))] disabled: Signal<bool>,
    #[prop(optional_no_strip)] on_select: Option<Callback<()>>,
    children: Children,
) -> impl IntoView {
    let context = use_context_menu();

    let on_click = move |_| {
        if !disabled.get() {
            if let Some(cb) = &on_select {
                cb.run(());
            }
            context.close();
        }
    };

    view! {
        <div
            role="menuitem"
            tabindex="0"
            class=class
            on:click=on_click
            data-disabled=move || disabled.get()
        >
            {children()}
        </div>
    }
}

/// Separator between context menu items
#[component]
pub fn ContextMenuSeparator(#[prop(optional, into)] class: String) -> impl IntoView {
    view! { <div role="separator" class=class /> }
}

/// Actions that can result from a keydown within the context menu content
#[derive(Clone, Eq, PartialEq, Debug)]
enum ContentKeyAction {
    None,
    FocusNext,
    FocusPrevious,
    ActivateFocused,
    Close,
}

fn handle_content_keydown(key: &str) -> ContentKeyAction {
    match key {
        "ArrowDown" => ContentKeyAction::FocusNext,
        "ArrowUp" => ContentKeyAction::FocusPrevious,
        "Enter" | " " => ContentKeyAction::ActivateFocused,
        "Escape" => ContentKeyAction::Close,
        _ => ContentKeyAction::None,
    }
}

/// Direction to move focus within the menu
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum FocusDirection {
    Forward,
    Backward,
}

fn focus_next_item(container: &web_sys::Element) -> bool {
    focus_sibling_item(container, FocusDirection::Forward)
}

fn focus_previous_item(container: &web_sys::Element) -> bool {
    focus_sibling_item(container, FocusDirection::Backward)
}

fn focus_first_item(container: &web_sys::Element) -> bool {
    let items = container.query_selector_all("[role='menuitem']");
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

fn focus_sibling_item(container: &web_sys::Element, direction: FocusDirection) -> bool {
    let items = container.query_selector_all("[role='menuitem']");
    let Ok(items) = items else { return false };

    let count = items.length();
    if count == 0 {
        return false;
    }

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
        Some(idx) => match direction {
            FocusDirection::Forward => {
                if idx + 1 < count {
                    idx + 1
                } else {
                    0
                }
            }
            FocusDirection::Backward => {
                if idx > 0 {
                    idx - 1
                } else {
                    count - 1
                }
            }
        },
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::reactive_scope;

    #[test]
    fn class_prop_accepts_str_and_string() {
        assert_class_prop!(
            ContextMenuTriggerProps,
            ContextMenuContentProps,
            ContextMenuItemProps,
            ContextMenuSeparatorProps,
        );
    }

    #[test]
    fn context_opens_at_position() {
        reactive_scope(|| {
            let context = ContextMenuContext::new();
            assert!(!context.is_open());

            context.open_at(100.0, 200.0);
            assert!(context.is_open());
            assert_eq!(context.cursor_x.get(), 100.0);
            assert_eq!(context.cursor_y.get(), 200.0);
        });
    }

    #[test]
    fn context_closes() {
        reactive_scope(|| {
            let context = ContextMenuContext::new();
            context.open_at(50.0, 75.0);
            assert!(context.is_open());

            context.close();
            assert!(!context.is_open());
        });
    }

    #[test]
    fn arrow_down_focuses_next() {
        assert_eq!(
            handle_content_keydown("ArrowDown"),
            ContentKeyAction::FocusNext
        );
    }

    #[test]
    fn arrow_up_focuses_previous() {
        assert_eq!(
            handle_content_keydown("ArrowUp"),
            ContentKeyAction::FocusPrevious
        );
    }

    #[test]
    fn enter_activates_focused() {
        assert_eq!(
            handle_content_keydown("Enter"),
            ContentKeyAction::ActivateFocused
        );
    }

    #[test]
    fn space_activates_focused() {
        assert_eq!(
            handle_content_keydown(" "),
            ContentKeyAction::ActivateFocused
        );
    }

    #[test]
    fn escape_closes_menu() {
        assert_eq!(handle_content_keydown("Escape"), ContentKeyAction::Close);
    }

    #[test]
    fn unhandled_keys_are_noop() {
        assert_eq!(handle_content_keydown("Tab"), ContentKeyAction::None);
        assert_eq!(handle_content_keydown("a"), ContentKeyAction::None);
    }
}
