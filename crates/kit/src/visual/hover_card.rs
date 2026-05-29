use leptos::prelude::*;
use leptos_floating::{Align, Side};
use tailwind_fuse::*;

use crate::behavior::{
    HoverCardContent as HoverCardContentPrimitive, HoverCardRoot as HoverCardRootPrimitive,
    HoverCardTrigger as HoverCardTriggerPrimitive,
};

/// The main HoverCard component.
#[component]
pub fn HoverCard(
    #[prop(into, default = Signal::stored(700))] open_delay: Signal<i32>,
    #[prop(into, default = Signal::stored(300))] close_delay: Signal<i32>,
    children: Children,
) -> impl IntoView {
    view! {
        <HoverCardRootPrimitive open_delay=open_delay close_delay=close_delay>
            {children()}
        </HoverCardRootPrimitive>
    }
}

/// HoverCard trigger with Shadcn styling.
#[component]
pub fn HoverCardTrigger(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let classes = tw_merge!("inline-block", &class);

    view! { <HoverCardTriggerPrimitive class=classes>{children()}</HoverCardTriggerPrimitive> }
}

/// HoverCard content with Shadcn styling.
#[component]
pub fn HoverCardContent(
    #[prop(optional, into)] class: String,
    #[prop(into, default = Side::Bottom)] side: Side,
    #[prop(into, default = Align::Center)] align: Align,
    #[prop(into, default = 4.0)] side_offset: f64,
    children: ChildrenFn,
) -> impl IntoView {
    let classes = tw_merge!(
        "z-50 w-64 rounded-md border bg-popover p-4 text-popover-foreground shadow-md outline-none data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2",
        &class
    );

    view! {
        <HoverCardContentPrimitive class=classes side=side align=align side_offset=side_offset>
            {children()}
        </HoverCardContentPrimitive>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn class_prop_accepts_str_and_string() {
        assert_class_prop!(HoverCardTriggerProps, HoverCardContentProps,);
    }
}
