use leptos::prelude::*;

/// Whether the accordion allows single or multiple items to be open at once.
#[derive(Clone, Copy, Default, PartialEq)]
pub enum AccordionType {
    /// Only one item can be open at a time (default).
    #[default]
    Single,
    /// Multiple items can be open simultaneously.
    Multiple,
}

/// Context for managing which accordion items are open.
#[derive(Clone, Copy)]
pub struct AccordionContext {
    pub accordion_type: AccordionType,
    pub open_items: RwSignal<Vec<String>>,
}

impl AccordionContext {
    /// Toggle an item by value. In single mode, opening one item closes others.
    pub fn toggle(&self, value: &str) {
        self.open_items.update(|items| {
            if let Some(pos) = items.iter().position(|v| v == value) {
                items.remove(pos);
            } else {
                match self.accordion_type {
                    AccordionType::Single => {
                        items.clear();
                        items.push(value.to_string());
                    }
                    AccordionType::Multiple => {
                        items.push(value.to_string());
                    }
                }
            }
        });
    }

    /// Check if an item is currently open.
    pub fn is_open(&self, value: &str) -> bool {
        self.open_items
            .with(|items| items.iter().any(|v| v == value))
    }
}

/// Context for an individual accordion item.
#[derive(Clone)]
pub struct AccordionItemContext {
    pub value: String,
}

/// Root accordion primitive that provides context.
#[component]
pub fn AccordionRoot(
    #[prop(optional, into)] class: String,
    #[prop(optional)] accordion_type: AccordionType,
    #[prop(optional, into)] default_value: Vec<String>,
    children: Children,
) -> impl IntoView {
    let ctx = AccordionContext {
        accordion_type,
        open_items: RwSignal::new(default_value),
    };
    provide_context(ctx);

    view! {
        <div class=class>
            {children()}
        </div>
    }
}

/// Access the accordion context.
pub fn use_accordion() -> AccordionContext {
    use_context::<AccordionContext>().expect("use_accordion must be called within AccordionRoot")
}

/// Access the accordion item context.
pub fn use_accordion_item() -> AccordionItemContext {
    use_context::<AccordionItemContext>()
        .expect("use_accordion_item must be called within AccordionItem")
}

/// An individual accordion item wrapper.
#[component]
pub fn AccordionItem(
    #[prop(optional, into)] class: String,
    #[prop(into)] value: String,
    children: Children,
) -> impl IntoView {
    let ctx = use_accordion();
    let value_clone = value.clone();
    let item_ctx = AccordionItemContext {
        value: value.clone(),
    };
    provide_context(item_ctx);

    view! {
        <div
            class=class
            data-state=move || if ctx.is_open(&value_clone) { "open" } else { "closed" }
        >
            {children()}
        </div>
    }
}

/// A trigger button that toggles the accordion item.
#[component]
pub fn AccordionTrigger(
    #[prop(optional, into)] class: String,
    #[prop(into, default = Signal::stored(false))] disabled: Signal<bool>,
    children: Children,
) -> impl IntoView {
    let ctx = use_accordion();
    let item_ctx = use_accordion_item();
    let value_click = item_ctx.value.clone();
    let value_expanded = item_ctx.value.clone();
    let value_state = item_ctx.value.clone();

    let on_click = move |_| {
        if !disabled.get() {
            ctx.toggle(&value_click);
        }
    };

    view! {
        <h3>
            <button
                type="button"
                on:click=on_click
                class=class
                aria-expanded=move || ctx.is_open(&value_expanded)
                data-state=move || if ctx.is_open(&value_state) { "open" } else { "closed" }
                disabled=disabled
            >
                {children()}
            </button>
        </h3>
    }
}

/// Content area that is shown/hidden based on accordion state.
#[component]
pub fn AccordionContent(
    #[prop(optional, into)] class: String,
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = use_accordion();
    let item_ctx = use_accordion_item();
    let value_hidden = item_ctx.value.clone();
    let value_state = item_ctx.value.clone();

    view! {
        <div
            class=class
            hidden=move || !ctx.is_open(&value_hidden)
            data-state=move || if ctx.is_open(&value_state) { "open" } else { "closed" }
        >
            {children()}
        </div>
    }
    .into_any()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn class_prop_accepts_str_and_string() {
        assert_class_prop!(
            AccordionRootProps,
            AccordionItemProps,
            AccordionTriggerProps,
            AccordionContentProps,
        );
    }

    #[test]
    fn single_mode_closes_other_items() {
        let ctx = AccordionContext {
            accordion_type: AccordionType::Single,
            open_items: RwSignal::new(vec![]),
        };

        ctx.toggle("a");
        assert!(ctx.is_open("a"));

        ctx.toggle("b");
        assert!(!ctx.is_open("a"));
        assert!(ctx.is_open("b"));
    }

    #[test]
    fn multiple_mode_keeps_items_open() {
        let ctx = AccordionContext {
            accordion_type: AccordionType::Multiple,
            open_items: RwSignal::new(vec![]),
        };

        ctx.toggle("a");
        ctx.toggle("b");
        assert!(ctx.is_open("a"));
        assert!(ctx.is_open("b"));
    }

    #[test]
    fn toggle_closes_open_item() {
        let ctx = AccordionContext {
            accordion_type: AccordionType::Single,
            open_items: RwSignal::new(vec![]),
        };

        ctx.toggle("a");
        assert!(ctx.is_open("a"));

        ctx.toggle("a");
        assert!(!ctx.is_open("a"));
    }
}
