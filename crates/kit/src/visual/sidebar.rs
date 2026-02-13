use leptos::children::Children;
use leptos::prelude::*;
use tailwind_fuse::*;

use crate::behavior::SidebarContext;

#[derive(TwClass)]
#[tw(
    class = "h-full flex flex-col bg-sidebar text-sidebar-foreground transition-all duration-200 ease-in-out"
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
    #[tw(class = "")]
    OffCanvas,
    #[tw(default, class = "")]
    Icon,
    #[tw(class = "")]
    None,
}

/// The sidebar component that displays on the left or right side of the layout
#[component]
pub fn Sidebar(
    #[prop(optional, into)] class: String,
    #[prop(optional, default = SidebarSide::Left)] side: SidebarSide,
    #[prop(optional, default = SidebarVariant::Sidebar)] variant: SidebarVariant,
    #[prop(optional, default = SidebarCollapsible::OffCanvas)] collapsible: SidebarCollapsible,
    children: Children,
) -> impl IntoView {
    let context = use_context::<SidebarContext>().expect("SidebarProvider must be an ancestor");

    // Handle the case when collapsible is None
    if collapsible == SidebarCollapsible::None {
        let simple_classes = {
            let mut classes =
                "flex h-full w-[var(--sidebar-width)] flex-col bg-sidebar text-sidebar-foreground"
                    .to_string();
            if !class.is_empty() {
                classes.push(' ');
                classes.push_str(&class);
            }
            classes
        };

        return view! { <div class=simple_classes>{children()}</div> }.into_any();
    }

    // Handle mobile view
    let is_mobile = context.is_mobile.get();
    if is_mobile {
        // For mobile, we don't have Sheet component in this codebase
        // So we'll create a simplified mobile view
        let class_suffix = if class.is_empty() {
            String::new()
        } else {
            format!(" {class}")
        };
        let mobile_classes = move || {
            let mut classes = "fixed inset-y-0 z-50 flex h-svh w-[var(--sidebar-width-mobile)] flex-col bg-sidebar p-0 text-sidebar-foreground transition-transform".to_string();

            if !context.is_open() {
                match side {
                    SidebarSide::Left => classes.push_str(" -translate-x-full"),
                    SidebarSide::Right => classes.push_str(" translate-x-full"),
                }
            }

            match side {
                SidebarSide::Left => classes.push_str(" left-0"),
                SidebarSide::Right => classes.push_str(" right-0"),
            }

            classes.push_str(&class_suffix);

            classes
        };

        return view! {
            <div
                data-sidebar="sidebar"
                data-mobile="true"
                class=mobile_classes
                style="var(--sidebar-width): SIDEBAR_WIDTH_MOBILE"
            >
                <div class="flex h-full w-full flex-col">{children()}</div>
            </div>
        }
        .into_any();
    }

    // Desktop view
    let state = move || {
        if context.is_open() {
            "expanded"
        } else {
            "collapsed"
        }
    };
    let other_state = state.clone();
    let collapsible_value = move || {
        if other_state() == "collapsed" {
            match collapsible {
                SidebarCollapsible::Icon => "icon",
                SidebarCollapsible::OffCanvas => "offcanvas",
                SidebarCollapsible::None => "",
            }
        } else {
            ""
        }
    };

    let variant_value = move || match variant {
        SidebarVariant::Sidebar => "sidebar",
        SidebarVariant::Floating => "floating",
        SidebarVariant::Inset => "inset",
    };

    let side_value = move || match side {
        SidebarSide::Left => "left",
        SidebarSide::Right => "right",
    };

    // Outer container classes
    let outer_classes = "group peer hidden text-sidebar-foreground md:block";

    // Gap div classes
    let gap_classes = move || {
        let base = "relative w-[var(--sidebar-width)] bg-transparent transition-[width] duration-200 ease-linear".to_string();
        let mut classes = vec![base];

        classes.push(String::from("group-data-[collapsible=offcanvas]:w-0"));
        classes.push(String::from("group-data-[side=right]:rotate-180"));

        match variant {
            SidebarVariant::Floating | SidebarVariant::Inset => {
                classes.push("group-data-[collapsible=icon]:w-[calc(var(--sidebar-width-icon)+_theme(spacing.4))]".to_string());
            }
            _ => {
                classes.push(
                    "group-data-[collapsible=icon]:w-[var(--sidebar-width-icon)]".to_string(),
                );
            }
        }

        classes.join(" ")
    };

    // Main sidebar container classes - pre-compute the class-dependent part
    let sidebar_base_classes = {
        let mut classes = vec![
            "fixed inset-y-0 z-10 hidden h-svh w-[var(--sidebar-width)] transition-[left,right,width] duration-200 ease-linear md:flex".to_string()
        ];

        match side {
            SidebarSide::Left => {
                classes.push("left-0 group-data-[collapsible=offcanvas]:left-[calc(var(--sidebar-width)*-1)]".to_string());
            }
            SidebarSide::Right => {
                classes.push("right-0 group-data-[collapsible=offcanvas]:right-[calc(var(--sidebar-width)*-1)]".to_string());
            }
        }

        match variant {
            SidebarVariant::Floating | SidebarVariant::Inset => {
                classes.push("p-2 group-data-[collapsible=icon]:w-[calc(var(--sidebar-width-icon)+theme(spacing.4)+2px)]".to_string());
            }
            _ => {
                classes.push("group-data-[collapsible=icon]:w-[var(--sidebar-width-icon)] group-data-[side=left]:border-r group-data-[side=right]:border-l".to_string());
            }
        }

        if !class.is_empty() {
            classes.push(class);
        }

        classes.join(" ")
    };

    // Inner content classes
    let content_classes = "flex h-full w-full flex-col bg-sidebar group-data-[variant=floating]:rounded-lg group-data-[variant=floating]:border group-data-[variant=floating]:border-sidebar-border group-data-[variant=floating]:shadow";

    view! {
        <div
            class=outer_classes
            data-state=state
            data-collapsible=collapsible_value
            data-variant=variant_value
            data-side=side_value
        >
            // Gap div
            <div class=gap_classes></div>

            // Main sidebar container
            <div class=sidebar_base_classes>
                <div data-sidebar="sidebar" class=content_classes>
                    {children()}
                </div>
            </div>
        </div>
    }
    .into_any()
}

/// A rail can be used to toggle the sidebar
#[component]
pub fn SidebarRail(#[prop(optional, into)] class: String) -> impl IntoView {
    let context = use_context::<SidebarContext>().expect("SidebarProvider must be an ancestor");

    let classes = {
        let mut classes =
            "absolute top-0 right-0 h-full w-1 bg-sidebar-border cursor-ew-resize".to_string();
        if !class.is_empty() {
            classes.push(' ');
            classes.push_str(&class);
        }
        classes
    };

    view! { <div class=classes on:click=move |_| context.set_open.update(|open| *open = !*open)></div> }
}

/// Badge component for menu items
#[component]
pub fn SidebarMenuBadge(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let classes = {
        let mut classes = "ml-auto flex h-5 items-center justify-center rounded-full bg-sidebar-accent px-2 text-xs font-medium text-sidebar-accent-foreground".to_string();
        if !class.is_empty() {
            classes.push(' ');
            classes.push_str(&class);
        }
        classes
    };

    view! { <span class=classes>{children()}</span> }
}

/// The header component of the sidebar, shown at the top
#[component]
pub fn SidebarHeader(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let classes = {
        let mut classes =
            "sticky top-0 h-[60px] flex items-center border-b border-sidebar-border px-4 py-2"
                .to_string();
        if !class.is_empty() {
            classes.push(' ');
            classes.push_str(&class);
        }
        classes
    };

    view! { <div class=classes>{children()}</div> }
}

/// The content area of the sidebar, typically containing groups and navigation items
#[component]
pub fn SidebarContent(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let classes = {
        let mut classes = "flex-1 overflow-auto py-2".to_string();
        if !class.is_empty() {
            classes.push(' ');
            classes.push_str(&class);
        }
        classes
    };

    view! { <div class=classes>{children()}</div> }
}

/// A group within the sidebar content, used to organize menu items
#[component]
pub fn SidebarGroup(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let classes = {
        let mut classes = "px-3 py-2".to_string();
        if !class.is_empty() {
            classes.push(' ');
            classes.push_str(&class);
        }
        classes
    };

    view! { <div class=classes>{children()}</div> }
}

/// A label for a sidebar group
#[component]
pub fn SidebarGroupLabel(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let classes = {
        let mut classes = "text-xs font-medium text-sidebar-foreground/70 mb-2".to_string();
        if !class.is_empty() {
            classes.push(' ');
            classes.push_str(&class);
        }
        classes
    };

    view! { <h3 class=classes>{children()}</h3> }
}

/// Content container for sidebar group items
#[component]
pub fn SidebarGroupContent(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let classes = {
        let mut classes = "space-y-1".to_string();
        if !class.is_empty() {
            classes.push(' ');
            classes.push_str(&class);
        }
        classes
    };

    view! { <div class=classes>{children()}</div> }
}

/// A menu component within the sidebar
#[component]
pub fn SidebarMenu(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let classes = {
        let mut classes = "space-y-1".to_string();
        if !class.is_empty() {
            classes.push(' ');
            classes.push_str(&class);
        }
        classes
    };

    view! { <nav class=classes>{children()}</nav> }
}

/// A menu item within the sidebar menu
#[component]
pub fn SidebarMenuItem(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let classes = {
        let mut classes = "relative".to_string();
        if !class.is_empty() {
            classes.push(' ');
            classes.push_str(&class);
        }
        classes
    };

    view! { <div class=classes>{children()}</div> }
}

/// A button within a sidebar menu item
#[component]
pub fn SidebarMenuButton(
    #[prop(optional, into)] class: String,
    #[prop(optional)] is_active: bool,
    children: Children,
) -> impl IntoView {
    let classes = {
        let mut classes = "group flex items-center w-full rounded-md px-3 py-2 text-sm font-medium hover:bg-sidebar-accent hover:text-sidebar-accent-foreground cursor-pointer transition-colors peer/menu-button".to_string();

        if is_active {
            classes.push_str(" bg-sidebar-accent text-sidebar-accent-foreground");
        }

        if !class.is_empty() {
            classes.push(' ');
            classes.push_str(&class);
        }

        classes
    };

    view! {
        <button class=classes data-active=move || if is_active { "true" } else { "false" }>
            {children()}
        </button>
    }
}

/// The footer component of the sidebar, shown at the bottom
#[component]
pub fn SidebarFooter(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let classes = {
        let mut classes =
            "sticky bottom-0 h-[60px] flex items-center border-t border-sidebar-border px-4 py-2"
                .to_string();
        if !class.is_empty() {
            classes.push(' ');
            classes.push_str(&class);
        }
        classes
    };

    view! { <div class=classes>{children()}</div> }
}

/// A separator line within the sidebar
#[component]
pub fn SidebarSeparator(#[prop(optional, into)] class: String) -> impl IntoView {
    let classes = {
        let mut classes = "h-px bg-sidebar-border my-2".to_string();
        if !class.is_empty() {
            classes.push(' ');
            classes.push_str(&class);
        }
        classes
    };

    view! { <div class=classes></div> }
}

/// A button that toggles the sidebar visibility
#[component]
pub fn SidebarTrigger(#[prop(optional, into)] class: String) -> impl IntoView {
    let context = use_context::<SidebarContext>().expect("SidebarProvider must be an ancestor");

    let toggle_sidebar = move |_| {
        if context.is_mobile.get() {
            // For mobile views
            context.set_open.update(|open| *open = !*open);
        } else {
            // For desktop views
            context.set_open.update(|open| *open = !*open);
        }
    };

    let classes = {
        let mut classes = "inline-flex items-center justify-center rounded-md text-sm font-medium h-9 w-9 px-0 transition-colors hover:bg-sidebar-accent hover:text-sidebar-accent-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-sidebar-ring".to_string();
        if !class.is_empty() {
            classes.push(' ');
            classes.push_str(&class);
        }
        classes
    };

    view! {
        <button
            class=classes
            on:click=toggle_sidebar
            title="Toggle sidebar"
            aria-label="Toggle sidebar"
            data-state=move || if context.is_open() { "open" } else { "closed" }
        >
            <span class="sr-only">Toggle sidebar</span>
            <span aria-hidden="true">"≡"</span>
        </button>
    }
}

/// A component to provide content for an inset sidebar layout
#[component]
pub fn SidebarInset(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let classes = {
        let mut classes = "relative flex w-full flex-1 flex-col bg-background".to_string();
        classes.push_str(" md:peer-data-[variant=inset]:m-2 md:peer-data-[state=collapsed]:peer-data-[variant=inset]:ml-2 md:peer-data-[variant=inset]:ml-0 md:peer-data-[variant=inset]:rounded-xl md:peer-data-[variant=inset]:shadow");

        if !class.is_empty() {
            classes.push(' ');
            classes.push_str(&class);
        }

        classes
    };

    view! { <main class=classes>{children()}</main> }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn class_prop_accepts_str_and_string() {
        assert_class_prop!(
            SidebarProps,
            SidebarRailProps,
            SidebarMenuBadgeProps,
            SidebarHeaderProps,
            SidebarContentProps,
            SidebarGroupProps,
            SidebarGroupLabelProps,
            SidebarGroupContentProps,
            SidebarMenuProps,
            SidebarMenuItemProps,
            SidebarMenuButtonProps,
            SidebarFooterProps,
            SidebarSeparatorProps,
            SidebarTriggerProps,
            SidebarInsetProps,
        );
    }
}
