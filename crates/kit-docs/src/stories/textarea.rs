// @component Textarea
use holt_book::{story, variant};
use holt_kit::visual::{Textarea, TextareaSize};
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
            <Textarea size=TextareaSize::Sm value=v1 placeholder="Small" rows=3 />
            <Textarea value=v2 placeholder="Default" rows=4 />
            <Textarea size=TextareaSize::Lg value=v3 placeholder="Large" rows=5 />
        </div>
    }
    .into_any()
}

#[variant]
fn disabled() -> AnyView {
    let value = RwSignal::new("This textarea is disabled and cannot be edited.".to_string());
    view! {
        <div class="w-80">
            <Textarea value=value disabled=true rows=3 />
        </div>
    }
    .into_any()
}

#[variant]
fn readonly() -> AnyView {
    let value = RwSignal::new("This textarea is readonly and cannot be edited.".to_string());
    view! {
        <div class="w-80">
            <Textarea value=value readonly=true rows=3 />
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
                placeholder="Tell us a bit about yourself..."
                rows=5
            />
            <p class="text-xs text-muted-foreground">
                "You can use markdown in your bio. Max 500 characters."
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
            <Textarea value=value placeholder="Large text area with 10 rows" rows=10 />
            <p class="text-sm text-gray-600">"Lines: " {move || value.get().lines().count()}</p>
        </div>
    }
    .into_any()
}

#[variant]
fn required_with_validation() -> AnyView {
    let value = RwSignal::new(String::new());
    view! {
        <div class="w-80 space-y-2">
            <label for="message" class="text-sm font-medium leading-none">
                "Message "
                <span class="text-red-500">"*"</span>
            </label>
            <Textarea
                id="message"
                name="message"
                value=value
                placeholder="This field is required..."
                rows=4
                required=true
            />
            <p class="text-xs text-muted-foreground">
                {move || {
                    if value.get().is_empty() { "This field is required." } else { "Valid input." }
                }}
            </p>
        </div>
    }
    .into_any()
}

include!(concat!(env!("OUT_DIR"), "/stories/textarea_source.rs"));

#[story(id = "textarea", name = "Textarea", extra_docs = TEXTAREA_SOURCE)]
/// `value` is two‑way bound (`bind:value`).
const TEXTAREA_STORY: () = &[
    default,
    sizes,
    disabled,
    readonly,
    with_label_and_help,
    custom_rows,
    required_with_validation,
];
