use leptos::prelude::*;
use tailwind_fuse::*;

use crate::behavior::{
    DropdownMenuContent as DropdownMenuContentPrimitive,
    DropdownMenuItem as DropdownMenuItemPrimitive, DropdownMenuLabel as DropdownMenuLabelPrimitive,
    DropdownMenuRoot as DropdownMenuRootPrimitive,
    DropdownMenuSeparator as DropdownMenuSeparatorPrimitive,
    DropdownMenuTrigger as DropdownMenuTriggerPrimitive,
};
use leptos_floating::{Align, Side};

/// The main DropdownMenu component
#[component]
pub fn DropdownMenu(children: Children) -> impl IntoView {
    view! {
        <DropdownMenuRootPrimitive>{children()}</DropdownMenuRootPrimitive>
    }
}

/// Dropdown menu trigger with styling
#[component]
pub fn DropdownMenuTrigger(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let classes = tw_merge!(
        "inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50",
        &class
    );

    view! {
        <DropdownMenuTriggerPrimitive class=classes>
            {children()}
        </DropdownMenuTriggerPrimitive>
    }
}

/// Dropdown menu content with positioning and styling
#[component]
pub fn DropdownMenuContent(
    #[prop(optional, into)] class: String,
    #[prop(into, default = Side::Bottom)] side: Side,
    #[prop(into, default = Align::Start)] align: Align,
    #[prop(into, default = 4.0)] side_offset: f64,
    children: ChildrenFn,
) -> impl IntoView {
    let classes = tw_merge!(
        "z-50 min-w-[8rem] overflow-hidden rounded-md border bg-popover p-1 text-popover-foreground shadow-md data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2",
        &class
    );

    view! {
        <DropdownMenuContentPrimitive class=classes side=side align=align side_offset=side_offset>
            {children()}
        </DropdownMenuContentPrimitive>
    }
}

/// Dropdown menu item with hover and focus states
#[component]
pub fn DropdownMenuItem(
    #[prop(optional, into)] class: String,
    #[prop(into, default = Signal::stored(false))] disabled: Signal<bool>,
    #[prop(optional_no_strip)] on_select: Option<Callback<()>>,
    children: Children,
) -> impl IntoView {
    let classes = tw_merge!(
        "relative flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none transition-colors focus:bg-accent focus:text-accent-foreground data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50",
        &class
    );

    view! {
        <DropdownMenuItemPrimitive class=classes disabled=disabled on_select=on_select>
            {children()}
        </DropdownMenuItemPrimitive>
    }
}

/// Label for grouping menu items
#[component]
pub fn DropdownMenuLabel(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let classes = tw_merge!("px-2 py-1.5 text-sm font-semibold", &class);

    view! {
        <DropdownMenuLabelPrimitive class=classes>
            {children()}
        </DropdownMenuLabelPrimitive>
    }
}

/// Separator between menu items
#[component]
pub fn DropdownMenuSeparator(#[prop(optional, into)] class: String) -> impl IntoView {
    let classes = tw_merge!("-mx-1 my-1 h-px bg-muted", &class);

    view! { <DropdownMenuSeparatorPrimitive class=classes /> }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn class_prop_accepts_str_and_string() {
        assert_class_prop!(
            DropdownMenuTriggerProps,
            DropdownMenuContentProps,
            DropdownMenuItemProps,
            DropdownMenuLabelProps,
            DropdownMenuSeparatorProps,
        );
    }
}
