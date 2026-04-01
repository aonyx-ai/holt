// @component Popover
#[cfg(feature = "e2e")]
mod e2e;

use holt_book::{story, variant};
use holt_kit::visual::{Button, Popover, PopoverContent, PopoverTrigger};
use leptos::prelude::*;

#[variant]
fn basic() -> AnyView {
    view! {
        <Popover>
            <PopoverTrigger>
                <Button>"Open popover"</Button>
            </PopoverTrigger>
            <PopoverContent>
                <div class="space-y-2">
                    <h4 class="font-medium leading-none">"Dimensions"</h4>
                    <p class="text-sm text-muted-foreground">
                        "Set the dimensions for the layer."
                    </p>
                </div>
            </PopoverContent>
        </Popover>
    }
    .into_any()
}

#[variant]
fn controlled() -> AnyView {
    let open = RwSignal::new(false);

    view! {
        <div class="space-y-3">
            <div class="flex gap-2">
                <Button on:click=move |_| open.set(true)>"Open"</Button>
                <Button
                    variant=holt_kit::visual::ButtonVariant::Outline
                    on:click=move |_| open.set(false)
                >
                    "Close"
                </Button>
            </div>
            <Popover open=open>
                <PopoverTrigger>
                    <Button variant=holt_kit::visual::ButtonVariant::Secondary>
                        "Toggle popover"
                    </Button>
                </PopoverTrigger>
                <PopoverContent>
                    <p class="text-sm">"This popover is controlled externally."</p>
                </PopoverContent>
            </Popover>
        </div>
    }
    .into_any()
}

#[variant]
fn with_form() -> AnyView {
    view! {
        <Popover>
            <PopoverTrigger>
                <Button variant=holt_kit::visual::ButtonVariant::Outline>"Edit settings"</Button>
            </PopoverTrigger>
            <PopoverContent class="w-80">
                <div class="grid gap-4">
                    <div class="space-y-2">
                        <h4 class="font-medium leading-none">"Settings"</h4>
                        <p class="text-sm text-muted-foreground">
                            "Configure your preferences."
                        </p>
                    </div>
                    <div class="grid gap-2">
                        <div class="grid grid-cols-3 items-center gap-4">
                            <label class="text-sm">"Width"</label>
                            <input
                                type="text"
                                value="100%"
                                class="col-span-2 h-8 rounded-md border px-2 text-sm"
                            />
                        </div>
                        <div class="grid grid-cols-3 items-center gap-4">
                            <label class="text-sm">"Height"</label>
                            <input
                                type="text"
                                value="25px"
                                class="col-span-2 h-8 rounded-md border px-2 text-sm"
                            />
                        </div>
                    </div>
                </div>
            </PopoverContent>
        </Popover>
    }
    .into_any()
}

include!(concat!(env!("OUT_DIR"), "/stories/popover_source.rs"));

#[story(id = "popover", name = "Popover", extra_docs = POPOVER_SOURCE)]
const POPOVER_STORY: () = &[basic, controlled, with_form];
