use holt_book::{story, variant};
use holt_ui::visual::{Checkbox, CheckboxSize};
use leptos::prelude::*;

#[variant]
fn default() -> AnyView {
    let checked = RwSignal::new(false);
    view! { <Checkbox checked=checked /> }.into_any()
}

#[variant]
fn checked() -> AnyView {
    let checked = RwSignal::new(true);
    view! { <Checkbox checked=checked /> }.into_any()
}

#[variant]
fn disabled() -> AnyView {
    let checked = RwSignal::new(false);
    view! { <Checkbox checked=checked disabled=true /> }.into_any()
}

#[variant]
fn disabled_checked() -> AnyView {
    let checked = RwSignal::new(true);
    view! { <Checkbox checked=checked disabled=true /> }.into_any()
}

#[variant]
fn small() -> AnyView {
    let checked = RwSignal::new(false);
    view! { <Checkbox checked=checked size=CheckboxSize::Sm /> }.into_any()
}

#[variant]
fn large() -> AnyView {
    let checked = RwSignal::new(false);
    view! { <Checkbox checked=checked size=CheckboxSize::Lg /> }.into_any()
}

#[variant]
fn with_label() -> AnyView {
    let checked = RwSignal::new(false);
    view! {
        <div class="flex items-center space-x-2">
            <Checkbox checked=checked id="terms" />
            <label for="terms" class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70">
                "Accept terms and conditions"
            </label>
        </div>
    }.into_any()
}

#[variant]
fn interactive() -> AnyView {
    let checked = RwSignal::new(false);
    let count = RwSignal::new(0);

    let on_change = Box::new(move |is_checked: bool| {
        if is_checked {
            count.update(|c| *c += 1);
        }
    });

    view! {
        <div class="flex flex-col space-y-2">
            <div class="flex items-center space-x-2">
                <Checkbox checked=checked on_change=on_change />
                <label class="text-sm font-medium">
                    "Toggle me"
                </label>
            </div>
            <p class="text-sm text-muted-foreground">
                "Checked " {move || count.get()} " times"
            </p>
        </div>
    }
    .into_any()
}

#[variant]
fn multiple_options() -> AnyView {
    let option1 = RwSignal::new(true);
    let option2 = RwSignal::new(false);
    let option3 = RwSignal::new(true);

    view! {
        <div class="flex flex-col space-y-3">
            <div class="flex items-center space-x-2">
                <Checkbox checked=option1 id="option1" />
                <label for="option1" class="text-sm font-medium">
                    "Email notifications"
                </label>
            </div>
            <div class="flex items-center space-x-2">
                <Checkbox checked=option2 id="option2" />
                <label for="option2" class="text-sm font-medium">
                    "SMS notifications"
                </label>
            </div>
            <div class="flex items-center space-x-2">
                <Checkbox checked=option3 id="option3" />
                <label for="option3" class="text-sm font-medium">
                    "Push notifications"
                </label>
            </div>
        </div>
    }
    .into_any()
}

include!(concat!(env!("OUT_DIR"), "/stories/checkbox_source.rs"));

#[story(id = "checkbox", name = "Checkbox", extra_docs = CHECKBOX_SOURCE)]
/// Checkboxes allow users to select one or more items from a set of options
const CHECKBOX_STORY: () = &[
    default,
    checked,
    disabled,
    disabled_checked,
    small,
    large,
    with_label,
    interactive,
    multiple_options,
];
