use leptos::prelude::*;
use tailwind_fuse::*;

use crate::behavior::{
    ContextMenuContent as ContextMenuContentPrimitive, ContextMenuItem as ContextMenuItemPrimitive,
    ContextMenuRoot as ContextMenuRootPrimitive,
    ContextMenuSeparator as ContextMenuSeparatorPrimitive,
    ContextMenuTrigger as ContextMenuTriggerPrimitive,
};

/// The main ContextMenu component
#[component]
pub fn ContextMenu(children: Children) -> impl IntoView {
    view! { <ContextMenuRootPrimitive>{children()}</ContextMenuRootPrimitive> }
}

/// Context menu trigger with optional styling
#[component]
pub fn ContextMenuTrigger(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let classes = tw_merge!("", &class);

    view! { <ContextMenuTriggerPrimitive class=classes>{children()}</ContextMenuTriggerPrimitive> }
}

/// Context menu content with Shadcn styling
#[component]
pub fn ContextMenuContent(
    #[prop(optional, into)] class: String,
    children: ChildrenFn,
) -> impl IntoView {
    let classes = tw_merge!(
        "z-50 min-w-[8rem] overflow-hidden rounded-md border bg-popover p-1 text-popover-foreground shadow-md animate-in fade-in-0 zoom-in-95",
        &class
    );

    view! { <ContextMenuContentPrimitive class=classes>{children()}</ContextMenuContentPrimitive> }
}

/// Context menu item with hover and focus states
#[component]
pub fn ContextMenuItem(
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
        <ContextMenuItemPrimitive class=classes disabled=disabled on_select=on_select>
            {children()}
        </ContextMenuItemPrimitive>
    }
}

/// Separator between context menu items
#[component]
pub fn ContextMenuSeparator(#[prop(optional, into)] class: String) -> impl IntoView {
    let classes = tw_merge!("-mx-1 my-1 h-px bg-muted", &class);

    view! { <ContextMenuSeparatorPrimitive class=classes /> }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn class_prop_accepts_str_and_string() {
        assert_class_prop!(
            ContextMenuTriggerProps,
            ContextMenuContentProps,
            ContextMenuItemProps,
            ContextMenuSeparatorProps,
        );
    }
}
