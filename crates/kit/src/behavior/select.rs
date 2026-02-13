use crate::floating::{Align, FloatingOptions, Side, use_floating};
use leptos::html::Div;
use leptos::portal::Portal;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use leptos::web_sys::KeyboardEvent;

/// Select behavior context that manages state and interactions
#[derive(Clone, Copy)]
pub struct SelectContext {
    pub value: RwSignal<Option<String>>,
    pub open: RwSignal<bool>,
    pub disabled: Signal<bool>,
    pub trigger_ref: NodeRef<leptos::html::Button>,
}

impl SelectContext {
    pub fn new(value: RwSignal<Option<String>>, disabled: Signal<bool>) -> Self {
        Self {
            value,
            open: RwSignal::new(false),
            disabled,
            trigger_ref: NodeRef::new(),
        }
    }

    pub fn is_open(&self) -> bool {
        self.open.get()
    }

    pub fn is_disabled(&self) -> bool {
        self.disabled.get()
    }

    pub fn toggle(&self) {
        if !self.is_disabled() {
            self.open.update(|open| *open = !*open);
        }
    }

    pub fn close(&self) {
        self.open.set(false);
    }

    pub fn open(&self) {
        if !self.is_disabled() {
            self.open.set(true);
        }
    }

    pub fn select_value(&self, new_value: String) {
        if !self.is_disabled() {
            self.value.set(Some(new_value));
            self.close();
        }
    }

    pub fn get_value(&self) -> Option<String> {
        self.value.get()
    }

    pub fn focus_trigger(&self) {
        if let Some(el) = self.trigger_ref.get() {
            let _ = el.focus();
        }
    }
}

/// Root select primitive that provides context
#[component]
pub fn SelectRoot(
    #[prop(optional)] value: RwSignal<Option<String>>,
    #[prop(into, default = Signal::stored(false))] disabled: Signal<bool>,
    children: Children,
) -> impl IntoView {
    let context = SelectContext::new(value, disabled);

    provide_context(context);

    view! {
        <div class="relative" data-state=move || if context.open.get() { "open" } else { "closed" }>
            {children()}
        </div>
    }
}

/// Hook to access select context
pub fn use_select() -> SelectContext {
    use_context::<SelectContext>().expect("use_select must be called within SelectRoot")
}

/// Trigger button that opens/closes the select
#[component]
pub fn SelectTrigger(
    #[prop(optional, into)] class: String,
    #[prop(optional_no_strip, into)] id: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    let context = use_select();

    let on_keydown = move |ev: KeyboardEvent| {
        let action = handle_trigger_keydown(&ev.key(), context.is_open());
        match action {
            KeyAction::None => {}
            KeyAction::Toggle => {
                ev.prevent_default();
                context.toggle();
            }
            KeyAction::Close => {
                ev.prevent_default();
                context.close();
            }
            KeyAction::OpenAndFocusFirst => {
                ev.prevent_default();
                context.open();
                // Focus will be handled by SelectContent's on-mount effect
            }
        }
    };

    view! {
        <button
            type="button"
            role="combobox"
            aria-expanded=move || context.open.get()
            aria-haspopup="listbox"
            class=class
            id=id
            node_ref=context.trigger_ref
            on:click=move |_| context.toggle()
            on:keydown=on_keydown
            disabled=move || context.disabled.get()
            data-state=move || if context.open.get() { "open" } else { "closed" }
        >
            {children()}
        </button>
    }
}

/// Content area that shows when select is open
#[component]
pub fn SelectContent(
    #[prop(optional, into)] class: String,
    #[prop(into, default = Side::Bottom)] side: Side,
    #[prop(into, default = Align::Start)] align: Align,
    #[prop(into, default = 4.0)] side_offset: f64,
    children: ChildrenFn,
) -> impl IntoView {
    let context = use_select();
    let children = StoredValue::new(children);
    let content_ref = NodeRef::<Div>::new();

    // Set up floating positioning
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
            // Use request_animation_frame to ensure the DOM is rendered
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
                    role="listbox"
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
                            ContentKeyAction::SelectFocused => {
                                ev.prevent_default();
                                if let Some(active) = web_sys::window()
                                    .and_then(|w| w.document())
                                    .and_then(|d| d.active_element())
                                    && let Some(value) = active.get_attribute("data-value")
                                {
                                    context.select_value(value);
                                    context.focus_trigger();
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

/// Individual select item
#[component]
pub fn SelectItem(
    #[prop(into)] value: String,
    #[prop(optional, into)] class: String,
    #[prop(into, default = Signal::stored(false))] disabled: Signal<bool>,
    children: Children,
) -> impl IntoView {
    let context = use_select();
    let item_value = value.clone();
    let select_value = value.clone();
    let data_value = value.clone();

    let is_selected = Signal::derive(move || context.get_value().is_some_and(|v| v == item_value));

    view! {
        <div
            role="option"
            tabindex="0"
            aria-selected=move || is_selected.get()
            class=class
            data-value=data_value
            on:click=move |_| {
                if !disabled.get() {
                    context.select_value(select_value.clone());
                    context.focus_trigger();
                }
            }
            data-disabled=move || disabled.get()
            data-state=move || if is_selected.get() { "checked" } else { "unchecked" }
        >
            {children()}
        </div>
    }
}

/// Component to display current value or placeholder
#[component]
pub fn SelectValue(
    #[prop(optional_no_strip, into)] placeholder: Option<String>,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let context = use_select();
    let placeholder_text = placeholder.unwrap_or_default();

    view! {
        <span class=class>
            {move || context.get_value().unwrap_or_else(|| placeholder_text.clone())}
        </span>
    }
}

/// Actions that can result from a keydown on the select trigger
#[derive(Debug, Clone, PartialEq, Eq)]
enum KeyAction {
    None,
    Toggle,
    Close,
    OpenAndFocusFirst,
}

/// Actions that can result from a keydown within the select content/listbox
#[derive(Debug, Clone, PartialEq, Eq)]
enum ContentKeyAction {
    None,
    FocusNext,
    FocusPrevious,
    SelectFocused,
    CloseAndFocusTrigger,
}

fn handle_trigger_keydown(key: &str, is_open: bool) -> KeyAction {
    match key {
        "Escape" if is_open => KeyAction::Close,
        "ArrowDown" => KeyAction::OpenAndFocusFirst,
        "Enter" | " " => KeyAction::Toggle,
        _ => KeyAction::None,
    }
}

fn handle_content_keydown(key: &str) -> ContentKeyAction {
    match key {
        "ArrowDown" => ContentKeyAction::FocusNext,
        "ArrowUp" => ContentKeyAction::FocusPrevious,
        "Enter" | " " => ContentKeyAction::SelectFocused,
        "Escape" => ContentKeyAction::CloseAndFocusTrigger,
        _ => ContentKeyAction::None,
    }
}

fn focus_next_item(container: &web_sys::Element) -> bool {
    focus_sibling_item(container, true)
}

fn focus_previous_item(container: &web_sys::Element) -> bool {
    focus_sibling_item(container, false)
}

fn focus_first_item(container: &web_sys::Element) -> bool {
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

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test_configure;
    wasm_bindgen_test_configure!(run_in_browser);

    #[test]
    fn class_prop_accepts_str_and_string() {
        assert_class_prop!(
            SelectTriggerProps,
            SelectContentProps,
            SelectItemProps,
            SelectValueProps,
        );
    }

    #[test]
    fn escape_closes_open_select() {
        assert_eq!(handle_trigger_keydown("Escape", true), KeyAction::Close);
    }

    #[test]
    fn escape_on_closed_select_is_noop() {
        assert_eq!(handle_trigger_keydown("Escape", false), KeyAction::None);
    }

    #[test]
    fn arrow_down_opens_select() {
        assert_eq!(
            handle_trigger_keydown("ArrowDown", false),
            KeyAction::OpenAndFocusFirst
        );
    }

    #[test]
    fn arrow_down_on_open_trigger_focuses_first() {
        assert_eq!(
            handle_trigger_keydown("ArrowDown", true),
            KeyAction::OpenAndFocusFirst
        );
    }

    #[test]
    fn enter_and_space_toggle_trigger() {
        assert_eq!(handle_trigger_keydown("Enter", false), KeyAction::Toggle);
        assert_eq!(handle_trigger_keydown(" ", false), KeyAction::Toggle);
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
    fn enter_and_space_in_content_select_item() {
        assert_eq!(
            handle_content_keydown("Enter"),
            ContentKeyAction::SelectFocused
        );
        assert_eq!(handle_content_keydown(" "), ContentKeyAction::SelectFocused);
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
        assert_eq!(handle_trigger_keydown("Tab", false), KeyAction::None);
        assert_eq!(handle_trigger_keydown("a", false), KeyAction::None);
        assert_eq!(handle_content_keydown("Tab"), ContentKeyAction::None);
        assert_eq!(handle_content_keydown("a"), ContentKeyAction::None);
    }

    #[wasm_bindgen_test::wasm_bindgen_test(unsupported = test)]
    #[cfg_attr(not(target_family = "wasm"), ignore)]
    fn focus_navigation_within_dom() {
        use web_sys::*;

        let document = window().unwrap().document().unwrap();
        let body = document.body().unwrap();

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
            item.set_attribute("data-value", &format!("item-{i}"))
                .unwrap();
            listbox.append_child(&item).unwrap();
        }

        body.append_child(&listbox).unwrap();

        let active_value = || {
            document
                .active_element()
                .and_then(|el| el.get_attribute("data-value"))
        };

        // focus_first_item should land on item-0
        assert!(focus_first_item(&listbox));
        assert_eq!(active_value().as_deref(), Some("item-0"));

        // focus_next_item should advance to item-1
        assert!(focus_next_item(&listbox));
        assert_eq!(active_value().as_deref(), Some("item-1"));

        // focus_next_item again to item-2
        assert!(focus_next_item(&listbox));
        assert_eq!(active_value().as_deref(), Some("item-2"));

        // focus_next_item wraps around to item-0
        assert!(focus_next_item(&listbox));
        assert_eq!(active_value().as_deref(), Some("item-0"));

        // focus_previous_item wraps around to item-2
        assert!(focus_previous_item(&listbox));
        assert_eq!(active_value().as_deref(), Some("item-2"));

        // focus_previous_item back to item-1
        assert!(focus_previous_item(&listbox));
        assert_eq!(active_value().as_deref(), Some("item-1"));

        body.remove_child(&listbox).unwrap();
    }
}
