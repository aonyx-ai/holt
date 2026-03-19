use leptos::prelude::*;
use tailwind_fuse::*;

use crate::behavior::{
    TabsContent as TabsContentPrimitive, TabsList as TabsListPrimitive,
    TabsRoot as TabsRootPrimitive, TabsTrigger as TabsTriggerPrimitive,
};

#[derive(TwClass)]
#[tw(class = "")]
struct TabsStyle {}

#[derive(TwClass)]
#[tw(
    class = "inline-flex h-9 items-center justify-center rounded-lg bg-muted p-1 text-muted-foreground"
)]
struct TabsListStyle {}

#[derive(TwClass)]
#[tw(
    class = "inline-flex items-center justify-center whitespace-nowrap rounded-md px-3 py-1 text-sm font-medium ring-offset-background transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 data-[state=active]:bg-background data-[state=active]:text-foreground data-[state=active]:shadow"
)]
struct TabsTriggerStyle {}

#[derive(TwClass)]
#[tw(
    class = "mt-2 ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
)]
struct TabsContentStyle {}

#[component]
pub fn Tabs(
    #[prop(optional, into)] class: String,
    #[prop(into)] default_value: String,
    children: Children,
) -> impl IntoView {
    let class = TabsStyle {}.with_class(&class);
    view! {
        <TabsRootPrimitive class=class default_value=default_value>
            {children()}
        </TabsRootPrimitive>
    }
}

#[component]
pub fn TabsList(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let class = TabsListStyle {}.with_class(&class);
    view! {
        <TabsListPrimitive class=class>{children()}</TabsListPrimitive>
    }
}

#[component]
pub fn TabsTrigger(
    #[prop(optional, into)] class: String,
    #[prop(into)] value: String,
    #[prop(optional, into)] disabled: Signal<bool>,
    children: Children,
) -> impl IntoView {
    let class = TabsTriggerStyle {}.with_class(&class);
    view! {
        <TabsTriggerPrimitive class=class value=value disabled=disabled>
            {children()}
        </TabsTriggerPrimitive>
    }
}

#[component]
pub fn TabsContent(
    #[prop(optional, into)] class: String,
    #[prop(into)] value: String,
    children: ChildrenFn,
) -> impl IntoView {
    let class = TabsContentStyle {}.with_class(&class);
    view! { <TabsContentPrimitive class=class value=value>{children()}</TabsContentPrimitive> }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn class_prop_accepts_str_and_string() {
        assert_class_prop!(TabsProps, TabsListProps, TabsTriggerProps, TabsContentProps,);
    }
}
