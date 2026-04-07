use leptos::html::Div;
use leptos::portal::Portal;
use leptos::prelude::*;
use leptos_floating::{Align, FloatingOptions, Side, use_floating};

/// Default delay in milliseconds before the tooltip opens on hover.
const DEFAULT_OPEN_DELAY_MS: u64 = 700;

/// Default delay in milliseconds before the tooltip closes after leaving.
const DEFAULT_CLOSE_DELAY_MS: u64 = 300;

/// Context for Tooltip components managing visibility and positioning
#[derive(Copy, Clone)]
pub struct TooltipContext {
    pub open: RwSignal<bool>,
    pub trigger_ref: NodeRef<leptos::html::Button>,
    pub open_delay_ms: u64,
    pub close_delay_ms: u64,
    /// Incremented on each intent change to invalidate stale delayed actions.
    generation: RwSignal<u64>,
}

impl TooltipContext {
    pub fn new(open_delay_ms: u64, close_delay_ms: u64) -> Self {
        Self {
            open: RwSignal::new(false),
            trigger_ref: NodeRef::new(),
            open_delay_ms,
            close_delay_ms,
            generation: RwSignal::new(0),
        }
    }

    /// Bump the generation counter and return the new value. Any pending
    /// delayed action from a previous generation should be discarded.
    fn bump_generation(&self) -> u64 {
        self.generation.update(|g| *g += 1);
        self.generation.get_untracked()
    }

    pub fn show(&self) {
        self.open.set(true);
    }

    pub fn hide(&self) {
        self.open.set(false);
    }

    /// Cancel any pending delayed action and show the tooltip immediately.
    pub fn show_immediately(&self) {
        self.bump_generation();
        self.show();
    }

    /// Cancel any pending delayed action and hide the tooltip immediately.
    pub fn hide_immediately(&self) {
        self.bump_generation();
        self.hide();
    }

    pub fn is_open(&self) -> bool {
        self.open.get()
    }

    /// Schedule opening after the configured delay. A subsequent call to
    /// `schedule_close` (or another `schedule_open`) will cancel the pending
    /// open.
    pub fn schedule_open(&self) {
        let epoch = self.bump_generation();
        if self.open_delay_ms == 0 {
            self.show();
            return;
        }
        let ctx = *self;
        set_timeout(
            move || {
                if ctx.generation.get_untracked() == epoch {
                    ctx.show();
                }
            },
            std::time::Duration::from_millis(self.open_delay_ms),
        );
    }

    /// Schedule closing after the configured delay. A subsequent call to
    /// `schedule_open` (or another `schedule_close`) will cancel the pending
    /// close.
    pub fn schedule_close(&self) {
        let epoch = self.bump_generation();
        if self.close_delay_ms == 0 {
            self.hide();
            return;
        }
        let ctx = *self;
        set_timeout(
            move || {
                if ctx.generation.get_untracked() == epoch {
                    ctx.hide();
                }
            },
            std::time::Duration::from_millis(self.close_delay_ms),
        );
    }
}

/// Root tooltip primitive that provides context
#[component]
pub fn TooltipRoot(
    #[prop(into, default = DEFAULT_OPEN_DELAY_MS)] open_delay_ms: u64,
    #[prop(into, default = DEFAULT_CLOSE_DELAY_MS)] close_delay_ms: u64,
    children: Children,
) -> impl IntoView {
    let context = TooltipContext::new(open_delay_ms, close_delay_ms);
    provide_context(context);

    view! { <>{children()}</> }
}

/// Access the tooltip context
pub fn use_tooltip() -> TooltipContext {
    use_context::<TooltipContext>().expect("use_tooltip must be called within TooltipRoot")
}

/// Trigger element that shows the tooltip on hover and focus
#[component]
pub fn TooltipTrigger(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let context = use_tooltip();

    view! {
        <button
            type="button"
            class=class
            node_ref=context.trigger_ref
            on:mouseenter=move |_| context.schedule_open()
            on:mouseleave=move |_| context.schedule_close()
            on:focus=move |_| context.show_immediately()
            on:blur=move |_| context.hide_immediately()
            data-state=move || if context.is_open() { "open" } else { "closed" }
        >
            {children()}
        </button>
    }
}

/// Tooltip content displayed in a portal with floating positioning
#[component]
pub fn TooltipContent(
    #[prop(optional, into)] class: String,
    #[prop(into, default = Side::Top)] side: Side,
    #[prop(into, default = Align::Center)] align: Align,
    #[prop(into, default = 4.0)] side_offset: f64,
    children: ChildrenFn,
) -> impl IntoView {
    let context = use_tooltip();
    let children = StoredValue::new(children);
    let content_ref = NodeRef::<Div>::new();

    let floating_options = FloatingOptions {
        side,
        align,
        side_offset,
        align_offset: 0.0,
    };

    let floating = use_floating(context.trigger_ref, content_ref, floating_options);

    view! {
        <Show when=move || context.open.get() clone:class>
            <Portal clone:class>
                <div
                    role="tooltip"
                    class=class.clone()
                    node_ref=content_ref
                    style:position="fixed"
                    style:left=move || format!("{}px", floating.x.get())
                    style:top=move || format!("{}px", floating.y.get())
                    style:z-index="50"
                    data-side=move || format!("{:?}", floating.side.get()).to_lowercase()
                    data-state=move || if context.is_open() { "open" } else { "closed" }
                >
                    {children.read_value()()}
                </div>
            </Portal>
        </Show>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::reactive_scope;

    #[test]
    fn class_prop_accepts_str_and_string() {
        assert_class_prop!(TooltipTriggerProps, TooltipContentProps,);
    }

    #[test]
    fn context_show_hide() {
        reactive_scope(|| {
            let ctx = TooltipContext::new(0, 0);
            assert!(!ctx.is_open());

            ctx.show();
            assert!(ctx.is_open());

            ctx.hide();
            assert!(!ctx.is_open());
        });
    }

    #[test]
    fn default_delays() {
        reactive_scope(|| {
            let ctx = TooltipContext::new(DEFAULT_OPEN_DELAY_MS, DEFAULT_CLOSE_DELAY_MS);
            assert_eq!(ctx.open_delay_ms, 700);
            assert_eq!(ctx.close_delay_ms, 300);
        });
    }

    #[test]
    fn custom_delays() {
        reactive_scope(|| {
            let ctx = TooltipContext::new(100, 50);
            assert_eq!(ctx.open_delay_ms, 100);
            assert_eq!(ctx.close_delay_ms, 50);
        });
    }

    #[test]
    fn bump_generation_increments() {
        reactive_scope(|| {
            let ctx = TooltipContext::new(0, 0);
            let g1 = ctx.bump_generation();
            let g2 = ctx.bump_generation();
            assert_eq!(g2, g1 + 1);
        });
    }

    #[test]
    fn schedule_open_with_zero_delay_opens_immediately() {
        reactive_scope(|| {
            let ctx = TooltipContext::new(0, 0);
            assert!(!ctx.is_open());
            ctx.schedule_open();
            assert!(ctx.is_open());
        });
    }

    #[test]
    fn schedule_close_with_zero_delay_closes_immediately() {
        reactive_scope(|| {
            let ctx = TooltipContext::new(0, 0);
            ctx.show();
            assert!(ctx.is_open());
            ctx.schedule_close();
            assert!(!ctx.is_open());
        });
    }
}
