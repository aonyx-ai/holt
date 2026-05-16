use leptos::html::Div;
use leptos::portal::Portal;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use leptos::web_sys::KeyboardEvent;
use leptos_floating::{Align, FloatingOptions, Side, use_floating};

/// Context shared across all menus in a menubar.
///
/// Tracks which menu (if any) is currently open so that hovering over a
/// sibling trigger can transfer the open state without an extra click.
#[derive(Copy, Clone)]
pub struct MenubarContext {
    /// The `value` of the currently-open menu, or `None` when all are closed.
    pub open_menu: RwSignal<Option<String>>,
}

impl MenubarContext {
    pub fn new() -> Self {
        Self {
            open_menu: RwSignal::new(None),
        }
    }

    pub fn is_any_open(&self) -> bool {
        self.open_menu.get().is_some()
    }

    pub fn open(&self, value: &str) {
        self.open_menu.set(Some(value.to_string()));
    }

    pub fn close(&self) {
        self.open_menu.set(None);
    }

    pub fn is_open(&self, value: &str) -> bool {
        self.open_menu.get().as_deref() == Some(value)
    }
}

/// Context for a single menu within the menubar.
#[derive(Copy, Clone)]
pub struct MenubarMenuContext {
    pub value: StoredValue<String>,
    pub trigger_ref: NodeRef<leptos::html::Button>,
}

/// Root container for the menubar. Provides [`MenubarContext`].
#[component]
pub fn MenubarRoot(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let context = MenubarContext::new();
    provide_context(context);

    view! {
        <div class=class role="menubar">
            {children()}
        </div>
    }
}

/// Access the menubar context.
pub fn use_menubar() -> MenubarContext {
    use_context::<MenubarContext>().expect("use_menubar must be called within MenubarRoot")
}

/// Access the menu context.
pub fn use_menubar_menu() -> MenubarMenuContext {
    use_context::<MenubarMenuContext>().expect("use_menubar_menu must be called within MenubarMenu")
}

/// A single menu within the menubar. Each menu has a unique `value` that
/// the root context uses to track which one is open.
#[component]
pub fn MenubarMenu(#[prop(into)] value: String, children: Children) -> impl IntoView {
    let menu_ctx = MenubarMenuContext {
        value: StoredValue::new(value),
        trigger_ref: NodeRef::new(),
    };
    provide_context(menu_ctx);

    view! { <div class="relative">{children()}</div> }
}

/// Trigger button that opens/closes its parent menu.
#[component]
pub fn MenubarTrigger(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let bar = use_menubar();
    let menu = use_menubar_menu();

    let is_open = Signal::derive(move || bar.is_open(&menu.value.get_value()));

    let on_click = move |_| {
        if is_open.get() {
            bar.close();
        } else {
            bar.open(&menu.value.get_value());
        }
    };

    let on_pointerenter = move |_| {
        if bar.is_any_open() {
            bar.open(&menu.value.get_value());
        }
    };

    let on_keydown = move |ev: KeyboardEvent| {
        let action = handle_trigger_keydown(&ev.key(), is_open.get());
        match action {
            TriggerKeyAction::None => {}
            TriggerKeyAction::Toggle => {
                ev.prevent_default();
                if is_open.get() {
                    bar.close();
                } else {
                    bar.open(&menu.value.get_value());
                }
            }
            TriggerKeyAction::Close => {
                ev.prevent_default();
                bar.close();
            }
            TriggerKeyAction::Open => {
                ev.prevent_default();
                bar.open(&menu.value.get_value());
            }
        }
    };

    view! {
        <button
            type="button"
            role="menuitem"
            aria-haspopup="menu"
            aria-expanded=move || is_open.get()
            class=class
            node_ref=menu.trigger_ref
            on:click=on_click
            on:pointerenter=on_pointerenter
            on:keydown=on_keydown
            data-state=move || if is_open.get() { "open" } else { "closed" }
        >
            {children()}
        </button>
    }
}

/// Dropdown content for a menu. Rendered in a [`Portal`] with floating
/// positioning anchored to the trigger.
#[component]
pub fn MenubarContent(
    #[prop(optional, into)] class: String,
    #[prop(into, default = Side::Bottom)] side: Side,
    #[prop(into, default = Align::Start)] align: Align,
    #[prop(into, default = 4.0)] side_offset: f64,
    children: ChildrenFn,
) -> impl IntoView {
    let bar = use_menubar();
    let menu = use_menubar_menu();
    let children = StoredValue::new(children);
    let content_ref = NodeRef::<Div>::new();

    let is_open = Signal::derive(move || bar.is_open(&menu.value.get_value()));

    let floating_options = FloatingOptions {
        side,
        align,
        side_offset,
        align_offset: 0.0,
    };

    let floating = use_floating(menu.trigger_ref, content_ref, floating_options);

    // Focus first item when content opens
    Effect::new(move |_| {
        if is_open.get() {
            request_animation_frame(move || {
                if let Some(el) = content_ref.get() {
                    let el: &web_sys::Element = &el;
                    focus_first_item(el);
                }
            });
        }
    });

    view! {
        <Show when=move || is_open.get() clone:class>
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
                                bar.close();
                                if let Some(el) = menu.trigger_ref.get() {
                                    let _ = el.focus();
                                }
                            }
                            ContentKeyAction::Close => {
                                ev.prevent_default();
                                bar.close();
                                if let Some(el) = menu.trigger_ref.get() {
                                    let _ = el.focus();
                                }
                            }
                        }
                    }
                    style:position="fixed"
                    style:left=move || format!("{}px", floating.x.get())
                    style:top=move || format!("{}px", floating.y.get())
                    style:z-index="50"
                    data-side=move || format!("{:?}", floating.side.get()).to_lowercase()
                >
                    {children.read_value()()}
                </div>
            </Portal>
        </Show>
    }
}

/// A clickable menu item.
#[component]
pub fn MenubarItem(
    #[prop(optional, into)] class: String,
    #[prop(into, default = Signal::stored(false))] disabled: Signal<bool>,
    #[prop(optional_no_strip)] on_select: Option<Callback<()>>,
    children: Children,
) -> impl IntoView {
    let bar = use_menubar();

    let on_click = move |_| {
        if !disabled.get() {
            if let Some(cb) = &on_select {
                cb.run(());
            }
            bar.close();
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

/// A horizontal separator within a menu dropdown.
#[component]
pub fn MenubarSeparator(#[prop(optional, into)] class: String) -> impl IntoView {
    view! { <div role="separator" class=class /> }
}

// --- Keyboard handling ---

#[derive(Clone, Eq, PartialEq, Debug)]
enum TriggerKeyAction {
    None,
    Toggle,
    Close,
    Open,
}

#[derive(Clone, Eq, PartialEq, Debug)]
enum ContentKeyAction {
    None,
    FocusNext,
    FocusPrevious,
    ActivateFocused,
    Close,
}

fn handle_trigger_keydown(key: &str, is_open: bool) -> TriggerKeyAction {
    match key {
        "Enter" | " " => TriggerKeyAction::Toggle,
        "ArrowDown" => TriggerKeyAction::Open,
        "Escape" if is_open => TriggerKeyAction::Close,
        _ => TriggerKeyAction::None,
    }
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

// --- Focus helpers ---

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum FocusDirection {
    Forward,
    Backward,
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

fn focus_next_item(container: &web_sys::Element) -> bool {
    focus_sibling_item(container, FocusDirection::Forward)
}

fn focus_previous_item(container: &web_sys::Element) -> bool {
    focus_sibling_item(container, FocusDirection::Backward)
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
            MenubarRootProps,
            MenubarTriggerProps,
            MenubarContentProps,
            MenubarItemProps,
            MenubarSeparatorProps,
        );
    }

    #[test]
    fn open_menu_tracks_value() {
        reactive_scope(|| {
            let ctx = MenubarContext::new();
            assert!(!ctx.is_any_open());
            assert!(!ctx.is_open("file"));

            ctx.open("file");
            assert!(ctx.is_any_open());
            assert!(ctx.is_open("file"));
            assert!(!ctx.is_open("edit"));

            ctx.open("edit");
            assert!(ctx.is_open("edit"));
            assert!(!ctx.is_open("file"));

            ctx.close();
            assert!(!ctx.is_any_open());
        });
    }

    #[test]
    fn enter_and_space_toggle_trigger() {
        assert_eq!(
            handle_trigger_keydown("Enter", false),
            TriggerKeyAction::Toggle
        );
        assert_eq!(handle_trigger_keydown(" ", false), TriggerKeyAction::Toggle);
        assert_eq!(
            handle_trigger_keydown("Enter", true),
            TriggerKeyAction::Toggle
        );
    }

    #[test]
    fn arrow_down_opens_trigger() {
        assert_eq!(
            handle_trigger_keydown("ArrowDown", false),
            TriggerKeyAction::Open
        );
    }

    #[test]
    fn escape_closes_open_trigger() {
        assert_eq!(
            handle_trigger_keydown("Escape", true),
            TriggerKeyAction::Close
        );
    }

    #[test]
    fn escape_on_closed_trigger_is_noop() {
        assert_eq!(
            handle_trigger_keydown("Escape", false),
            TriggerKeyAction::None
        );
    }

    #[test]
    fn content_arrow_keys() {
        assert_eq!(
            handle_content_keydown("ArrowDown"),
            ContentKeyAction::FocusNext
        );
        assert_eq!(
            handle_content_keydown("ArrowUp"),
            ContentKeyAction::FocusPrevious
        );
    }

    #[test]
    fn content_enter_activates() {
        assert_eq!(
            handle_content_keydown("Enter"),
            ContentKeyAction::ActivateFocused
        );
        assert_eq!(
            handle_content_keydown(" "),
            ContentKeyAction::ActivateFocused
        );
    }

    #[test]
    fn content_escape_closes() {
        assert_eq!(handle_content_keydown("Escape"), ContentKeyAction::Close);
    }

    #[test]
    fn unhandled_keys_are_noop() {
        assert_eq!(handle_trigger_keydown("Tab", false), TriggerKeyAction::None);
        assert_eq!(handle_trigger_keydown("a", true), TriggerKeyAction::None);
        assert_eq!(handle_content_keydown("Tab"), ContentKeyAction::None);
        assert_eq!(handle_content_keydown("a"), ContentKeyAction::None);
    }
}
