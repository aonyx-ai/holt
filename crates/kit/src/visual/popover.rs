use leptos::prelude::*;
use tailwind_fuse::*;

use crate::behavior::{
    PopoverContent as PopoverContentPrimitive, PopoverRoot as PopoverRootPrimitive,
    PopoverTrigger as PopoverTriggerPrimitive,
};
use leptos_floating::{Align, Side};

/// The main Popover component
#[component]
pub fn Popover(#[prop(optional)] open: RwSignal<bool>, children: Children) -> impl IntoView {
    view! {
        <PopoverRootPrimitive open=open>
            {children()}
        </PopoverRootPrimitive>
    }
}

/// Popover trigger with optional styling
#[component]
pub fn PopoverTrigger(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let classes = tw_merge!("inline-flex items-center justify-center", &class);

    view! {
        <PopoverTriggerPrimitive class=classes>
            {children()}
        </PopoverTriggerPrimitive>
    }
}

/// Popover content with Shadcn styling
#[component]
pub fn PopoverContent(
    #[prop(optional, into)] class: String,
    #[prop(into, default = Side::Bottom)] side: Side,
    #[prop(into, default = Align::Center)] align: Align,
    #[prop(into, default = 4.0)] side_offset: f64,
    children: ChildrenFn,
) -> impl IntoView {
    let classes = tw_merge!(
        "z-50 w-72 rounded-md border bg-popover p-4 text-popover-foreground shadow-md outline-none data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2",
        &class
    );

    view! {
        <PopoverContentPrimitive class=classes side=side align=align side_offset=side_offset>
            {children()}
        </PopoverContentPrimitive>
    }
}
