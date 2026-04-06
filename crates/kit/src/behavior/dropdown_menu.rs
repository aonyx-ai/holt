use leptos::html::Div;
use leptos::portal::Portal;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use leptos::web_sys::KeyboardEvent;
use leptos_floating::{Align, FloatingOptions, Side, use_floating};

/// Dropdown menu behavior context that manages state and interactions
#[derive(Copy, Clone)]
pub struct DropdownMenuContext {
    pub open: RwSignal<bool>,
    pub trigger_ref: NodeRef<leptos::html::Button>,
}

impl DropdownMenuContext {
    pub fn new() -> Self {
        Self {
            open: RwSignal::new(false),
            trigger_ref: NodeRef::new(),
        }
    }

    pub fn is_open(&self) -> bool {
        self.open.get()
    }

    pub fn toggle(&self) {
        self.open.update(|open| *open = !*open);
    }

    pub fn close(&self) {
        self.open.set(false);
    }

    pub fn open(&self) {
        self.open.set(true);
    }

    pub fn focus_trigger(&self) {
        if let Some(el) = self.trigger_ref.get() {
            let _ = el.focus();
        }
    }
}

/// Root dropdown menu primitive that provides context
#[component]
pub fn DropdownMenuRoot(children: Children) -> impl IntoView {
    let context = DropdownMenuContext::new();

    provide_context(context);

    view! {
        <div class="relative" data-state=move || if context.open.get() { "open" } else { "closed" }>
            {children()}
        </div>
    }
}

/// Hook to access dropdown menu context
pub fn use_dropdown_menu() -> DropdownMenuContext {
    use_context::<DropdownMenuContext>()
        .expect("use_dropdown_menu must be called within DropdownMenuRoot")
}

/// Trigger button that opens/closes the dropdown menu
#[component]
pub fn DropdownMenuTrigger(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let context = use_dropdown_menu();

    let on_keydown = move |ev: KeyboardEvent| {
        let state = if context.is_open() {
            OpenState::Open
        } else {
            OpenState::Closed
        };
        let action = handle_trigger_keydown(&ev.key(), state);
        match action {
            TriggerKeyAction::None => {}
            TriggerKeyAction::Toggle => {
                ev.prevent_default();
                context.toggle();
            }
            TriggerKeyAction::Close => {
                ev.prevent_default();
                context.close();
            }
            TriggerKeyAction::OpenAndFocusFirst => {
                ev.prevent_default();
                context.open();
            }
        }
    };

    view! {
        <button
            type="button"
            aria-expanded=move || context.open.get()
            aria-haspopup="menu"
            class=class
            node_ref=context.trigger_ref
            on:click=move |_| context.toggle()
            on:keydown=on_keydown
            data-state=move || if context.open.get() { "open" } else { "closed" }
        >
            {children()}
        </button>
    }
}

/// Content area that shows when dropdown menu is open
#[component]
pub fn DropdownMenuContent(
    #[prop(optional, into)] class: String,
    #[prop(into, default = Side::Bottom)] side: Side,
    #[prop(into, default = Align::Start)] align: Align,
    #[prop(into, default = 4.0)] side_offset: f64,
    children: ChildrenFn,
) -> impl IntoView {
    let context = use_dropdown_menu();
    let children = StoredValue::new(children);
    let content_ref = NodeRef::<Div>::new();

    let floating_options = FloatingOptions {
        side,
        align,
        side_offset,
        align_offset: 0.0,
    };

    let floating = use_floating(context.trigger_ref, content_ref, floating_options);

    // Focus first item when content mounts
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
                            ContentKeyAction::CloseAndFocusTrigger => {
                                ev.prevent_default();
                                context.close();
                                context.focus_trigger();
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

/// Clickable menu item
#[component]
pub fn DropdownMenuItem(
    #[prop(optional, into)] class: String,
    #[prop(into, default = Signal::stored(false))] disabled: Signal<bool>,
    #[prop(optional_no_strip)] on_select: Option<Callback<()>>,
    children: Children,
) -> impl IntoView {
    let context = use_dropdown_menu();

    view! {
        <div
            role="menuitem"
            tabindex="0"
            class=class
            data-disabled=move || disabled.get()
            on:click=move |_| {
                if !disabled.get() {
                    if let Some(cb) = &on_select {
                        cb.run(());
                    }
                    context.close();
                    context.focus_trigger();
                }
            }
        >
            {children()}
        </div>
    }
}

/// Non-interactive label within the dropdown menu
#[component]
pub fn DropdownMenuLabel(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    view! { <div class=class>{children()}</div> }
}

/// Visual separator between menu items
#[component]
pub fn DropdownMenuSeparator(#[prop(optional, into)] class: String) -> impl IntoView {
    view! { <div role="separator" class=class /> }
}

/// Actions that can result from a keydown on the trigger
#[derive(Clone, Eq, PartialEq, Debug)]
enum TriggerKeyAction {
    None,
    Toggle,
    Close,
    OpenAndFocusFirst,
}

/// Actions that can result from a keydown within the menu content
#[derive(Clone, Eq, PartialEq, Debug)]
enum ContentKeyAction {
    None,
    FocusNext,
    FocusPrevious,
    ActivateFocused,
    CloseAndFocusTrigger,
}

/// Whether the dropdown menu is currently open or closed
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum OpenState {
    Open,
    Closed,
}

fn handle_trigger_keydown(key: &str, state: OpenState) -> TriggerKeyAction {
    match key {
        "Escape" if state == OpenState::Open => TriggerKeyAction::Close,
        "ArrowDown" => TriggerKeyAction::OpenAndFocusFirst,
        "Enter" | " " => TriggerKeyAction::Toggle,
        _ => TriggerKeyAction::None,
    }
}

fn handle_content_keydown(key: &str) -> ContentKeyAction {
    match key {
        "ArrowDown" => ContentKeyAction::FocusNext,
        "ArrowUp" => ContentKeyAction::FocusPrevious,
        "Enter" | " " => ContentKeyAction::ActivateFocused,
        "Escape" => ContentKeyAction::CloseAndFocusTrigger,
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

    #[test]
    fn class_prop_accepts_str_and_string() {
        assert_class_prop!(
            DropdownMenuTriggerProps,
            DropdownMenuContentProps,
            DropdownMenuItemProps,
            DropdownMenuLabelProps,
            DropdownMenuSeparatorProps,
        );
    }

    #[test]
    fn escape_closes_open_menu() {
        assert_eq!(
            handle_trigger_keydown("Escape", OpenState::Open),
            TriggerKeyAction::Close
        );
    }

    #[test]
    fn escape_on_closed_menu_is_noop() {
        assert_eq!(
            handle_trigger_keydown("Escape", OpenState::Closed),
            TriggerKeyAction::None
        );
    }

    #[test]
    fn arrow_down_opens_menu() {
        assert_eq!(
            handle_trigger_keydown("ArrowDown", OpenState::Closed),
            TriggerKeyAction::OpenAndFocusFirst
        );
    }

    #[test]
    fn enter_and_space_toggle_trigger() {
        assert_eq!(
            handle_trigger_keydown("Enter", OpenState::Closed),
            TriggerKeyAction::Toggle
        );
        assert_eq!(
            handle_trigger_keydown(" ", OpenState::Closed),
            TriggerKeyAction::Toggle
        );
    }

    #[test]
    fn arrow_down_in_content_focuses_next() {
        assert_eq!(
            handle_content_keydown("ArrowDown"),
            ContentKeyAction::FocusNext
        );
    }

    #[test]
    fn arrow_up_in_content_focuses_previous() {
        assert_eq!(
            handle_content_keydown("ArrowUp"),
            ContentKeyAction::FocusPrevious
        );
    }

    #[test]
    fn enter_and_space_in_content_activate_item() {
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
    fn escape_in_content_closes() {
        assert_eq!(
            handle_content_keydown("Escape"),
            ContentKeyAction::CloseAndFocusTrigger
        );
    }

    #[test]
    fn unhandled_keys_are_noop() {
        assert_eq!(
            handle_trigger_keydown("Tab", OpenState::Closed),
            TriggerKeyAction::None
        );
        assert_eq!(
            handle_trigger_keydown("a", OpenState::Closed),
            TriggerKeyAction::None
        );
        assert_eq!(handle_content_keydown("Tab"), ContentKeyAction::None);
        assert_eq!(handle_content_keydown("a"), ContentKeyAction::None);
    }
}
