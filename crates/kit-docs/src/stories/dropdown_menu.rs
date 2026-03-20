// @component dropdown_menu
#[cfg(feature = "e2e")]
mod e2e;

use holt_book::{story, variant};
use holt_kit::visual::{
    DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuLabel, DropdownMenuSeparator,
    DropdownMenuTrigger,
};
use leptos::prelude::*;

#[variant]
fn default() -> AnyView {
    view! {
        <DropdownMenu>
            <DropdownMenuTrigger class="h-10 px-4 py-2 border rounded-md">
                "Open Menu"
            </DropdownMenuTrigger>
            <DropdownMenuContent>
                <DropdownMenuItem>"Profile"</DropdownMenuItem>
                <DropdownMenuItem>"Settings"</DropdownMenuItem>
                <DropdownMenuItem>"Keyboard shortcuts"</DropdownMenuItem>
                <DropdownMenuSeparator />
                <DropdownMenuItem>"Log out"</DropdownMenuItem>
            </DropdownMenuContent>
        </DropdownMenu>
    }
    .into_any()
}

#[variant]
fn with_labels() -> AnyView {
    view! {
        <DropdownMenu>
            <DropdownMenuTrigger class="h-10 px-4 py-2 border rounded-md">
                "My Account"
            </DropdownMenuTrigger>
            <DropdownMenuContent>
                <DropdownMenuLabel>"My Account"</DropdownMenuLabel>
                <DropdownMenuSeparator />
                <DropdownMenuItem>"Profile"</DropdownMenuItem>
                <DropdownMenuItem>"Billing"</DropdownMenuItem>
                <DropdownMenuItem>"Team"</DropdownMenuItem>
                <DropdownMenuItem>"Subscription"</DropdownMenuItem>
                <DropdownMenuSeparator />
                <DropdownMenuLabel>"Help"</DropdownMenuLabel>
                <DropdownMenuSeparator />
                <DropdownMenuItem>"Support"</DropdownMenuItem>
                <DropdownMenuItem>"Documentation"</DropdownMenuItem>
                <DropdownMenuSeparator />
                <DropdownMenuItem>"Log out"</DropdownMenuItem>
            </DropdownMenuContent>
        </DropdownMenu>
    }
    .into_any()
}

#[variant]
fn with_disabled_items() -> AnyView {
    view! {
        <DropdownMenu>
            <DropdownMenuTrigger class="h-10 px-4 py-2 border rounded-md">
                "Actions"
            </DropdownMenuTrigger>
            <DropdownMenuContent>
                <DropdownMenuItem>"Edit"</DropdownMenuItem>
                <DropdownMenuItem>"Duplicate"</DropdownMenuItem>
                <DropdownMenuItem disabled=true>"Archive"</DropdownMenuItem>
                <DropdownMenuSeparator />
                <DropdownMenuItem disabled=true>"Delete"</DropdownMenuItem>
            </DropdownMenuContent>
        </DropdownMenu>
    }
    .into_any()
}

include!(concat!(env!("OUT_DIR"), "/stories/dropdown_menu_source.rs"));

#[story(id = "dropdown-menu", name = "Dropdown Menu", extra_docs = DROPDOWN_MENU_SOURCE)]
/// Dropdown menus display a list of actions triggered by a button
const DROPDOWN_MENU_STORY: () = &[default, with_labels, with_disabled_items];
