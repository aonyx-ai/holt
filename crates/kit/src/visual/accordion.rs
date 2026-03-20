use leptos::prelude::*;
use tailwind_fuse::*;

use crate::behavior::{
    AccordionContent as AccordionContentPrimitive, AccordionItem as AccordionItemPrimitive,
    AccordionRoot as AccordionRootPrimitive, AccordionTrigger as AccordionTriggerPrimitive,
    AccordionType,
};

#[derive(TwClass)]
#[tw(class = "w-full")]
struct AccordionStyle {}

#[derive(TwClass)]
#[tw(class = "border-b")]
struct AccordionItemStyle {}

#[derive(TwClass)]
#[tw(
    class = "flex flex-1 items-center justify-between py-4 text-sm font-medium transition-all hover:underline [&[data-state=open]>svg]:rotate-180"
)]
struct AccordionTriggerStyle {}

#[derive(TwClass)]
#[tw(
    class = "overflow-hidden text-sm data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=closed]:animate-out data-[state=closed]:fade-out-0"
)]
struct AccordionContentStyle {}

#[derive(TwClass)]
#[tw(class = "pb-4 pt-0")]
struct AccordionContentInnerStyle {}

#[component]
pub fn Accordion(
    #[prop(optional, into)] class: String,
    #[prop(optional)] accordion_type: AccordionType,
    #[prop(optional, into)] default_value: Vec<String>,
    children: Children,
) -> impl IntoView {
    let class = AccordionStyle {}.with_class(&class);
    view! {
        <AccordionRootPrimitive class=class accordion_type=accordion_type default_value=default_value>
            {children()}
        </AccordionRootPrimitive>
    }
}

#[component]
pub fn AccordionItem(
    #[prop(optional, into)] class: String,
    #[prop(into)] value: String,
    children: Children,
) -> impl IntoView {
    let class = AccordionItemStyle {}.with_class(&class);
    view! {
        <AccordionItemPrimitive class=class value=value>
            {children()}
        </AccordionItemPrimitive>
    }
}

#[component]
pub fn AccordionTrigger(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] disabled: Signal<bool>,
    children: Children,
) -> impl IntoView {
    let class = AccordionTriggerStyle {}.with_class(&class);
    view! {
        <AccordionTriggerPrimitive class=class disabled=disabled>
            {children()}
        </AccordionTriggerPrimitive>
    }
}

#[component]
pub fn AccordionContent(
    #[prop(optional, into)] class: String,
    children: ChildrenFn,
) -> impl IntoView {
    let class = AccordionContentStyle {}.with_class(&class);
    view! {
        <AccordionContentPrimitive class=class>
            <div class=AccordionContentInnerStyle {}.to_class()>
                {children()}
            </div>
        </AccordionContentPrimitive>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn class_prop_accepts_str_and_string() {
        assert_class_prop!(
            AccordionProps,
            AccordionItemProps,
            AccordionTriggerProps,
            AccordionContentProps,
        );
    }
}
