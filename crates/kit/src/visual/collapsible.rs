use leptos::prelude::*;
use tailwind_fuse::*;

use crate::behavior::{
    CollapsibleContent as CollapsibleContentPrimitive, CollapsibleRoot as CollapsibleRootPrimitive,
    CollapsibleTrigger as CollapsibleTriggerPrimitive,
};

#[derive(TwClass)]
#[tw(class = "space-y-2")]
struct CollapsibleStyle {}

#[derive(TwClass)]
#[tw(
    class = "flex w-full items-center justify-between rounded-md px-3 py-2 text-sm font-medium transition-colors hover:bg-accent hover:text-accent-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:opacity-50 disabled:pointer-events-none"
)]
struct CollapsibleTriggerStyle {}

#[derive(TwClass)]
#[tw(
    class = "overflow-hidden text-sm data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=open]:slide-in-from-top-1 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:slide-out-to-top-1"
)]
struct CollapsibleContentStyle {}

#[component]
pub fn Collapsible(
    #[prop(optional, into)] class: String,
    open: RwSignal<bool>,
    children: Children,
) -> impl IntoView {
    let class = CollapsibleStyle {}.with_class(&class);
    view! {
        <CollapsibleRootPrimitive class=class open=open>
            {children()}
        </CollapsibleRootPrimitive>
    }
}

#[component]
pub fn CollapsibleTrigger(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] disabled: Signal<bool>,
    children: Children,
) -> impl IntoView {
    let class = CollapsibleTriggerStyle {}.with_class(&class);
    view! {
        <CollapsibleTriggerPrimitive class=class disabled=disabled>
            {children()}
        </CollapsibleTriggerPrimitive>
    }
}

#[component]
pub fn CollapsibleContent(
    #[prop(optional, into)] class: String,
    children: ChildrenFn,
) -> impl IntoView {
    let class = CollapsibleContentStyle {}.with_class(&class);
    view! { <CollapsibleContentPrimitive class=class>{children()}</CollapsibleContentPrimitive> }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn class_prop_accepts_str_and_string() {
        assert_class_prop!(
            CollapsibleProps,
            CollapsibleTriggerProps,
            CollapsibleContentProps,
        );
    }
}
