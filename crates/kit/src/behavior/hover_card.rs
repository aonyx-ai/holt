use leptos::html::Div;
use leptos::portal::Portal;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use leptos::wasm_bindgen::closure::Closure;
use leptos_floating::{Align, FloatingOptions, Side, use_floating};

/// Default delay in milliseconds before the hover card opens.
pub const DEFAULT_OPEN_DELAY: i32 = 700;

/// Default delay in milliseconds before the hover card closes.
pub const DEFAULT_CLOSE_DELAY: i32 = 300;

/// Hover card behavior context that manages open state and delay timers.
#[derive(Copy, Clone)]
pub struct HoverCardContext {
    pub open: RwSignal<bool>,
    pub trigger_ref: NodeRef<leptos::html::Button>,
    open_delay: Signal<i32>,
    close_delay: Signal<i32>,
    /// Stores the current timeout handle so it can be cancelled.
    open_timer: RwSignal<Option<i32>>,
    close_timer: RwSignal<Option<i32>>,
}

impl HoverCardContext {
    pub fn new(open_delay: Signal<i32>, close_delay: Signal<i32>) -> Self {
        Self {
            open: RwSignal::new(false),
            trigger_ref: NodeRef::new(),
            open_delay,
            close_delay,
            open_timer: RwSignal::new(None),
            close_timer: RwSignal::new(None),
        }
    }

    pub fn is_open(&self) -> bool {
        self.open.get()
    }

    /// Schedule opening the hover card after the open delay.
    pub fn schedule_open(&self) {
        self.clear_close_timer();
        if self.is_open() {
            return;
        }
        let open = self.open;
        let open_timer = self.open_timer;
        let delay = self.open_delay.get();
        let handle = set_timeout(delay, move || {
            open.set(true);
            open_timer.set(None);
        });
        self.open_timer.set(Some(handle));
    }

    /// Schedule closing the hover card after the close delay.
    pub fn schedule_close(&self) {
        self.clear_open_timer();
        if !self.is_open() {
            return;
        }
        let open = self.open;
        let close_timer = self.close_timer;
        let delay = self.close_delay.get();
        let handle = set_timeout(delay, move || {
            open.set(false);
            close_timer.set(None);
        });
        self.close_timer.set(Some(handle));
    }

    /// Cancel any pending open timer.
    pub fn clear_open_timer(&self) {
        if let Some(handle) = self.open_timer.get() {
            clear_timeout(handle);
            self.open_timer.set(None);
        }
    }

    /// Cancel any pending close timer.
    pub fn clear_close_timer(&self) {
        if let Some(handle) = self.close_timer.get() {
            clear_timeout(handle);
            self.close_timer.set(None);
        }
    }

    /// Cancel all pending timers.
    pub fn clear_timers(&self) {
        self.clear_open_timer();
        self.clear_close_timer();
    }
}

/// Set a timeout using `web_sys` and return the handle for cancellation.
fn set_timeout(delay_ms: i32, f: impl FnOnce() + 'static) -> i32 {
    let closure = Closure::once_into_js(f);
    web_sys::window()
        .expect("should have window")
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            delay_ms,
        )
        .expect("should set timeout")
}

/// Clear a timeout by handle.
fn clear_timeout(handle: i32) {
    if let Some(window) = web_sys::window() {
        window.clear_timeout_with_handle(handle);
    }
}

/// Root hover card primitive that provides context.
#[component]
pub fn HoverCardRoot(
    #[prop(into, default = Signal::stored(DEFAULT_OPEN_DELAY))] open_delay: Signal<i32>,
    #[prop(into, default = Signal::stored(DEFAULT_CLOSE_DELAY))] close_delay: Signal<i32>,
    children: Children,
) -> impl IntoView {
    let context = HoverCardContext::new(open_delay, close_delay);

    provide_context(context);

    // Clean up timers on unmount
    on_cleanup(move || {
        context.clear_timers();
    });

    view! {
        <div data-state=move || {
            if context.open.get() { "open" } else { "closed" }
        }>{children()}</div>
    }
}

/// Hook to access hover card context.
pub fn use_hover_card() -> HoverCardContext {
    use_context::<HoverCardContext>().expect("use_hover_card must be called within HoverCardRoot")
}

/// Trigger element that opens the hover card on mouse enter.
#[component]
pub fn HoverCardTrigger(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let context = use_hover_card();

    view! {
        <button
            type="button"
            class=class
            node_ref=context.trigger_ref
            on:mouseenter=move |_| context.schedule_open()
            on:mouseleave=move |_| context.schedule_close()
            on:focusin=move |_| context.schedule_open()
            on:focusout=move |_| context.schedule_close()
        >
            {children()}
        </button>
    }
}

/// Content area that floats near the trigger when the hover card is open.
#[component]
pub fn HoverCardContent(
    #[prop(optional, into)] class: String,
    #[prop(into, default = Side::Bottom)] side: Side,
    #[prop(into, default = Align::Center)] align: Align,
    #[prop(into, default = 4.0)] side_offset: f64,
    children: ChildrenFn,
) -> impl IntoView {
    let context = use_hover_card();
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
                    class=class.clone()
                    data-state="open"
                    data-side=move || format!("{:?}", floating.side.get()).to_lowercase()
                    node_ref=content_ref
                    on:mouseenter=move |_| context.clear_close_timer()
                    on:mouseleave=move |_| context.schedule_close()
                    on:focusin=move |_| context.clear_close_timer()
                    on:focusout=move |_| context.schedule_close()
                    style:position="fixed"
                    style:left=move || format!("{}px", floating.x.get())
                    style:top=move || format!("{}px", floating.y.get())
                    style:z-index="50"
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
        assert_class_prop!(HoverCardTriggerProps, HoverCardContentProps,);
    }

    #[test]
    fn context_defaults_to_closed() {
        reactive_scope(|| {
            let ctx = HoverCardContext::new(
                Signal::stored(DEFAULT_OPEN_DELAY),
                Signal::stored(DEFAULT_CLOSE_DELAY),
            );
            assert!(!ctx.is_open());
        });
    }

    #[test]
    fn open_delay_defaults() {
        assert_eq!(DEFAULT_OPEN_DELAY, 700);
    }

    #[test]
    fn close_delay_defaults() {
        assert_eq!(DEFAULT_CLOSE_DELAY, 300);
    }
}
