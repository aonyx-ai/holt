// @component Separator
use holt_book::{story, variant};
use holt_kit::visual::{Orientation, Separator};
use leptos::prelude::*;

#[variant]
fn default() -> AnyView {
    view! {
        <div class="space-y-1">
            <h4 class="text-sm font-medium leading-none">"Holt UI"</h4>
            <p class="text-sm text-muted-foreground">"An open-source UI component library."</p>
        </div>
        <Separator class="my-4" />
        <div class="flex h-5 items-center space-x-4 text-sm">
            <div>"Docs"</div>
            <Separator orientation=Orientation::Vertical />
            <div>"Source"</div>
            <Separator orientation=Orientation::Vertical />
            <div>"Blog"</div>
        </div>
    }
    .into_any()
}

#[variant]
fn vertical() -> AnyView {
    view! {
        <div class="flex h-5 items-center space-x-4 text-sm">
            <div>"Home"</div>
            <Separator orientation=Orientation::Vertical />
            <div>"About"</div>
            <Separator orientation=Orientation::Vertical />
            <div>"Contact"</div>
        </div>
    }
    .into_any()
}

include!(concat!(env!("OUT_DIR"), "/stories/separator_source.rs"));

#[story(id = "separator", name = "Separator", extra_docs = SEPARATOR_SOURCE)]
/// A visual divider to separate content
const SEPARATOR_STORY: () = &[default, vertical];
