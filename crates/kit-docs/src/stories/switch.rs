// @component Switch
use holt_book::{story, variant};
use holt_kit::visual::{Label, Switch, SwitchSize};
use leptos::prelude::*;

#[variant]
fn default() -> AnyView {
    // let checked = RwSignal::new(false);
    view! { <Switch /> }.into_any()
}

#[variant]
fn checked() -> AnyView {
    let checked = RwSignal::new(true);
    view! { <Switch checked=checked /> }.into_any()
}

#[variant]
fn disabled() -> AnyView {
    let checked = RwSignal::new(false);
    view! { <Switch checked=checked disabled=true /> }.into_any()
}

#[variant]
fn disabled_checked() -> AnyView {
    let checked = RwSignal::new(true);
    view! { <Switch checked=checked disabled=true /> }.into_any()
}

#[variant]
fn small() -> AnyView {
    let checked = RwSignal::new(false);
    view! { <Switch checked=checked size=SwitchSize::Sm /> }.into_any()
}

#[variant]
fn large() -> AnyView {
    let checked = RwSignal::new(false);
    view! { <Switch checked=checked size=SwitchSize::Lg /> }.into_any()
}

#[variant]
fn with_label() -> AnyView {
    let checked = RwSignal::new(false);
    view! {
        <div class="flex items-center space-x-2">
            <Switch checked=checked id="airplane-mode" />
            <Label r#for="airplane-mode">"Airplane Mode"</Label>
        </div>
    }
    .into_any()
}

#[variant]
fn interactive() -> AnyView {
    let checked = RwSignal::new(false);

    view! {
        <div class="flex flex-col space-y-2">
            <div class="flex items-center space-x-2">
                <Switch checked=checked id="notifications" />
                <Label r#for="notifications">"Notifications"</Label>
            </div>
            <p class="text-sm text-muted-foreground">
                {move || {
                    if checked.get() { "Notifications enabled" } else { "Notifications disabled" }
                }}
            </p>
        </div>
    }
    .into_any()
}

#[variant]
fn multiple_options() -> AnyView {
    let wifi = RwSignal::new(true);
    let bluetooth = RwSignal::new(false);
    let airplane = RwSignal::new(false);

    view! {
        <div class="flex flex-col space-y-3">
            <div class="flex items-center space-x-2">
                <Switch checked=wifi id="wifi" />
                <Label r#for="wifi">"Wi-Fi"</Label>
            </div>
            <div class="flex items-center space-x-2">
                <Switch checked=bluetooth id="bluetooth" />
                <Label r#for="bluetooth">"Bluetooth"</Label>
            </div>
            <div class="flex items-center space-x-2">
                <Switch checked=airplane id="airplane" />
                <Label r#for="airplane">"Airplane Mode"</Label>
            </div>
        </div>
    }
    .into_any()
}

include!(concat!(env!("OUT_DIR"), "/stories/switch_source.rs"));

#[story(id = "switch", name = "Switch", extra_docs = SWITCH_SOURCE)]
/// Switches allow users to toggle between two mutually exclusive states
const SWITCH_STORY: () = &[
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
