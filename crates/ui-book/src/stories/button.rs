use holt_story_macro::{story, variant};
use holt_ui::visual::{Button, ButtonVariant};
use leptos::prelude::*;

#[variant]
fn default() -> AnyView {
    view! { <Button class="w-32">"Click me!"</Button> }.into_any()
}

#[variant]
fn outline() -> AnyView {
    view! { <Button class="w-32" variant=ButtonVariant::Outline>"Click me!"</Button> }.into_any()
}

#[variant]
fn destructive() -> AnyView {
    view! { <Button class="w-32" variant=ButtonVariant::Destructive>"Click me!"</Button> }
        .into_any()
}

#[variant]
fn secondary() -> AnyView {
    view! { <Button class="w-32" variant=ButtonVariant::Secondary>"Click me!"</Button> }.into_any()
}

#[variant]
fn ghost() -> AnyView {
    view! { <Button class="w-32" variant=ButtonVariant::Ghost>"Click me!"</Button> }.into_any()
}

#[variant]
fn link() -> AnyView {
    view! { <Button class="w-32" variant=ButtonVariant::Link>"Click me!"</Button> }.into_any()
}

include!(concat!(env!("OUT_DIR"), "/stories/button_source.rs"));

/// Buttons are for clicking and doing button things
#[story(id = "button", name = "Button", extra_docs = BUTTON_SOURCE)]
const BUTTON_STORY: () = &[default, outline, destructive, secondary, ghost, link];
