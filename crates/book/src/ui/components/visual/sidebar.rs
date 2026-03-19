use leptos::children::Children;
use leptos::prelude::*;

use super::super::behavior::SidebarContext;

#[derive(Copy, Clone, PartialEq)]
#[allow(dead_code)]
pub enum SidebarVariant {
    Sidebar,
    Floating,
    Inset,
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum SidebarSide {
    Left,
    Right,
}

#[derive(Copy, Clone, PartialEq)]
#[allow(dead_code)]
pub enum SidebarCollapsible {
    OffCanvas,
    Icon,
    None,
}

/// Responsive sidebar that works with both CSR and SSR/hydration.
///
/// Renders a single DOM structure with responsive CSS classes so SSR output
/// contains everything needed for both mobile and desktop viewports.
///
/// - **Mobile (<md):** fixed overlay with translate, backdrop, shadow
/// - **Desktop (>=md):** static in-flow sidebar with collapsible width
#[component]
pub fn Sidebar(
    #[prop(optional, into)] class: String,
    #[prop(optional, default = SidebarSide::Left)] side: SidebarSide,
    #[prop(optional, default = SidebarVariant::Sidebar)] variant: SidebarVariant,
    #[prop(optional, default = SidebarCollapsible::OffCanvas)] collapsible: SidebarCollapsible,
    children: Children,
) -> impl IntoView {
    let context = use_context::<SidebarContext>().expect("SidebarProvider must be an ancestor");

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

    let state = move || {
        if context.is_open() {
            "expanded"
        } else {
            "collapsed"
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

    // Overlay backdrop — only visible on mobile when sidebar is open
    let overlay_classes = move || {
        let base =
            "fixed inset-0 z-40 bg-black/50 md:hidden transition-opacity duration-200 ease-in-out";
        if context.is_open() {
            base.to_string()
        } else {
            format!("{base} opacity-0 pointer-events-none")
        }
    };

    // Single sidebar element with responsive classes:
    // Mobile: fixed overlay with translate-x to show/hide
    // Desktop: static in-flow with width transition for collapse
    let sidebar_classes = {
        let class_extra = class;
        move || {
            let mut c = String::with_capacity(512);

            // Base
            c.push_str("peer flex h-svh flex-col bg-sidebar text-sidebar-foreground");

            // Mobile: fixed overlay
            c.push_str(" fixed inset-y-0 z-50 w-[var(--sidebar-width-mobile)] shadow-xl");
            c.push_str(" transition-transform duration-200 ease-in-out");

            // Mobile side
            match side {
                SidebarSide::Left => c.push_str(" left-0"),
                SidebarSide::Right => c.push_str(" right-0"),
            }

            // Mobile: hidden when closed
            if !context.is_open() {
                match side {
                    SidebarSide::Left => c.push_str(" -translate-x-full"),
                    SidebarSide::Right => c.push_str(" translate-x-full"),
                }
            }

            // Desktop: static in-flow, reset fixed positioning
            c.push_str(" md:static md:z-auto md:translate-x-0 md:shadow-none");
            c.push_str(" md:w-[var(--sidebar-width)] md:transition-[width] md:duration-200");

            // Desktop: border
            match (variant, side) {
                (SidebarVariant::Floating | SidebarVariant::Inset, _) => {}
                (_, SidebarSide::Left) => c.push_str(" md:border-r"),
                (_, SidebarSide::Right) => c.push_str(" md:border-l"),
            }

            // Desktop: collapsed state
            if !context.is_open() {
                match collapsible {
                    SidebarCollapsible::OffCanvas => {
                        c.push_str(" md:w-0 md:overflow-hidden md:border-0");
                    }
                    SidebarCollapsible::Icon => {
                        c.push_str(" md:w-[var(--sidebar-width-icon)]");
                    }
                    SidebarCollapsible::None => {}
                }
            }

            if !class_extra.is_empty() {
                c.push(' ');
                c.push_str(&class_extra);
            }

            c
        }
    };

    view! {
        <div data-state=state data-variant=variant_value data-side=side_value>
            <div class=overlay_classes on:click=move |_| context.toggle()></div>
            <div data-sidebar="sidebar" class=sidebar_classes>
                <div class="flex h-full w-full flex-col">{children()}</div>
            </div>
        </div>
    }
    .into_any()
}

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

#[component]
pub fn SidebarTrigger(#[prop(optional, into)] class: String) -> impl IntoView {
    let context = use_context::<SidebarContext>().expect("SidebarProvider must be an ancestor");

    let toggle_sidebar = move |_| {
        context.set_open.update(|open| *open = !*open);
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

#[component]
pub fn SidebarInset(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let classes = {
        let mut classes =
            "relative flex w-full flex-1 flex-col bg-background overflow-auto".to_string();
        classes.push_str(" md:peer-data-[variant=inset]:m-2 md:peer-data-[state=collapsed]:peer-data-[variant=inset]:ml-2 md:peer-data-[variant=inset]:ml-0 md:peer-data-[variant=inset]:rounded-xl md:peer-data-[variant=inset]:shadow");

        if !class.is_empty() {
            classes.push(' ');
            classes.push_str(&class);
        }

        classes
    };

    view! { <main class=classes>{children()}</main> }
}
