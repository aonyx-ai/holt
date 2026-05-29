// @component hover_card
#[cfg(feature = "e2e")]
mod e2e;

use holt_book::{story, variant};
use holt_kit::visual::{HoverCard, HoverCardContent, HoverCardTrigger};
use leptos::prelude::*;

#[variant]
fn default() -> AnyView {
    view! {
        <HoverCard>
            <HoverCardTrigger>
                <a
                    href="https://github.com/aonyx-rs/holt"
                    target="_blank"
                    rel="noreferrer noopener"
                    class="inline-flex items-center rounded-md border px-3 py-1 text-sm font-semibold transition-colors hover:bg-accent"
                >
                    "@holt"
                </a>
            </HoverCardTrigger>
            <HoverCardContent>
                <div class="space-y-2">
                    <h4 class="text-sm font-semibold">"@holt"</h4>
                    <p class="text-sm text-muted-foreground">
                        "A UI toolkit for Leptos following the Shadcn model."
                    </p>
                    <div class="flex items-center gap-2 text-xs text-muted-foreground">
                        <span>"Rust"</span>
                        <span>"Open Source"</span>
                    </div>
                </div>
            </HoverCardContent>
        </HoverCard>
    }
    .into_any()
}

#[variant]
fn custom_delay() -> AnyView {
    view! {
        <HoverCard open_delay=200 close_delay=100>
            <HoverCardTrigger>
                <span class="inline-flex items-center rounded-md border px-3 py-1 text-sm font-semibold transition-colors hover:bg-accent cursor-default">
                    "Fast hover (200ms open, 100ms close)"
                </span>
            </HoverCardTrigger>
            <HoverCardContent>
                <p class="text-sm">"This hover card opens and closes faster than the default."</p>
            </HoverCardContent>
        </HoverCard>
    }
    .into_any()
}

#[variant]
fn profile_card() -> AnyView {
    view! {
        <HoverCard>
            <HoverCardTrigger>
                <a
                    href="#"
                    class="inline-flex items-center gap-2 rounded-md border px-3 py-1 text-sm font-semibold transition-colors hover:bg-accent"
                >
                    <div class="h-6 w-6 rounded-full bg-muted flex items-center justify-center text-xs">
                        "JD"
                    </div>
                    "Jane Doe"
                </a>
            </HoverCardTrigger>
            <HoverCardContent>
                <div class="flex gap-4">
                    <div class="h-10 w-10 rounded-full bg-muted flex items-center justify-center text-sm font-medium">
                        "JD"
                    </div>
                    <div class="space-y-1">
                        <h4 class="text-sm font-semibold">"Jane Doe"</h4>
                        <p class="text-sm text-muted-foreground">
                            "Software engineer working on Leptos components."
                        </p>
                        <p class="text-xs text-muted-foreground">"Joined March 2025"</p>
                    </div>
                </div>
            </HoverCardContent>
        </HoverCard>
    }
    .into_any()
}

include!(concat!(env!("OUT_DIR"), "/stories/hover_card_source.rs"));

#[story(id = "hover-card", name = "Hover Card", extra_docs = HOVER_CARD_SOURCE)]
/// Hover cards display supplementary content when hovering over a trigger element
const HOVER_CARD_STORY: () = &[default, custom_delay, profile_card];
