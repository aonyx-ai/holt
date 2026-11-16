use leptos::children::Children;
use leptos::web_sys::KeyboardEvent;
use leptos::web_sys::window;
use leptos::{ev, prelude::*};

/// The sidebar context keeps state and behavior
///
/// # Examples
///
/// ```
/// use holt_kit::behavior::SidebarContext;
/// use holt_kit::testing::reactive_scope;
/// use leptos::prelude::*;
///
/// reactive_scope(|| {
///     let (is_open, set_open) = signal(true);
///     let (is_mobile, _) = signal(false);
///     let context = SidebarContext::builder()
///         .is_open(is_open)
///         .set_open(set_open)
///         .is_mobile(is_mobile)
///         .build();
///
///     context.set_open().set(false);
///
///     assert!(!context.is_open());
/// });
/// ```
#[derive(Copy, Clone, bon::Builder, getset::CopyGetters)]
pub struct SidebarContext {
    is_open: ReadSignal<bool>,
    #[getset(get_copy = "pub")]
    set_open: WriteSignal<bool>,
    #[getset(get_copy = "pub")]
    is_mobile: ReadSignal<bool>,
}

impl SidebarContext {
    pub fn is_open(&self) -> bool {
        self.is_open.get()
    }
}

/// Initial open/closed state for the sidebar
///
/// # Examples
///
/// ```
/// use holt_kit::behavior::SidebarInitialState;
///
/// assert!(SidebarInitialState::default() == SidebarInitialState::Open);
/// ```
#[derive(Copy, Clone, PartialEq, Default)]
pub enum SidebarInitialState {
    #[default]
    Open,
    Closed,
}

/// Renders the provider that shares the sidebar state with its descendants
///
/// The provider detects the mobile size with a simple window width check.
///
/// # Examples
///
/// ```no_run
/// use holt_kit::behavior::SidebarProvider;
/// use holt_kit::testing::reactive_scope;
/// use leptos::prelude::*;
///
/// reactive_scope(|| {
///     let _view = view! { <SidebarProvider>"Content"</SidebarProvider> };
/// });
/// ```
#[component]
pub fn SidebarProvider(
    #[prop(optional)] initial_state: SidebarInitialState,
    #[prop(optional)] open: Option<ReadSignal<bool>>,
    #[prop(optional)] set_open_prop: Option<WriteSignal<bool>>,
    children: Children,
) -> impl IntoView {
    let initial_state = initial_state == SidebarInitialState::Open;

    let (is_open_local, set_open_local) = signal(initial_state);
    let is_open = open.unwrap_or(is_open_local);
    let set_open = set_open_prop.unwrap_or(set_open_local);

    let (is_mobile, set_is_mobile) = signal(false);
    Effect::new(move |_| {
        if let Some(win) = window() {
            let width = win.inner_width().unwrap().as_f64().unwrap_or(9999.0);
            set_is_mobile.set(width < 768.0);
        }
    });

    let context = SidebarContext::builder()
        .is_open(is_open)
        .set_open(set_open)
        .is_mobile(is_mobile)
        .build();

    provide_context(context);

    view! {
        <div
            class="group/sidebar-wrapper flex min-h-svh w-full has-[[data-variant=inset]]:bg-sidebar"
            style="--sidebar-width: 16rem; --sidebar-width-icon: 3rem"
        >
            {children()}
        </div>
    }
}

/// Returns the keyboard shortcut for toggle
///
/// # Examples
///
/// ``` no_run
/// use holt_kit::behavior::{SidebarKeyboardShortcut, SidebarProvider};
/// use leptos::prelude::*;
///
/// let _view = view! {
///     <SidebarProvider>
///         <SidebarKeyboardShortcut />
///     </SidebarProvider>
/// };
/// ```
#[component]
pub fn SidebarKeyboardShortcut() -> impl IntoView {
    let context = use_context::<SidebarContext>().expect("SidebarProvider must be an ancestor");

    let handle_keydown = move |ev: KeyboardEvent| {
        if (ev.meta_key() || ev.ctrl_key()) && ev.key() == "b" {
            ev.prevent_default();
            context.set_open().update(|open| *open = !*open);
        }
    };

    window_event_listener(ev::keydown, handle_keydown);

    view! { <div class="hidden"></div> }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sidebar_context_is_send_sync_and_unpin() {
        fn assert_send_sync_and_unpin<T: Send + Sync + Unpin>() {}

        assert_send_sync_and_unpin::<SidebarContext>();
    }

    #[test]
    fn sidebar_initial_state_is_send_sync_and_unpin() {
        fn assert_send_sync_and_unpin<T: Send + Sync + Unpin>() {}

        assert_send_sync_and_unpin::<SidebarInitialState>();
    }
}
