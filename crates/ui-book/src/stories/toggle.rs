// @component Toggle
use holt_book::{story, variant};
use holt_ui::visual::{Toggle, ToggleSize, ToggleVariant};
use leptos::prelude::*;
use leptos_icons::Icon;

#[variant]
fn default() -> AnyView {
    let pressed = RwSignal::new(false);
    view! {
        <Toggle pressed=pressed aria_label="Toggle italic">
            <Icon icon=icondata::LuBold attr:class="h-4 w-4" />
        </Toggle>
    }
    .into_any()
}

#[variant]
fn outline() -> AnyView {
    let pressed = RwSignal::new(false);
    view! {
        <Toggle pressed=pressed variant=ToggleVariant::Outline aria_label="Toggle italic">
            <Icon icon=icondata::LuItalic attr:class="h-4 w-4" />
        </Toggle>
    }
    .into_any()
}

#[variant]
fn disabled() -> AnyView {
    let pressed = RwSignal::new(false);
    view! {
        <Toggle pressed=pressed disabled=true aria_label="Toggle underline">
            <Icon icon=icondata::LuUnderline attr:class="h-4 w-4" />
        </Toggle>
    }
    .into_any()
}

#[variant]
fn with_text() -> AnyView {
    let pressed = RwSignal::new(false);
    view! {
        <Toggle pressed=pressed aria_label="Toggle italic">
            <Icon icon=icondata::LuItalic attr:class="h-4 w-4" />
            "Italic"
        </Toggle>
    }
    .into_any()
}

#[variant]
fn small() -> AnyView {
    let pressed = RwSignal::new(false);
    view! {
        <Toggle pressed=pressed size=ToggleSize::Sm aria_label="Toggle bold">
            <Icon icon=icondata::LuBold attr:class="h-4 w-4" />
        </Toggle>
    }
    .into_any()
}

#[variant]
fn large() -> AnyView {
    let pressed = RwSignal::new(false);
    view! {
        <Toggle pressed=pressed size=ToggleSize::Lg aria_label="Toggle bold">
            <Icon icon=icondata::LuBold attr:class="h-4 w-4" />
        </Toggle>
    }
    .into_any()
}

#[variant]
fn pressed_state() -> AnyView {
    let pressed = RwSignal::new(true);
    view! {
        <Toggle pressed=pressed aria_label="Toggle bold">
            <Icon icon=icondata::LuBold attr:class="h-4 w-4" />
        </Toggle>
    }
    .into_any()
}

#[variant]
fn interactive() -> AnyView {
    let pressed = RwSignal::new(false);
    let count = RwSignal::new(0);

    // Create an effect to track when toggle is pressed
    Effect::new(move |_| {
        if pressed.get() {
            count.update(|c| *c += 1);
        }
    });

    view! {
        <div class="flex flex-col space-y-2">
            <Toggle pressed=pressed aria_label="Toggle bold">
                <Icon icon=icondata::LuBold attr:class="h-4 w-4" />
            </Toggle>
            <p class="text-sm text-muted-foreground">"Pressed " {move || count.get()} " times"</p>
        </div>
    }
    .into_any()
}

#[variant]
fn formatting_toolbar() -> AnyView {
    let bold = RwSignal::new(false);
    let italic = RwSignal::new(false);
    let underline = RwSignal::new(false);

    view! {
        <div class="flex items-center space-x-1">
            <Toggle pressed=bold aria_label="Toggle bold">
                <Icon icon=icondata::LuBold attr:class="h-4 w-4" />
            </Toggle>
            <Toggle pressed=italic aria_label="Toggle italic">
                <Icon icon=icondata::LuItalic attr:class="h-4 w-4" />
            </Toggle>
            <Toggle pressed=underline aria_label="Toggle underline">
                <Icon icon=icondata::LuUnderline attr:class="h-4 w-4" />
            </Toggle>
        </div>
    }
    .into_any()
}

include!(concat!(env!("OUT_DIR"), "/stories/toggle_source.rs"));

#[story(id = "toggle", name = "Toggle", extra_docs = TOGGLE_SOURCE)]
/// A two-state button that can be toggled on or off
const TOGGLE_STORY: () = &[
    default,
    outline,
    disabled,
    with_text,
    small,
    large,
    pressed_state,
    interactive,
    formatting_toolbar,
];
