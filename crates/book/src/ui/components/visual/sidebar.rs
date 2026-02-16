use leptos::children::Children;
use leptos::prelude::*;

use super::super::behavior::SidebarContext;

#[derive(Clone, Copy, PartialEq)]
pub enum SidebarVariant {
    Sidebar,
    Floating,
    Inset,
}

#[derive(Clone, Copy)]
pub enum SidebarSide {
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq)]
pub enum SidebarCollapsible {
    OffCanvas,
    Icon,
    None,
}

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

    let is_mobile = context.is_mobile.get();
    if is_mobile {
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

    let outer_classes = "group peer hidden text-sidebar-foreground md:block";

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

    let content_classes = "flex h-full w-full flex-col bg-sidebar group-data-[variant=floating]:rounded-lg group-data-[variant=floating]:border group-data-[variant=floating]:border-sidebar-border group-data-[variant=floating]:shadow";

    view! {
        <div
            class=outer_classes
            data-state=state
            data-collapsible=collapsible_value
            data-variant=variant_value
            data-side=side_value
        >
            <div class=gap_classes></div>
            <div class=sidebar_base_classes>
                <div data-sidebar="sidebar" class=content_classes>
                    {children()}
                </div>
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
