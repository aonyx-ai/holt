// @component Progress
#[cfg(feature = "e2e")]
mod e2e;

use holt_book::{story, variant};
use holt_kit::visual::Progress;
use leptos::prelude::*;

#[variant]
fn empty() -> AnyView {
    view! { <Progress value=0.0 /> }.into_any()
}

#[variant]
fn half() -> AnyView {
    view! { <Progress value=50.0 /> }.into_any()
}

#[variant]
fn full() -> AnyView {
    view! { <Progress value=100.0 /> }.into_any()
}

#[variant]
fn various() -> AnyView {
    view! {
        <div class="flex flex-col space-y-4 w-full">
            <Progress value=10.0 />
            <Progress value=33.0 />
            <Progress value=66.0 />
            <Progress value=90.0 />
        </div>
    }
    .into_any()
}

include!(concat!(env!("OUT_DIR"), "/stories/progress_source.rs"));

#[story(id = "progress", name = "Progress", extra_docs = PROGRESS_SOURCE)]
/// A progress bar showing completion status
const PROGRESS_STORY: () = &[empty, half, full, various];
