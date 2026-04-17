// @component alert_dialog
#[cfg(feature = "e2e")]
mod e2e;

use holt_book::{story, variant};
use holt_kit::visual::{
    AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent, AlertDialogDescription,
    AlertDialogFooter, AlertDialogHeader, AlertDialogTitle, AlertDialogTrigger,
};
use leptos::prelude::*;

#[variant]
fn default() -> AnyView {
    view! {
        <AlertDialog>
            <AlertDialogTrigger>
                <span class="inline-flex h-10 items-center justify-center rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90">
                    "Show Dialog"
                </span>
            </AlertDialogTrigger>
            <AlertDialogContent>
                <AlertDialogHeader>
                    <AlertDialogTitle>"Are you absolutely sure?"</AlertDialogTitle>
                    <AlertDialogDescription>
                        "This action cannot be undone. This will permanently delete your account and remove your data from our servers."
                    </AlertDialogDescription>
                </AlertDialogHeader>
                <AlertDialogFooter>
                    <AlertDialogCancel>"Cancel"</AlertDialogCancel>
                    <AlertDialogAction>"Continue"</AlertDialogAction>
                </AlertDialogFooter>
            </AlertDialogContent>
        </AlertDialog>
    }
    .into_any()
}

#[variant]
fn with_callback() -> AnyView {
    let confirmed = RwSignal::new(false);

    view! {
        <div class="space-y-4">
            <AlertDialog>
                <AlertDialogTrigger>
                    <span class="inline-flex h-10 items-center justify-center rounded-md border border-destructive bg-background px-4 py-2 text-sm font-medium text-destructive hover:bg-destructive hover:text-destructive-foreground">
                        "Delete Account"
                    </span>
                </AlertDialogTrigger>
                <AlertDialogContent>
                    <AlertDialogHeader>
                        <AlertDialogTitle>"Delete Account"</AlertDialogTitle>
                        <AlertDialogDescription>
                            "Are you sure you want to delete your account? All of your data will be permanently removed. This action cannot be undone."
                        </AlertDialogDescription>
                    </AlertDialogHeader>
                    <AlertDialogFooter>
                        <AlertDialogCancel>"Cancel"</AlertDialogCancel>
                        <AlertDialogAction on_click=Some(
                            Callback::new(move |()| { confirmed.set(true) }),
                        )>"Yes, delete account"</AlertDialogAction>
                    </AlertDialogFooter>
                </AlertDialogContent>
            </AlertDialog>
            <p class="text-sm text-muted-foreground">
                {move || { if confirmed.get() { "Account deleted." } else { "Account active." } }}
            </p>
        </div>
    }
    .into_any()
}

include!(concat!(env!("OUT_DIR"), "/stories/alert_dialog_source.rs"));

#[story(id = "alert-dialog", name = "Alert Dialog", extra_docs = ALERT_DIALOG_SOURCE)]
/// A modal dialog that interrupts the user with important content and expects a response
const ALERT_DIALOG_STORY: () = &[default, with_callback];
