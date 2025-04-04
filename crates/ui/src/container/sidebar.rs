use leptos::{ev, prelude::*};
use leptos::children::Children;
use leptos::web_sys::KeyboardEvent;

// Define a context type for our sidebar
#[derive(Clone)]
pub struct SidebarContext {
    pub is_open: ReadSignal<bool>,
    pub set_open: WriteSignal<bool>,
}

impl SidebarContext {
    pub fn is_open(&self) -> bool {
        self.is_open.get()
    }
}

/// Provides the sidebar state management context
#[component]
pub fn SidebarProvider(
    #[prop(optional)] initial_state: Option<bool>,
    #[prop(optional)] open: Option<ReadSignal<bool>>,
    #[prop(optional)] set_open_prop: Option<WriteSignal<bool>>,
    #[prop(optional)] style: Option<&'static str>,
    children: Children
) -> impl IntoView {
    let initial_state = initial_state.unwrap_or(true);
    let (is_open_local, set_open_local) = signal(initial_state);

    // Use provided signals if available, otherwise use local ones
    let is_open = open.unwrap_or(is_open_local);
    let set_open = set_open_prop.unwrap_or(set_open_local);

    // Create and provide a unified context
    let context = SidebarContext {
        is_open,
        set_open,
    };

    provide_context(context);

    // // Default CSS variables for theming
    // let css_vars = r#"
    //     --sidebar-width: 16rem;
    //     --sidebar-width-mobile: 18rem;
    //     --sidebar-background: 0 0% 98%;
    //     --sidebar-foreground: 240 5.3% 26.1%;
    //     --sidebar-primary: 240 5.9% 10%;
    //     --sidebar-primary-foreground: 0 0% 98%;
    //     --sidebar-accent: 240 4.8% 95.9%;
    //     --sidebar-accent-foreground: 240 5.9% 10%;
    //     --sidebar-border: 220 13% 91%;
    //     --sidebar-ring: 217.2 91.2% 59.8%;
    // "#;

    // let styles = move || {
    //     if let Some(custom_style) = style {
    //         format!("{}; {}", css_vars, custom_style)
    //     } else {
    //         css_vars.to_string()
    //     }
    // };

    view! {
        {children()}
    }
}

/// Keyboard shortcut handler for sidebar toggle
#[component]
pub fn SidebarKeyboardShortcut() -> impl IntoView {
    let context = use_context::<SidebarContext>()
        .expect("SidebarProvider must be an ancestor");

    let handle_keydown = move |ev: KeyboardEvent| {
        // Check for Cmd+B or Ctrl+B
        if (ev.meta_key() || ev.ctrl_key()) && ev.key() == "b" {
            ev.prevent_default();
            context.set_open.update(|open| *open = !*open);
        }
    };

    // Add global event listener
    window_event_listener(ev::keydown, handle_keydown);

    // No visible UI
    view! { <div class="hidden"></div> }
}
