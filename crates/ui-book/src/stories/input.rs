// @component Input
use holt_book::{story, variant};
use holt_ui::visual::{Input, InputSize};
use leptos::prelude::*;

#[variant]
fn default() -> AnyView {
    let value = RwSignal::new(String::new());
    view! {
        <div class="w-80 space-y-2">
            <Input value=value placeholder="Email" />
            <p class="text-sm text-gray-600">"Value: " {move || value.get()}</p>
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
            <Input size=InputSize::Sm value=v1 placeholder="Small" />
            <Input value=v2 placeholder="Default" />
            <Input size=InputSize::Lg value=v3 placeholder="Large" />
        </div>
    }
    .into_any()
}

#[variant]
fn disabled() -> AnyView {
    let value = RwSignal::new("Disabled".to_string());
    view! {
        <div class="w-80">
            <Input value=value disabled=true />
        </div>
    }
    .into_any()
}

#[variant]
fn with_label_and_help() -> AnyView {
    let value = RwSignal::new(String::new());
    view! {
        <div class="w-80 space-y-2">
            <label for="email" class="text-sm font-medium leading-none">
                "Email"
            </label>
            <Input id="email" name="email" value=value placeholder="you@example.com" />
            <p class="text-xs text-muted-foreground">"We’ll never share your email."</p>
        </div>
    }
    .into_any()
}

include!(concat!(env!("OUT_DIR"), "/stories/input_source.rs"));

#[story(id = "input", name = "Input", extra_docs = INPUT_SOURCE)]
/// `value` is two‑way bound (`bind:value`).
const INPUT_STORY: () = &[default, sizes, disabled, with_label_and_help];
