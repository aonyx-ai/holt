// @component Collapsible
use holt_book::{story, variant};
use holt_ui::visual::{Button, Collapsible, CollapsibleContent, CollapsibleTrigger};
use leptos::prelude::*;
use leptos_icons::Icon;

#[variant]
fn basic() -> AnyView {
    let open = RwSignal::new(false);
    view! {
        <Collapsible open=open class="w-full max-w-sm">
            <CollapsibleTrigger class="group">
                <span>"Open details"</span>
            </CollapsibleTrigger>
            <CollapsibleContent class="mt-2 rounded-md border p-3 text-sm text-muted-foreground">
                "This is some additional content that can be hidden."
            </CollapsibleContent>
        </Collapsible>
    }
    .into_any()
}

#[variant]
fn with_icon() -> AnyView {
    let open = RwSignal::new(false);
    view! {
        <Collapsible open=open class="w-full max-w-sm">
            <CollapsibleTrigger class="group">
                <div class="flex w-full items-center justify-between">
                    <span>"Usage stats"</span>
                    <span class="transition-transform group-data-[state=open]:rotate-180">
                        <Icon icon=icondata::LuChevronDown attr:class="size-4" />
                    </span>
                </div>
            </CollapsibleTrigger>
            <CollapsibleContent class="mt-2 space-y-2 text-sm text-muted-foreground">
                <p>"Requests today: 1,204"</p>
                <p>"Errors: 0.02%"</p>
                <p>"Latency p95: 210ms"</p>
            </CollapsibleContent>
        </Collapsible>
    }
    .into_any()
}

#[variant]
fn controlled() -> AnyView {
    let open = RwSignal::new(false);

    view! {
        <div class="space-y-3">
            <div class="flex gap-2">
                <Button on:click=move |_| open.set(true)>"Open"</Button>
                <Button
                    variant=holt_ui::visual::ButtonVariant::Outline
                    on:click=move |_| open.set(false)
                >
                    "Close"
                </Button>
                <Button
                    variant=holt_ui::visual::ButtonVariant::Secondary
                    on:click=move |_| open.update(|o| *o = !*o)
                >
                    "Toggle"
                </Button>
            </div>
            <Collapsible open=open class="w-full max-w-sm">
                <CollapsibleTrigger class="group">
                    <div class="flex w-full items-center justify-between">
                        <span>"Controlled section"</span>
                        <span class="transition-transform group-data-[state=open]:rotate-180">
                            <Icon icon=icondata::LuChevronDown attr:class="size-4" />
                        </span>
                    </div>
                </CollapsibleTrigger>
                <CollapsibleContent class="mt-2 rounded-md border p-3 text-sm text-muted-foreground">
                    "This content stays mounted for smoother animations."
                </CollapsibleContent>
            </Collapsible>
        </div>
    }
    .into_any()
}

include!(concat!(env!("OUT_DIR"), "/stories/collapsible_source.rs"));

#[story(id = "collapsible", name = "Collapsible", extra_docs = COLLAPSIBLE_SOURCE)]
const COLLAPSIBLE_STORY: () = &[basic, with_icon, controlled];
