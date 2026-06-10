// @component toggle_group
#[cfg(feature = "e2e")]
mod e2e;

use holt_book::{story, variant};
use holt_kit::behavior::ToggleGroupType;
use holt_kit::visual::{ToggleGroup, ToggleGroupItem, ToggleSize, ToggleVariant};
use leptos::prelude::*;
use leptos_icons::Icon;

#[variant]
fn single() -> AnyView {
    let value = RwSignal::new(vec![]);
    view! {
        <ToggleGroup value=value group_type=ToggleGroupType::Single>
            <ToggleGroupItem value="bold" aria_label="Toggle bold">
                <Icon icon=icondata::LuBold attr:class="h-4 w-4" />
            </ToggleGroupItem>
            <ToggleGroupItem value="italic" aria_label="Toggle italic">
                <Icon icon=icondata::LuItalic attr:class="h-4 w-4" />
            </ToggleGroupItem>
            <ToggleGroupItem value="underline" aria_label="Toggle underline">
                <Icon icon=icondata::LuUnderline attr:class="h-4 w-4" />
            </ToggleGroupItem>
        </ToggleGroup>
    }
    .into_any()
}

#[variant]
fn multiple() -> AnyView {
    let value = RwSignal::new(vec![]);
    view! {
        <ToggleGroup value=value group_type=ToggleGroupType::Multiple>
            <ToggleGroupItem value="bold" aria_label="Toggle bold">
                <Icon icon=icondata::LuBold attr:class="h-4 w-4" />
            </ToggleGroupItem>
            <ToggleGroupItem value="italic" aria_label="Toggle italic">
                <Icon icon=icondata::LuItalic attr:class="h-4 w-4" />
            </ToggleGroupItem>
            <ToggleGroupItem value="underline" aria_label="Toggle underline">
                <Icon icon=icondata::LuUnderline attr:class="h-4 w-4" />
            </ToggleGroupItem>
        </ToggleGroup>
    }
    .into_any()
}

#[variant]
fn outline() -> AnyView {
    let value = RwSignal::new(vec![]);
    view! {
        <ToggleGroup value=value group_type=ToggleGroupType::Single variant=ToggleVariant::Outline>
            <ToggleGroupItem value="bold" aria_label="Toggle bold">
                <Icon icon=icondata::LuBold attr:class="h-4 w-4" />
            </ToggleGroupItem>
            <ToggleGroupItem value="italic" aria_label="Toggle italic">
                <Icon icon=icondata::LuItalic attr:class="h-4 w-4" />
            </ToggleGroupItem>
            <ToggleGroupItem value="underline" aria_label="Toggle underline">
                <Icon icon=icondata::LuUnderline attr:class="h-4 w-4" />
            </ToggleGroupItem>
        </ToggleGroup>
    }
    .into_any()
}

#[variant]
fn small() -> AnyView {
    let value = RwSignal::new(vec![]);
    view! {
        <ToggleGroup value=value group_type=ToggleGroupType::Single size=ToggleSize::Sm>
            <ToggleGroupItem value="bold" aria_label="Toggle bold">
                <Icon icon=icondata::LuBold attr:class="h-4 w-4" />
            </ToggleGroupItem>
            <ToggleGroupItem value="italic" aria_label="Toggle italic">
                <Icon icon=icondata::LuItalic attr:class="h-4 w-4" />
            </ToggleGroupItem>
            <ToggleGroupItem value="underline" aria_label="Toggle underline">
                <Icon icon=icondata::LuUnderline attr:class="h-4 w-4" />
            </ToggleGroupItem>
        </ToggleGroup>
    }
    .into_any()
}

#[variant]
fn large() -> AnyView {
    let value = RwSignal::new(vec![]);
    view! {
        <ToggleGroup value=value group_type=ToggleGroupType::Single size=ToggleSize::Lg>
            <ToggleGroupItem value="bold" aria_label="Toggle bold">
                <Icon icon=icondata::LuBold attr:class="h-4 w-4" />
            </ToggleGroupItem>
            <ToggleGroupItem value="italic" aria_label="Toggle italic">
                <Icon icon=icondata::LuItalic attr:class="h-4 w-4" />
            </ToggleGroupItem>
            <ToggleGroupItem value="underline" aria_label="Toggle underline">
                <Icon icon=icondata::LuUnderline attr:class="h-4 w-4" />
            </ToggleGroupItem>
        </ToggleGroup>
    }
    .into_any()
}

#[variant]
fn disabled() -> AnyView {
    let value = RwSignal::new(vec![]);
    view! {
        <ToggleGroup value=value group_type=ToggleGroupType::Single disabled=true>
            <ToggleGroupItem value="bold" aria_label="Toggle bold">
                <Icon icon=icondata::LuBold attr:class="h-4 w-4" />
            </ToggleGroupItem>
            <ToggleGroupItem value="italic" aria_label="Toggle italic">
                <Icon icon=icondata::LuItalic attr:class="h-4 w-4" />
            </ToggleGroupItem>
            <ToggleGroupItem value="underline" aria_label="Toggle underline">
                <Icon icon=icondata::LuUnderline attr:class="h-4 w-4" />
            </ToggleGroupItem>
        </ToggleGroup>
    }
    .into_any()
}

include!(concat!(env!("OUT_DIR"), "/stories/toggle_group_source.rs"));

#[story(
    id = "toggle-group",
    name = "Toggle Group",
    extra_docs = TOGGLE_GROUP_SOURCE
)]
/// A group of toggle buttons where one or multiple can be active
const TOGGLE_GROUP_STORY: () = &[single, multiple, outline, small, large, disabled];
