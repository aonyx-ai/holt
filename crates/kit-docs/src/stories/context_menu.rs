// @component context_menu
#[cfg(feature = "e2e")]
mod e2e;

use holt_book::{story, variant};
use holt_kit::visual::{
    ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuSeparator, ContextMenuTrigger,
};
use leptos::prelude::*;

#[variant]
fn default() -> AnyView {
    view! {
        <ContextMenu>
            <ContextMenuTrigger class="flex h-[150px] w-[300px] items-center justify-center rounded-md border border-dashed text-sm">
                "Right click here"
            </ContextMenuTrigger>
            <ContextMenuContent>
                <ContextMenuItem>"Back"</ContextMenuItem>
                <ContextMenuItem>"Forward"</ContextMenuItem>
                <ContextMenuItem>"Reload"</ContextMenuItem>
                <ContextMenuSeparator />
                <ContextMenuItem>"Save As..."</ContextMenuItem>
                <ContextMenuItem>"Print..."</ContextMenuItem>
            </ContextMenuContent>
        </ContextMenu>
    }
    .into_any()
}

#[variant]
fn with_disabled_items() -> AnyView {
    view! {
        <ContextMenu>
            <ContextMenuTrigger class="flex h-[150px] w-[300px] items-center justify-center rounded-md border border-dashed text-sm">
                "Right click here"
            </ContextMenuTrigger>
            <ContextMenuContent>
                <ContextMenuItem>"Cut"</ContextMenuItem>
                <ContextMenuItem>"Copy"</ContextMenuItem>
                <ContextMenuItem disabled=true>"Paste"</ContextMenuItem>
                <ContextMenuSeparator />
                <ContextMenuItem>"Select All"</ContextMenuItem>
            </ContextMenuContent>
        </ContextMenu>
    }
    .into_any()
}

#[variant]
fn with_callbacks() -> AnyView {
    let last_action = RwSignal::new("None".to_string());

    view! {
        <div class="space-y-4">
            <ContextMenu>
                <ContextMenuTrigger class="flex h-[150px] w-[300px] items-center justify-center rounded-md border border-dashed text-sm">
                    "Right click here"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem on_select=Some(
                        Callback::new(move |_| { last_action.set("New File".to_string()) }),
                    )>"New File"</ContextMenuItem>
                    <ContextMenuItem on_select=Some(
                        Callback::new(move |_| { last_action.set("New Folder".to_string()) }),
                    )>"New Folder"</ContextMenuItem>
                    <ContextMenuSeparator />
                    <ContextMenuItem on_select=Some(
                        Callback::new(move |_| { last_action.set("Delete".to_string()) }),
                    )>"Delete"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
            <p class="text-sm text-gray-600">"Last action: " {move || last_action.get()}</p>
        </div>
    }
    .into_any()
}

include!(concat!(env!("OUT_DIR"), "/stories/context_menu_source.rs"));

#[story(id = "context-menu", name = "Context Menu", extra_docs = CONTEXT_MENU_SOURCE)]
/// Context menu triggered by right-clicking on a target area
const CONTEXT_MENU_STORY: () = &[default, with_disabled_items, with_callbacks];
