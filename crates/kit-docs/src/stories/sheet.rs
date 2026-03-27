// @component Sheet
#[cfg(feature = "e2e")]
mod e2e;

use holt_book::{story, variant};
use holt_kit::visual::{
    Button, ButtonVariant, Sheet, SheetContent, SheetDescription, SheetFooter, SheetHeader,
    SheetSide, SheetTitle, SheetTrigger,
};
use leptos::prelude::*;

#[variant]
fn default() -> AnyView {
    let open = RwSignal::new(false);

    view! {
        <Sheet open=open>
            <SheetTrigger>
                <Button>"Open Sheet"</Button>
            </SheetTrigger>
            <SheetContent>
                <SheetHeader>
                    <SheetTitle>"Edit Profile"</SheetTitle>
                    <SheetDescription>
                        "Make changes to your profile here. Click save when you're done."
                    </SheetDescription>
                </SheetHeader>
                <div class="grid gap-4 py-4">
                    <div class="grid grid-cols-4 items-center gap-4">
                        <label class="text-right text-sm font-medium">"Name"</label>
                        <input
                            class="col-span-3 rounded-md border px-3 py-2 text-sm"
                            value="John Doe"
                        />
                    </div>
                    <div class="grid grid-cols-4 items-center gap-4">
                        <label class="text-right text-sm font-medium">"Username"</label>
                        <input
                            class="col-span-3 rounded-md border px-3 py-2 text-sm"
                            value="@johndoe"
                        />
                    </div>
                </div>
                <SheetFooter>
                    <Button>"Save changes"</Button>
                </SheetFooter>
            </SheetContent>
        </Sheet>
    }
    .into_any()
}

#[variant]
fn left_side() -> AnyView {
    let open = RwSignal::new(false);

    view! {
        <Sheet open=open>
            <SheetTrigger>
                <Button variant=ButtonVariant::Outline>"Open Left"</Button>
            </SheetTrigger>
            <SheetContent side=SheetSide::Left>
                <SheetHeader>
                    <SheetTitle>"Navigation"</SheetTitle>
                    <SheetDescription>"Browse the application sections."</SheetDescription>
                </SheetHeader>
                <nav class="flex flex-col gap-2 py-4">
                    <a href="#" class="rounded-md px-3 py-2 text-sm hover:bg-accent">
                        "Dashboard"
                    </a>
                    <a href="#" class="rounded-md px-3 py-2 text-sm hover:bg-accent">
                        "Settings"
                    </a>
                    <a href="#" class="rounded-md px-3 py-2 text-sm hover:bg-accent">
                        "Profile"
                    </a>
                    <a href="#" class="rounded-md px-3 py-2 text-sm hover:bg-accent">
                        "Help"
                    </a>
                </nav>
            </SheetContent>
        </Sheet>
    }
    .into_any()
}

#[variant]
fn top_side() -> AnyView {
    let open = RwSignal::new(false);

    view! {
        <Sheet open=open>
            <SheetTrigger>
                <Button variant=ButtonVariant::Outline>"Open Top"</Button>
            </SheetTrigger>
            <SheetContent side=SheetSide::Top>
                <SheetHeader>
                    <SheetTitle>"Notifications"</SheetTitle>
                    <SheetDescription>"You have 3 unread messages."</SheetDescription>
                </SheetHeader>
            </SheetContent>
        </Sheet>
    }
    .into_any()
}

#[variant]
fn bottom_side() -> AnyView {
    let open = RwSignal::new(false);

    view! {
        <Sheet open=open>
            <SheetTrigger>
                <Button variant=ButtonVariant::Outline>"Open Bottom"</Button>
            </SheetTrigger>
            <SheetContent side=SheetSide::Bottom>
                <SheetHeader>
                    <SheetTitle>"Cookie Preferences"</SheetTitle>
                    <SheetDescription>
                        "Manage your cookie settings. You can enable or disable different types."
                    </SheetDescription>
                </SheetHeader>
                <SheetFooter>
                    <Button variant=ButtonVariant::Outline>"Decline"</Button>
                    <Button>"Accept"</Button>
                </SheetFooter>
            </SheetContent>
        </Sheet>
    }
    .into_any()
}

include!(concat!(env!("OUT_DIR"), "/stories/sheet_source.rs"));

#[story(id = "sheet", name = "Sheet", extra_docs = SHEET_SOURCE)]
/// Sheet panels that slide out from the edge of the screen
const SHEET_STORY: () = &[default, left_side, top_side, bottom_side];
