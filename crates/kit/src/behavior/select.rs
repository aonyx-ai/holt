use crate::behavior::select_keyboard::{self, ContentKeyAction, KeyAction};
use crate::floating::{Align, FloatingOptions, Side, use_floating};
use leptos::html::Div;
use leptos::portal::Portal;
use leptos::prelude::*;
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
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional_no_strip, into)] id: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    let context = use_select();

    let on_keydown = move |ev: KeyboardEvent| {
        let action = select_keyboard::handle_trigger_keydown(&ev.key(), context.is_open());
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
    #[prop(optional, into)] class: Option<String>,
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
                    select_keyboard::focus_first_item(el);
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
                        let action = select_keyboard::handle_content_keydown(&ev.key());
                        match action {
                            ContentKeyAction::None => {}
                            ContentKeyAction::FocusNext => {
                                ev.prevent_default();
                                if let Some(el) = content_ref.get() {
                                    let el: &web_sys::Element = &el;
                                    select_keyboard::focus_next_item(el);
                                }
                            }
                            ContentKeyAction::FocusPrevious => {
                                ev.prevent_default();
                                if let Some(el) = content_ref.get() {
                                    let el: &web_sys::Element = &el;
                                    select_keyboard::focus_previous_item(el);
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
    #[prop(optional, into)] class: Option<String>,
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
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let context = use_select();
    let placeholder_text = placeholder.unwrap_or_default();

    view! {
        <span class=class>
            {move || context.get_value().unwrap_or_else(|| placeholder_text.clone())}
        </span>
    }
}
