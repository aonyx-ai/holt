use leptos::prelude::*;
use tailwind_fuse::*;

use crate::behavior::{
    AlertDialogAction as AlertDialogActionPrimitive,
    AlertDialogCancel as AlertDialogCancelPrimitive,
    AlertDialogContent as AlertDialogContentPrimitive, AlertDialogRoot as AlertDialogRootPrimitive,
    AlertDialogTrigger as AlertDialogTriggerPrimitive,
};

/// The main AlertDialog component
#[component]
pub fn AlertDialog(#[prop(optional)] open: RwSignal<bool>, children: Children) -> impl IntoView {
    view! { <AlertDialogRootPrimitive open=open>{children()}</AlertDialogRootPrimitive> }
}

/// Trigger button that opens the alert dialog
#[component]
pub fn AlertDialogTrigger(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let classes = tw_merge!("", &class);

    view! { <AlertDialogTriggerPrimitive class=classes>{children()}</AlertDialogTriggerPrimitive> }
}

/// Content area with Shadcn styling
#[component]
pub fn AlertDialogContent(
    #[prop(optional, into)] class: String,
    children: ChildrenFn,
) -> impl IntoView {
    let classes = tw_merge!(
        "grid w-full max-w-lg gap-4 border bg-background p-6 shadow-lg sm:rounded-lg",
        &class
    );

    view! { <AlertDialogContentPrimitive class=classes>{children()}</AlertDialogContentPrimitive> }
}

/// Header section for the alert dialog
#[component]
pub fn AlertDialogHeader(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let classes = tw_merge!("flex flex-col space-y-2 text-center sm:text-left", &class);

    view! { <div class=classes>{children()}</div> }
}

/// Footer section for the alert dialog actions
#[component]
pub fn AlertDialogFooter(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let classes = tw_merge!(
        "flex flex-col-reverse sm:flex-row sm:justify-end sm:space-x-2",
        &class
    );

    view! { <div class=classes>{children()}</div> }
}

/// Title for the alert dialog
#[component]
pub fn AlertDialogTitle(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let classes = tw_merge!("text-lg font-semibold", &class);

    view! { <h2 class=classes>{children()}</h2> }
}

/// Description text for the alert dialog
#[component]
pub fn AlertDialogDescription(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let classes = tw_merge!("text-sm text-muted-foreground", &class);

    view! { <p class=classes>{children()}</p> }
}

/// Confirm action button with Shadcn styling
#[component]
pub fn AlertDialogAction(
    #[prop(optional, into)] class: String,
    #[prop(optional_no_strip)] on_click: Option<Callback<()>>,
    children: Children,
) -> impl IntoView {
    let classes = tw_merge!(
        "inline-flex h-10 items-center justify-center rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground ring-offset-background transition-colors hover:bg-primary/90 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50",
        &class
    );

    view! {
        <AlertDialogActionPrimitive class=classes on_click=on_click>
            {children()}
        </AlertDialogActionPrimitive>
    }
}

/// Cancel button with Shadcn styling
#[component]
pub fn AlertDialogCancel(
    #[prop(optional, into)] class: String,
    #[prop(optional_no_strip)] on_click: Option<Callback<()>>,
    children: Children,
) -> impl IntoView {
    let classes = tw_merge!(
        "mt-2 inline-flex h-10 items-center justify-center rounded-md border border-input bg-background px-4 py-2 text-sm font-medium ring-offset-background transition-colors hover:bg-accent hover:text-accent-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 sm:mt-0",
        &class
    );

    view! {
        <AlertDialogCancelPrimitive class=classes on_click=on_click>
            {children()}
        </AlertDialogCancelPrimitive>
    }
}
