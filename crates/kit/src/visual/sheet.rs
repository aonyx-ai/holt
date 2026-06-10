use leptos::prelude::*;
use leptos_icons::Icon;
use tailwind_fuse::*;

use crate::behavior::{
    SheetClose as SheetClosePrimitive, SheetContent as SheetContentPrimitive,
    SheetRoot as SheetRootPrimitive, SheetTrigger as SheetTriggerPrimitive,
};

/// Which edge of the screen the sheet slides out from.
#[derive(Default)]
pub enum SheetSide {
    Top,
    Bottom,
    Left,
    #[default]
    Right,
}

/// The main Sheet component
#[component]
pub fn Sheet(#[prop(optional)] open: RwSignal<bool>, children: Children) -> impl IntoView {
    view! { <SheetRootPrimitive open=open>{children()}</SheetRootPrimitive> }
}

/// Sheet trigger with no default styling (user styles via class prop)
#[component]
pub fn SheetTrigger(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <SheetTriggerPrimitive class=class>{children()}</SheetTriggerPrimitive> }
}

/// Sheet content panel that slides in from the specified side
#[component]
pub fn SheetContent(
    #[prop(optional, into)] class: String,
    #[prop(optional)] side: SheetSide,
    children: ChildrenFn,
) -> impl IntoView {
    let overlay_classes = tw_merge!(
        "fixed inset-0 z-50 bg-black/80 data-[state=open]:animate-in data-[state=open]:fade-in-0"
    );

    let side_classes = match &side {
        SheetSide::Top => "inset-x-0 top-0 border-b",
        SheetSide::Bottom => "inset-x-0 bottom-0 border-t",
        SheetSide::Left => "inset-y-0 left-0 h-full w-3/4 border-r sm:max-w-sm",
        SheetSide::Right => "inset-y-0 right-0 h-full w-3/4 border-l sm:max-w-sm",
    };

    let animation_classes = match &side {
        SheetSide::Top => "data-[state=open]:animate-in data-[state=open]:slide-in-from-top",
        SheetSide::Bottom => "data-[state=open]:animate-in data-[state=open]:slide-in-from-bottom",
        SheetSide::Left => "data-[state=open]:animate-in data-[state=open]:slide-in-from-left",
        SheetSide::Right => "data-[state=open]:animate-in data-[state=open]:slide-in-from-right",
    };

    let content_classes = tw_merge!(
        "fixed z-50 gap-4 bg-background p-6 shadow-lg transition ease-in-out",
        side_classes,
        animation_classes,
        &class
    );

    let children = StoredValue::new(children);

    view! {
        <SheetContentPrimitive class=overlay_classes>
            <div class=content_classes
                .clone()>
                {children.read_value()()}
                <SheetClosePrimitive class="absolute right-4 top-4 rounded-sm opacity-70 ring-offset-background transition-opacity hover:opacity-100 focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:pointer-events-none">
                    <Icon icon=icondata::LuX attr:class="h-4 w-4" />
                    <span class="sr-only">"Close"</span>
                </SheetClosePrimitive>
            </div>
        </SheetContentPrimitive>
    }
}

/// Header section of the sheet
#[component]
pub fn SheetHeader(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let classes = tw_merge!("flex flex-col space-y-2 text-center sm:text-left", &class);
    view! { <div class=classes>{children()}</div> }
}

/// Footer section of the sheet
#[component]
pub fn SheetFooter(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let classes = tw_merge!(
        "flex flex-col-reverse sm:flex-row sm:justify-end sm:space-x-2",
        &class
    );
    view! { <div class=classes>{children()}</div> }
}

/// Title of the sheet
#[component]
pub fn SheetTitle(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let classes = tw_merge!("text-lg font-semibold text-foreground", &class);
    view! { <h3 class=classes>{children()}</h3> }
}

/// Description text in the sheet
#[component]
pub fn SheetDescription(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let classes = tw_merge!("text-sm text-muted-foreground", &class);
    view! { <p class=classes>{children()}</p> }
}

/// Close button (re-export of behavior primitive with optional styling)
#[component]
pub fn SheetClose(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <SheetClosePrimitive class=class>{children()}</SheetClosePrimitive> }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn class_prop_accepts_str_and_string() {
        assert_class_prop!(
            SheetContentProps,
            SheetHeaderProps,
            SheetFooterProps,
            SheetTitleProps,
            SheetDescriptionProps,
            SheetCloseProps,
            SheetTriggerProps,
        );
    }
}
