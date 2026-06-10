// @component Tooltip
#[cfg(feature = "e2e")]
mod e2e;

use holt_book::{story, variant};
use holt_kit::visual::{Button, Tooltip, TooltipContent, TooltipTrigger};
use leptos::prelude::*;

#[variant]
fn basic() -> AnyView {
    view! {
        <Tooltip>
            <TooltipTrigger>
                <Button variant=holt_kit::visual::ButtonVariant::Outline>"Hover me"</Button>
            </TooltipTrigger>
            <TooltipContent>"This is a tooltip"</TooltipContent>
        </Tooltip>
    }
    .into_any()
}

#[variant]
fn custom_delay() -> AnyView {
    view! {
        <Tooltip open_delay_ms=0u64>
            <TooltipTrigger>
                <Button variant=holt_kit::visual::ButtonVariant::Secondary>"No delay"</Button>
            </TooltipTrigger>
            <TooltipContent>"Instant tooltip"</TooltipContent>
        </Tooltip>
    }
    .into_any()
}

#[variant]
fn bottom_placement() -> AnyView {
    view! {
        <Tooltip>
            <TooltipTrigger>
                <Button>"Bottom tooltip"</Button>
            </TooltipTrigger>
            <TooltipContent>"Appears below"</TooltipContent>
        </Tooltip>
    }
    .into_any()
}

include!(concat!(env!("OUT_DIR"), "/stories/tooltip_source.rs"));

#[story(id = "tooltip", name = "Tooltip", extra_docs = TOOLTIP_SOURCE)]
const TOOLTIP_STORY: () = &[basic, custom_delay, bottom_placement];
