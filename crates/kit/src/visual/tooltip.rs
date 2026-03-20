use leptos::prelude::*;
use leptos_floating::{Align, Side};
use tailwind_fuse::*;

use crate::behavior::{
    TooltipContent as TooltipContentPrimitive, TooltipRoot as TooltipRootPrimitive,
    TooltipTrigger as TooltipTriggerPrimitive,
};

/// Root tooltip that provides context
#[component]
pub fn Tooltip(
    #[prop(into, default = 700u64)] open_delay_ms: u64,
    #[prop(into, default = 300u64)] close_delay_ms: u64,
    children: Children,
) -> impl IntoView {
    view! {
        <TooltipRootPrimitive open_delay_ms=open_delay_ms close_delay_ms=close_delay_ms>
            {children()}
        </TooltipRootPrimitive>
    }
}

/// Trigger that shows the tooltip on hover/focus
#[component]
pub fn TooltipTrigger(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <TooltipTriggerPrimitive class=class>{children()}</TooltipTriggerPrimitive> }
}

#[derive(TwClass)]
#[tw(
    class = "z-50 overflow-hidden rounded-md bg-primary px-3 py-1.5 text-xs text-primary-foreground animate-in fade-in-0 zoom-in-95"
)]
struct TooltipContentStyle {}

/// Styled tooltip content with floating positioning
#[component]
pub fn TooltipContent(
    #[prop(optional, into)] class: String,
    #[prop(into, default = Side::Top)] side: Side,
    #[prop(into, default = Align::Center)] align: Align,
    #[prop(into, default = 4.0)] side_offset: f64,
    children: ChildrenFn,
) -> impl IntoView {
    let class = TooltipContentStyle {}.with_class(&class);
    view! {
        <TooltipContentPrimitive class=class side=side align=align side_offset=side_offset>
            {children()}
        </TooltipContentPrimitive>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn class_prop_accepts_str_and_string() {
        assert_class_prop!(TooltipTriggerProps, TooltipContentProps,);
    }
}
