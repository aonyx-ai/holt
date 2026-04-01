use leptos::prelude::*;
use leptos_icons::Icon;
use tailwind_fuse::*;

use crate::behavior::{
    NavigationMenuContent as NavigationMenuContentPrimitive,
    NavigationMenuItemRoot as NavigationMenuItemRootPrimitive,
    NavigationMenuLink as NavigationMenuLinkPrimitive,
    NavigationMenuList as NavigationMenuListPrimitive,
    NavigationMenuRoot as NavigationMenuRootPrimitive,
    NavigationMenuTrigger as NavigationMenuTriggerPrimitive,
};

#[derive(TwClass)]
#[tw(class = "relative")]
struct NavigationMenuStyle {}

#[derive(TwClass)]
#[tw(class = "flex flex-1 list-none items-center justify-center gap-1")]
struct NavigationMenuListStyle {}

#[derive(TwClass)]
#[tw(class = "relative")]
struct NavigationMenuItemStyle {}

#[derive(TwClass)]
#[tw(
    class = "group inline-flex h-9 w-max items-center justify-center gap-1 rounded-md bg-background px-4 py-2 text-sm font-medium transition-colors hover:bg-accent hover:text-accent-foreground focus:bg-accent focus:text-accent-foreground focus:outline-none disabled:pointer-events-none disabled:opacity-50 data-[state=open]:bg-accent/50"
)]
struct NavigationMenuTriggerStyle {}

#[derive(TwClass)]
#[tw(
    class = "absolute left-0 top-full mt-1.5 w-max min-w-[12rem] rounded-md border bg-popover p-4 text-popover-foreground shadow-lg data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=open]:zoom-in-95 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95"
)]
struct NavigationMenuContentStyle {}

#[derive(TwClass)]
#[tw(
    class = "block select-none rounded-md p-3 text-sm leading-none no-underline outline-none transition-colors hover:bg-accent hover:text-accent-foreground focus:bg-accent focus:text-accent-foreground"
)]
struct NavigationMenuLinkStyle {}

/// Root navigation menu with Shadcn-style defaults.
#[component]
pub fn NavigationMenu(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let class = NavigationMenuStyle {}.with_class(&class);
    view! { <NavigationMenuRootPrimitive class=class>{children()}</NavigationMenuRootPrimitive> }
}

/// Styled horizontal list of navigation items.
#[component]
pub fn NavigationMenuList(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let class = NavigationMenuListStyle {}.with_class(&class);
    view! { <NavigationMenuListPrimitive class=class>{children()}</NavigationMenuListPrimitive> }
}

/// An individual navigation menu item.
#[component]
pub fn NavigationMenuItem(
    #[prop(into)] id: &'static str,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let class = NavigationMenuItemStyle {}.with_class(&class);
    view! {
        <NavigationMenuItemRootPrimitive id=id class=class>
            {children()}
        </NavigationMenuItemRootPrimitive>
    }
}

/// Trigger button that opens a dropdown panel for its parent item.
#[component]
pub fn NavigationMenuTrigger(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let class = NavigationMenuTriggerStyle {}.with_class(&class);
    view! {
        <NavigationMenuTriggerPrimitive class=class>
            {children()}
            <Icon
                icon=icondata::LuChevronDown
                attr:class="relative top-px ml-1 h-3 w-3 transition duration-200 group-data-[state=open]:rotate-180"
            />
        </NavigationMenuTriggerPrimitive>
    }
}

/// Content panel displayed when its parent trigger is activated.
#[component]
pub fn NavigationMenuContent(
    #[prop(optional, into)] class: String,
    children: ChildrenFn,
) -> impl IntoView {
    let class = NavigationMenuContentStyle {}.with_class(&class);
    view! { <NavigationMenuContentPrimitive class=class>{children()}</NavigationMenuContentPrimitive> }
}

/// A styled navigation link.
#[component]
pub fn NavigationMenuLink(
    #[prop(into)] href: String,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let class = NavigationMenuLinkStyle {}.with_class(&class);
    view! {
        <NavigationMenuLinkPrimitive href=href class=class>
            {children()}
        </NavigationMenuLinkPrimitive>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn class_prop_accepts_str_and_string() {
        assert_class_prop!(
            NavigationMenuProps,
            NavigationMenuListProps,
            NavigationMenuItemProps,
            NavigationMenuTriggerProps,
            NavigationMenuContentProps,
            NavigationMenuLinkProps,
        );
    }
}
