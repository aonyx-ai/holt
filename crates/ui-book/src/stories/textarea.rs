// @component Textarea
use holt_book::{story, variant};
use holt_ui::visual::{Textarea, TextareaSize};
use leptos::prelude::*;

#[variant]
fn default() -> AnyView {
    let value = RwSignal::new(String::new());
    view! {
        <div class="w-80 space-y-2">
            <Textarea value=value placeholder="Type your message here..." />
            <p class="text-sm text-gray-600">"Character count: " {move || value.get().len()}</p>
        </div>
    }
    .into_any()
}

#[variant]
fn sizes() -> AnyView {
    let v1 = RwSignal::new(String::new());
    let v2 = RwSignal::new(String::new());
    let v3 = RwSignal::new(String::new());
    view! {
        <div class="w-80 space-y-3">
            <Textarea size=TextareaSize::Sm value=v1 placeholder="Small" />
            <Textarea value=v2 placeholder="Default" />
            <Textarea size=TextareaSize::Lg value=v3 placeholder="Large" />
        </div>
    }
    .into_any()
}

#[variant]
fn disabled() -> AnyView {
    let value = RwSignal::new("This textarea is disabled".to_string());
    view! {
        <div class="w-80">
            <Textarea value=value disabled=true />
        </div>
    }
    .into_any()
}

#[variant]
fn with_label_and_help() -> AnyView {
    let value = RwSignal::new(String::new());
    view! {
        <div class="w-80 space-y-2">
            <label for="bio" class="text-sm font-medium leading-none">
                "Bio"
            </label>
            <Textarea
                id="bio"
                name="bio"
                value=value
                placeholder="Tell us about yourself..."
            />
            <p class="text-xs text-muted-foreground">
                "You can use markdown syntax for formatting."
            </p>
        </div>
    }
    .into_any()
}

#[variant]
fn custom_rows() -> AnyView {
    let value = RwSignal::new(String::new());
    view! {
        <div class="w-80 space-y-2">
            <Textarea value=value placeholder="Custom height with 10 rows" rows=10 />
        </div>
    }
    .into_any()
}

include!(concat!(env!("OUT_DIR"), "/stories/textarea_source.rs"));

#[story(id = "textarea", name = "Textarea", extra_docs = TEXTAREA_SOURCE)]
/// `value` is two‑way bound (`bind:value`).
const TEXTAREA_STORY: () = &[default, sizes, disabled, with_label_and_help, custom_rows];
