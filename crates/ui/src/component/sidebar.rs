use leptos::children::Children;
use leptos::prelude::*;

use crate::container::SidebarContext;

/// The sidebar component that displays on the left side of the layout
#[component]
pub fn Sidebar(
    #[prop(optional)] class: &'static str,
    #[prop(optional, default = "left")] side: &'static str,
    #[prop(optional, default = "sidebar")] variant: &'static str,
    #[prop(optional, default = "icon")] collapsible: &'static str,
    children: Children,
) -> impl IntoView {
    let context = use_context::<SidebarContext>().expect("SidebarProvider must be an ancestor");

    let classes = move || {
        let mut classes = "h-full flex flex-col bg-sidebar-background text-sidebar-foreground border-sidebar-border transition-all duration-200 ease-in-out".to_string();

        // Add side-specific classes
        if side == "left" {
            classes.push_str(" border-r");
        } else if side == "right" {
            classes.push_str(" border-l");
        }

        // Add variant-specific classes
        match variant {
            "sidebar" => classes.push_str(" w-[16rem] md:w-[16rem]"),
            "floating" => classes.push_str(" rounded-lg shadow-lg"),
            "inset" => classes.push_str(" rounded-lg"),
            _ => classes.push_str(" w-[16rem] md:w-[16rem]"),
        }

        // Add custom classes
        if !class.is_empty() {
            classes.push(' ');
            classes.push_str(class);
        }

        classes
    };

    view! {
        <aside
            class=classes
            class:hidden=move || !context.is_open()
            data-side=side
            data-variant=variant
            data-collapsible=collapsible
        >
            {children()}
        </aside>
    }
}

/// The header component of the sidebar, shown at the top
#[component]
pub fn SidebarHeader(#[prop(optional)] class: &'static str, children: Children) -> impl IntoView {
    let classes = move || {
        let mut classes =
            "sticky top-0 h-[60px] flex items-center border-b border-sidebar-border px-4 py-2"
                .to_string();
        if !class.is_empty() {
            classes.push(' ');
            classes.push_str(class);
        }
        classes
    };

    view! {
        <div class=classes>
            {children()}
        </div>
    }
}

/// The content area of the sidebar, typically containing groups and navigation items
#[component]
pub fn SidebarContent(#[prop(optional)] class: &'static str, children: Children) -> impl IntoView {
    let classes = move || {
        let mut classes = "flex-1 overflow-auto py-2".to_string();
        if !class.is_empty() {
            classes.push(' ');
            classes.push_str(class);
        }
        classes
    };

    view! {
        <div class=classes>
            {children()}
        </div>
    }
}

/// A group within the sidebar content, used to organize menu items
#[component]
pub fn SidebarGroup(#[prop(optional)] class: &'static str, children: Children) -> impl IntoView {
    let classes = move || {
        let mut classes = "px-3 py-2".to_string();
        if !class.is_empty() {
            classes.push(' ');
            classes.push_str(class);
        }
        classes
    };

    view! {
        <div class=classes>
            {children()}
        </div>
    }
}

/// A label for a sidebar group
#[component]
pub fn SidebarGroupLabel(
    #[prop(optional)] class: &'static str,
    children: Children,
) -> impl IntoView {
    let classes = move || {
        let mut classes = "text-xs font-medium text-sidebar-foreground/70 mb-2".to_string();
        if !class.is_empty() {
            classes.push(' ');
            classes.push_str(class);
        }
        classes
    };

    view! {
        <h3 class=classes>
            {children()}
        </h3>
    }
}

/// Content container for sidebar group items
#[component]
pub fn SidebarGroupContent(
    #[prop(optional)] class: &'static str,
    children: Children,
) -> impl IntoView {
    let classes = move || {
        let mut classes = "space-y-1".to_string();
        if !class.is_empty() {
            classes.push(' ');
            classes.push_str(class);
        }
        classes
    };

    view! {
        <div class=classes>
            {children()}
        </div>
    }
}

/// A menu component within the sidebar
#[component]
pub fn SidebarMenu(#[prop(optional)] class: &'static str, children: Children) -> impl IntoView {
    let classes = move || {
        let mut classes = "space-y-1".to_string();
        if !class.is_empty() {
            classes.push(' ');
            classes.push_str(class);
        }
        classes
    };

    view! {
        <nav class=classes>
            {children()}
        </nav>
    }
}

/// A menu item within the sidebar menu
#[component]
pub fn SidebarMenuItem(#[prop(optional)] class: &'static str, children: Children) -> impl IntoView {
    let classes = move || {
        let mut classes = "relative".to_string();
        if !class.is_empty() {
            classes.push(' ');
            classes.push_str(class);
        }
        classes
    };

    view! {
        <div class=classes>
            {children()}
        </div>
    }
}

/// A button within a sidebar menu item
#[component]
pub fn SidebarMenuButton(
    #[prop(optional)] class: &'static str,
    #[prop(optional)] is_active: bool,
    children: Children,
) -> impl IntoView {
    let classes = move || {
        let mut classes = "group flex items-center w-full rounded-md px-3 py-2 text-sm font-medium hover:bg-sidebar-accent hover:text-sidebar-accent-foreground cursor-pointer transition-colors peer/menu-button".to_string();

        if is_active {
            classes.push_str(" bg-sidebar-accent text-sidebar-accent-foreground");
        }

        if !class.is_empty() {
            classes.push(' ');
            classes.push_str(class);
        }

        classes
    };

    view! {
        <button
            class=classes
            data-active=move || if is_active { "true" } else { "false" }
        >
            {children()}
        </button>
    }
}

/// The footer component of the sidebar, shown at the bottom
#[component]
pub fn SidebarFooter(#[prop(optional)] class: &'static str, children: Children) -> impl IntoView {
    let classes = move || {
        let mut classes =
            "sticky bottom-0 h-[60px] flex items-center border-t border-sidebar-border px-4 py-2"
                .to_string();
        if !class.is_empty() {
            classes.push(' ');
            classes.push_str(class);
        }
        classes
    };

    view! {
        <div class=classes>
            {children()}
        </div>
    }
}

/// A separator line within the sidebar
#[component]
pub fn SidebarSeparator(#[prop(optional)] class: &'static str) -> impl IntoView {
    let classes = move || {
        let mut classes = "h-px bg-sidebar-border my-2".to_string();
        if !class.is_empty() {
            classes.push(' ');
            classes.push_str(class);
        }
        classes
    };

    view! {
        <div class=classes></div>
    }
}

/// A button that toggles the sidebar visibility
#[component]
pub fn SidebarTrigger(#[prop(optional)] class: &'static str) -> impl IntoView {
    let context = use_context::<SidebarContext>().expect("SidebarProvider must be an ancestor");

    let toggle_sidebar = move |_| {
        context.set_open.update(|open| *open = !*open);
    };

    let classes = move || {
        let mut classes = "inline-flex items-center justify-center rounded-md text-sm font-medium h-9 w-9 px-0 transition-colors hover:bg-sidebar-accent hover:text-sidebar-accent-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-sidebar-ring".to_string();
        if !class.is_empty() {
            classes.push(' ');
            classes.push_str(class);
        }
        classes
    };

    view! {
        <button
            class=classes
            on:click=toggle_sidebar
            title="Toggle sidebar"
            aria-label="Toggle sidebar"
        >
            "≡"
        </button>
    }
}
