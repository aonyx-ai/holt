use leptos::prelude::*;
use leptos_floating::{Align, Side};
use tailwind_fuse::*;

use crate::behavior::{
    MenubarContent as MenubarContentPrimitive, MenubarItem as MenubarItemPrimitive,
    MenubarMenu as MenubarMenuPrimitive, MenubarRoot as MenubarRootPrimitive,
    MenubarSeparator as MenubarSeparatorPrimitive, MenubarTrigger as MenubarTriggerPrimitive,
};

/// Horizontal menu bar container.
#[component]
pub fn Menubar(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let classes = tw_merge!(
        "flex h-10 items-center space-x-1 rounded-md border bg-background p-1",
        &class
    );

    view! { <MenubarRootPrimitive class=classes>{children()}</MenubarRootPrimitive> }
}

/// A single menu within the menubar.
#[component]
pub fn MenubarMenu(#[prop(into)] value: String, children: Children) -> impl IntoView {
    view! { <MenubarMenuPrimitive value=value>{children()}</MenubarMenuPrimitive> }
}

/// Trigger button that opens a menu dropdown.
#[component]
pub fn MenubarTrigger(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let classes = tw_merge!(
        "flex cursor-default select-none items-center rounded-sm px-3 py-1.5 text-sm font-medium outline-none focus:bg-accent focus:text-accent-foreground data-[state=open]:bg-accent data-[state=open]:text-accent-foreground",
        &class
    );

    view! { <MenubarTriggerPrimitive class=classes>{children()}</MenubarTriggerPrimitive> }
}

/// Dropdown content for a menu, positioned below the trigger.
#[component]
pub fn MenubarContent(
    #[prop(optional, into)] class: String,
    #[prop(into, default = Side::Bottom)] side: Side,
    #[prop(into, default = Align::Start)] align: Align,
    #[prop(into, default = 4.0)] side_offset: f64,
    children: ChildrenFn,
) -> impl IntoView {
    let classes = tw_merge!(
        "z-50 min-w-[12rem] overflow-hidden rounded-md border bg-popover p-1 text-popover-foreground shadow-md data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2",
        &class
    );

    view! {
        <MenubarContentPrimitive class=classes side=side align=align side_offset=side_offset>
            {children()}
        </MenubarContentPrimitive>
    }
}

/// A clickable menu item.
#[component]
pub fn MenubarItem(
    #[prop(optional, into)] class: String,
    #[prop(into, default = Signal::stored(false))] disabled: Signal<bool>,
    #[prop(optional_no_strip)] on_select: Option<Callback<()>>,
    children: Children,
) -> impl IntoView {
    let classes = tw_merge!(
        "relative flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none focus:bg-accent focus:text-accent-foreground data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50",
        &class
    );

    view! {
        <MenubarItemPrimitive class=classes disabled=disabled on_select=on_select>
            {children()}
        </MenubarItemPrimitive>
    }
}

/// Separator line between menu items.
#[component]
pub fn MenubarSeparator(#[prop(optional, into)] class: String) -> impl IntoView {
    let classes = tw_merge!("-mx-1 my-1 h-px bg-muted", &class);

    view! { <MenubarSeparatorPrimitive class=classes /> }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn class_prop_accepts_str_and_string() {
        assert_class_prop!(
            MenubarProps,
            MenubarTriggerProps,
            MenubarContentProps,
            MenubarItemProps,
            MenubarSeparatorProps,
        );
    }
}
