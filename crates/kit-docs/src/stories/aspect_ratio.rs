// @component Aspect_Ratio
#[cfg(feature = "e2e")]
mod e2e;

use holt_book::{story, variant};
use holt_kit::visual::AspectRatio;
use leptos::prelude::*;

#[variant]
fn square() -> AnyView {
    view! {
        <div class="w-[300px]">
            <AspectRatio ratio=1.0>
                <div class="flex h-full w-full items-center justify-center rounded-md bg-muted">
                    "1:1"
                </div>
            </AspectRatio>
        </div>
    }
    .into_any()
}

#[variant]
fn widescreen() -> AnyView {
    view! {
        <div class="w-[450px]">
            <AspectRatio ratio=16.0 / 9.0>
                <div class="flex h-full w-full items-center justify-center rounded-md bg-muted">
                    "16:9"
                </div>
            </AspectRatio>
        </div>
    }
    .into_any()
}

#[variant]
fn standard() -> AnyView {
    view! {
        <div class="w-[400px]">
            <AspectRatio ratio=4.0 / 3.0>
                <div class="flex h-full w-full items-center justify-center rounded-md bg-muted">
                    "4:3"
                </div>
            </AspectRatio>
        </div>
    }
    .into_any()
}

include!(concat!(env!("OUT_DIR"), "/stories/aspect_ratio_source.rs"));

#[story(id = "aspect-ratio", name = "Aspect Ratio", extra_docs = ASPECT_RATIO_SOURCE)]
/// Displays content within a desired aspect ratio
const ASPECT_RATIO_STORY: () = &[square, widescreen, standard];
