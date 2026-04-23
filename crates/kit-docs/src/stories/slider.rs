// @component Slider
#[cfg(feature = "e2e")]
mod e2e;

use holt_book::{story, variant};
use holt_kit::visual::{Label, Slider};
use leptos::prelude::*;

#[variant]
fn default() -> AnyView {
    view! { <Slider /> }.into_any()
}

#[variant]
fn with_value() -> AnyView {
    let value = RwSignal::new(50.0);
    view! { <Slider value=value /> }.into_any()
}

#[variant]
fn custom_range() -> AnyView {
    let value = RwSignal::new(5.0);
    view! { <Slider value=value min=0.0 max=10.0 step=1.0 /> }.into_any()
}

#[variant]
fn disabled() -> AnyView {
    let value = RwSignal::new(40.0);
    view! { <Slider value=value disabled=true /> }.into_any()
}

#[variant]
fn with_label() -> AnyView {
    let value = RwSignal::new(25.0);
    view! {
        <div class="flex flex-col space-y-2 w-full max-w-sm">
            <div class="flex items-center justify-between">
                <Label>"Volume"</Label>
                <span class="text-sm text-muted-foreground">
                    {move || format!("{:.0}%", value.get())}
                </span>
            </div>
            <Slider value=value />
        </div>
    }
    .into_any()
}

#[variant]
fn stepped() -> AnyView {
    let value = RwSignal::new(50.0);
    view! {
        <div class="flex flex-col space-y-2 w-full max-w-sm">
            <div class="flex items-center justify-between">
                <Label>"Step: 25"</Label>
                <span class="text-sm text-muted-foreground">
                    {move || format!("{:.0}", value.get())}
                </span>
            </div>
            <Slider value=value step=25.0 />
        </div>
    }
    .into_any()
}

include!(concat!(env!("OUT_DIR"), "/stories/slider_source.rs"));

#[story(id = "slider", name = "Slider", extra_docs = SLIDER_SOURCE)]
/// Sliders allow users to select a value from a continuous or stepped range
const SLIDER_STORY: () = &[
    default,
    with_value,
    custom_range,
    disabled,
    with_label,
    stepped,
];
