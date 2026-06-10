// @component Dialog
#[cfg(feature = "e2e")]
mod e2e;

use holt_book::{story, variant};
use holt_kit::visual::{
    Button, ButtonVariant, Dialog, DialogClose, DialogContent, DialogDescription, DialogFooter,
    DialogHeader, DialogTitle, DialogTrigger, Input, Label,
};
use leptos::prelude::*;

#[variant]
fn basic() -> AnyView {
    let open = RwSignal::new(false);
    view! {
        <Dialog open=open>
            <DialogTrigger>
                <Button>"Open Dialog"</Button>
            </DialogTrigger>
            <DialogContent>
                <DialogHeader>
                    <DialogTitle>"Are you sure?"</DialogTitle>
                    <DialogDescription>
                        "This action cannot be undone. This will permanently delete your account and remove your data from our servers."
                    </DialogDescription>
                </DialogHeader>
                <DialogFooter>
                    <DialogClose>
                        <Button variant=ButtonVariant::Outline>"Cancel"</Button>
                    </DialogClose>
                    <Button>"Continue"</Button>
                </DialogFooter>
            </DialogContent>
        </Dialog>
    }
    .into_any()
}

#[variant]
fn with_form() -> AnyView {
    let open = RwSignal::new(false);
    view! {
        <Dialog open=open>
            <DialogTrigger>
                <Button variant=ButtonVariant::Outline>"Edit Profile"</Button>
            </DialogTrigger>
            <DialogContent>
                <DialogHeader>
                    <DialogTitle>"Edit profile"</DialogTitle>
                    <DialogDescription>
                        "Make changes to your profile here. Click save when you're done."
                    </DialogDescription>
                </DialogHeader>
                <div class="grid gap-4 py-4">
                    <div class="grid grid-cols-4 items-center gap-4">
                        <Label class="text-right">"Name"</Label>
                        <Input class="col-span-3" placeholder="Enter your name" />
                    </div>
                    <div class="grid grid-cols-4 items-center gap-4">
                        <Label class="text-right">"Username"</Label>
                        <Input class="col-span-3" placeholder="@username" />
                    </div>
                </div>
                <DialogFooter>
                    <Button>"Save changes"</Button>
                </DialogFooter>
            </DialogContent>
        </Dialog>
    }
    .into_any()
}

include!(concat!(env!("OUT_DIR"), "/stories/dialog_source.rs"));

#[story(id = "dialog", name = "Dialog", extra_docs = DIALOG_SOURCE)]
const DIALOG_STORY: () = &[basic, with_form];
