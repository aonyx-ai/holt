use leptos::children::Children;
use leptos::prelude::*;
use tailwind_fuse::*;

use crate::behavior::SidebarContext;

#[derive(TwClass)]
#[tw(
    class = "h-full flex flex-col bg-sidebar-background text-sidebar-foreground border-sidebar-border transition-all duration-200 ease-in-out"
)]
struct SidebarStyle {
    variant: SidebarVariant,
    side: SidebarSide,
    collapsible: SidebarCollapsible,
}

#[derive(TwVariant)]
pub enum SidebarVariant {
    #[tw(default, class = "")]
    Sidebar,
    #[tw(class = "")]
    Floating,
    #[tw(class = "")]
    Inset,
}

#[derive(TwVariant)]
pub enum SidebarSide {
    #[tw(default, class = "border-r")]
    Left,
    #[tw(class = "border-l")]
    Right,
}

#[derive(TwVariant, PartialEq)]
pub enum SidebarCollapsible {
    #[tw(default, class = "w-[60px] md:w-[60px]")]
    Icon,
    #[tw(class = "-translate-x-full")]
    OffCanvas,
    #[tw(class = "")]
    None,
}

/// The sidebar component that displays on the left side of the layout
#[component]
pub fn Sidebar(
    #[prop(optional)] class: &'static str,
    #[prop(optional)] side: SidebarSide,
    #[prop(optional)] variant: SidebarVariant,
    #[prop(optional)] collapsible: SidebarCollapsible,
    children: Children,
) -> impl IntoView {
    let context = use_context::<SidebarContext>().expect("SidebarProvider must be an ancestor");

    let classes = SidebarStyle {
        side,
        variant,
        collapsible,
    }
    .with_class(class);

    view! {
        <aside
            class=classes
            class:hidden=move || collapsible != SidebarCollapsible::None && !context.is_open() && context.is_mobile.get()
            data-side=move || match side {
                SidebarSide::Left => "left",
                SidebarSide::Right => "right",
            }
            data-variant=move || match variant {
                SidebarVariant::Sidebar => "sidebar",
                SidebarVariant::Floating => "floating",
                SidebarVariant::Inset => "inset",
            }
            data-collapsible=move || match collapsible {
                SidebarCollapsible::Icon => "icon",
                SidebarCollapsible::OffCanvas => "offcanvas",
                SidebarCollapsible::None => "none",
            }
        >
            {children()}
        </aside>
    }
    // data-state=move || match *context.is_open.read() {
    //     true => "expanded",
    //     false => "collapsed",
    // }
}

/// A rail can be used to toggle the sidebar
#[component]
pub fn SidebarRail(#[prop(optional)] class: &'static str) -> impl IntoView {
    let context = use_context::<SidebarContext>().expect("SidebarProvider must be an ancestor");

    let classes = move || {
        let mut classes =
            "absolute top-0 right-0 h-full w-1 bg-sidebar-border cursor-ew-resize".to_string();
        if !class.is_empty() {
            classes.push(' ');
            classes.push_str(class);
        }
        classes
    };

    view! {
        <div
            class=classes
            on:click=move |_| context.set_open.update(|open| *open = !*open)
        ></div>
    }
}

/// Badge component for menu items
#[component]
pub fn SidebarMenuBadge(
    #[prop(optional)] class: &'static str,
    children: Children,
) -> impl IntoView {
    let classes = move || {
        let mut classes = "ml-auto flex h-5 items-center justify-center rounded-full bg-sidebar-accent px-2 text-xs font-medium text-sidebar-accent-foreground".to_string();
        if !class.is_empty() {
            classes.push(' ');
            classes.push_str(class);
        }
        classes
    };

    view! {
        <span class=classes>
            {children()}
        </span>
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
