// @component scroll_area
#[cfg(feature = "e2e")]
mod e2e;

use holt_book::{story, variant};
use holt_kit::visual::{ScrollArea, ScrollOrientation, Separator};
use leptos::prelude::*;

#[variant]
fn default() -> AnyView {
    let tags: Vec<&str> = vec![
        "v1.2.0-beta.1",
        "v1.2.0-beta.2",
        "v1.1.4",
        "v1.1.3",
        "v1.1.2",
        "v1.1.1",
        "v1.1.0",
        "v1.0.4",
        "v1.0.3",
        "v1.0.2",
        "v1.0.1",
        "v1.0.0",
        "v0.9.0",
        "v0.8.0",
        "v0.7.0",
    ];

    view! {
        <ScrollArea class="h-72 w-48 rounded-md border">
            <div class="p-4">
                <h4 class="mb-4 text-sm font-medium leading-none">"Tags"</h4>
                {tags
                    .into_iter()
                    .map(|tag| {
                        view! {
                            <div class="text-sm">{tag}</div>
                            <Separator class="my-2" />
                        }
                    })
                    .collect_view()}
            </div>
        </ScrollArea>
    }
    .into_any()
}

#[variant]
fn horizontal() -> AnyView {
    view! {
        <ScrollArea
            orientation=ScrollOrientation::Horizontal
            class="w-96 whitespace-nowrap rounded-md border"
        >
            <div class="flex w-max space-x-4 p-4">
                <div class="w-48 shrink-0 rounded-md border p-4">
                    <h4 class="text-sm font-medium">"Item 1"</h4>
                    <p class="text-sm text-muted-foreground">"A horizontally scrollable card."</p>
                </div>
                <div class="w-48 shrink-0 rounded-md border p-4">
                    <h4 class="text-sm font-medium">"Item 2"</h4>
                    <p class="text-sm text-muted-foreground">"Another scrollable card."</p>
                </div>
                <div class="w-48 shrink-0 rounded-md border p-4">
                    <h4 class="text-sm font-medium">"Item 3"</h4>
                    <p class="text-sm text-muted-foreground">"Yet another card."</p>
                </div>
                <div class="w-48 shrink-0 rounded-md border p-4">
                    <h4 class="text-sm font-medium">"Item 4"</h4>
                    <p class="text-sm text-muted-foreground">"The last card."</p>
                </div>
            </div>
        </ScrollArea>
    }
    .into_any()
}

include!(concat!(env!("OUT_DIR"), "/stories/scroll_area_source.rs"));

#[story(id = "scroll-area", name = "Scroll Area", extra_docs = SCROLL_AREA_SOURCE)]
/// A scrollable container with custom scrollbar styling
const SCROLL_AREA_STORY: () = &[default, horizontal];
