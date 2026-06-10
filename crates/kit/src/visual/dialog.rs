use leptos::prelude::*;
use leptos_icons::Icon;
use tailwind_fuse::*;

use crate::behavior::{
    DialogClose as DialogClosePrimitive, DialogContent as DialogContentPrimitive,
    DialogRoot as DialogRootPrimitive, DialogTrigger as DialogTriggerPrimitive,
};

#[derive(TwClass)]
#[tw(class = "fixed inset-0 z-50 bg-black/80")]
struct DialogOverlayStyle {}

#[derive(TwClass)]
#[tw(
    class = "fixed left-[50%] top-[50%] z-50 grid w-full max-w-lg translate-x-[-50%] translate-y-[-50%] gap-4 border bg-background p-6 shadow-lg sm:rounded-lg"
)]
struct DialogContentStyle {}

#[derive(TwClass)]
#[tw(
    class = "absolute right-4 top-4 rounded-sm opacity-70 ring-offset-background transition-opacity hover:opacity-100 focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:pointer-events-none"
)]
struct DialogCloseStyle {}

/// The main Dialog component
#[component]
pub fn Dialog(#[prop(optional)] open: RwSignal<bool>, children: Children) -> impl IntoView {
    view! {
        <DialogRootPrimitive open=open>
            {children()}
        </DialogRootPrimitive>
    }
}

/// Dialog trigger with optional styling
#[component]
pub fn DialogTrigger(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! {
        <DialogTriggerPrimitive class=class>
            {children()}
        </DialogTriggerPrimitive>
    }
}

/// Dialog content with overlay backdrop and centered positioning
#[component]
pub fn DialogContent(#[prop(optional, into)] class: String, children: ChildrenFn) -> impl IntoView {
    let overlay_class = DialogOverlayStyle {}.to_class();
    let content_class = DialogContentStyle {}.with_class(&class);

    view! {
        <DialogContentPrimitive class=content_class>
            {children()}
            <DialogClosePrimitive class=DialogCloseStyle {}.to_class()>
                <Icon icon=icondata::LuX attr:class="h-4 w-4" />
                <span class="sr-only">"Close"</span>
            </DialogClosePrimitive>
        </DialogContentPrimitive>
    }
}

/// Header section for dialog
#[component]
pub fn DialogHeader(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let classes = tw_merge!("flex flex-col space-y-1.5 text-center sm:text-left", &class);

    view! { <div class=classes>{children()}</div> }
}

/// Footer section for dialog
#[component]
pub fn DialogFooter(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let classes = tw_merge!(
        "flex flex-col-reverse sm:flex-row sm:justify-end sm:space-x-2",
        &class
    );

    view! { <div class=classes>{children()}</div> }
}

/// Dialog title
#[component]
pub fn DialogTitle(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let classes = tw_merge!("text-lg font-semibold leading-none tracking-tight", &class);

    view! { <h2 class=classes>{children()}</h2> }
}

/// Dialog description
#[component]
pub fn DialogDescription(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let classes = tw_merge!("text-sm text-muted-foreground", &class);

    view! { <p class=classes>{children()}</p> }
}

/// Close button for dialog
#[component]
pub fn DialogClose(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! {
        <DialogClosePrimitive class=class>
            {children()}
        </DialogClosePrimitive>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn class_prop_accepts_str_and_string() {
        assert_class_prop!(
            DialogTriggerProps,
            DialogContentProps,
            DialogHeaderProps,
            DialogFooterProps,
            DialogTitleProps,
            DialogDescriptionProps,
            DialogCloseProps,
        );
    }
}
