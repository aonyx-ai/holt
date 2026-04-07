// @component Skeleton
#[cfg(feature = "e2e")]
mod e2e;

use holt_book::{story, variant};
use holt_kit::visual::Skeleton;
use leptos::prelude::*;

#[variant]
fn default() -> AnyView {
    view! {
        <div class="flex items-center space-x-4">
            <Skeleton class="size-12 rounded-full" />
            <div class="space-y-2">
                <Skeleton class="h-4 w-[250px]" />
                <Skeleton class="h-4 w-[200px]" />
            </div>
        </div>
    }
    .into_any()
}

#[variant]
fn card() -> AnyView {
    view! {
        <div class="space-y-3">
            <Skeleton class="h-[125px] w-[250px] rounded-xl" />
            <div class="space-y-2">
                <Skeleton class="h-4 w-[250px]" />
                <Skeleton class="h-4 w-[200px]" />
            </div>
        </div>
    }
    .into_any()
}

include!(concat!(env!("OUT_DIR"), "/stories/skeleton_source.rs"));

#[story(id = "skeleton", name = "Skeleton", extra_docs = SKELETON_SOURCE)]
/// A placeholder animation to indicate loading content
const SKELETON_STORY: () = &[default, card];
