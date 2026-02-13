use leptos::prelude::*;
use leptos_icons::Icon;
use tailwind_fuse::*;

use crate::behavior::{
    SelectContent as SelectContentPrimitive, SelectItem as SelectItemPrimitive,
    SelectRoot as SelectRootPrimitive, SelectTrigger as SelectTriggerPrimitive,
    SelectValue as SelectValuePrimitive,
};
use crate::floating::{Align, Side};

/// The main Select component
#[component]
pub fn Select(
    #[prop(optional)] value: RwSignal<Option<String>>,
    #[prop(into, default = Signal::stored(false))] disabled: Signal<bool>,
    #[prop(optional_no_strip)] on_change: Option<Callback<Option<String>>>,
    children: Children,
) -> impl IntoView {
    view! {
        <SelectRootPrimitive value=value disabled=disabled on_change=on_change>
            {children()}
        </SelectRootPrimitive>
    }
}

/// Select trigger with Shadcn styling
#[component]
pub fn SelectTrigger(
    #[prop(optional)] class: &'static str,
    #[prop(optional_no_strip, into)] id: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    let classes = tw_merge!(
        "flex h-10 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 [&>span]:line-clamp-1",
        class
    );

    view! {
        <SelectTriggerPrimitive class=classes id=id>
            {children()}
            <Icon icon=icondata::LuChevronDown attr:class="h-4 w-4 opacity-50" />
        </SelectTriggerPrimitive>
    }
}

/// Select content with positioning and styling
#[component]
pub fn SelectContent(
    #[prop(optional)] class: &'static str,
    #[prop(into, default = Side::Bottom)] side: Side,
    #[prop(into, default = Align::Start)] align: Align,
    #[prop(into, default = 4.0)] side_offset: f64,
    children: ChildrenFn,
) -> impl IntoView {
    let classes = tw_merge!(
        "z-50 max-h-96 min-w-[8rem] overflow-hidden rounded-md border bg-popover text-popover-foreground shadow-md data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2",
        class
    );

    view! {
        <SelectContentPrimitive class=classes side=side align=align side_offset=side_offset>
            <div class="p-1">{children()}</div>
        </SelectContentPrimitive>
    }
}

/// Select item with hover and selection states
#[component]
pub fn SelectItem(
    #[prop(into)] value: String,
    #[prop(optional)] class: &'static str,
    #[prop(into, default = Signal::stored(false))] disabled: Signal<bool>,
    children: Children,
) -> impl IntoView {
    let classes = tw_merge!(
        "relative flex w-full cursor-default select-none items-center rounded-sm py-1.5 pl-2 pr-8 text-sm outline-none focus:bg-accent focus:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50",
        class
    );

    view! {
        <SelectItemPrimitive value=value.clone() class=classes disabled=disabled>
            <span class="absolute right-2 flex h-3.5 w-3.5 items-center justify-center">
                <Show when=move || {
                    let context = crate::behavior::use_select();
                    context.get_value().is_some_and(|v| v == value)
                }>
                    <Icon icon=icondata::LuCheck attr:class="h-4 w-4" />
                </Show>
            </span>
            {children()}
        </SelectItemPrimitive>
    }
}

/// Component to display selected value or placeholder
#[component]
pub fn SelectValue(
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(optional)] class: &'static str,
) -> impl IntoView {
    let classes = tw_merge!("", class);

    view! { <SelectValuePrimitive placeholder=placeholder class=classes /> }
}

/// Label for select groups
#[component]
pub fn SelectLabel(#[prop(optional)] class: &'static str, children: Children) -> impl IntoView {
    let classes = tw_merge!("py-1.5 pl-8 pr-2 text-sm font-semibold", class);

    view! { <div class=classes>{children()}</div> }
}

/// Separator between select items
#[component]
pub fn SelectSeparator(#[prop(optional)] class: &'static str) -> impl IntoView {
    let classes = tw_merge!("-mx-1 my-1 h-px bg-muted", class);

    view! { <div class=classes></div> }
}
