// @component Menubar
#[cfg(feature = "e2e")]
mod e2e;

use holt_book::{story, variant};
use holt_kit::visual::{
    Menubar, MenubarContent, MenubarItem, MenubarMenu, MenubarSeparator, MenubarTrigger,
};
use leptos::prelude::*;

#[variant]
fn default() -> AnyView {
    view! {
        <Menubar>
            <MenubarMenu value="file">
                <MenubarTrigger>"File"</MenubarTrigger>
                <MenubarContent>
                    <MenubarItem>"New Tab"</MenubarItem>
                    <MenubarItem>"New Window"</MenubarItem>
                    <MenubarSeparator />
                    <MenubarItem>"Print"</MenubarItem>
                </MenubarContent>
            </MenubarMenu>
            <MenubarMenu value="edit">
                <MenubarTrigger>"Edit"</MenubarTrigger>
                <MenubarContent>
                    <MenubarItem>"Undo"</MenubarItem>
                    <MenubarItem>"Redo"</MenubarItem>
                    <MenubarSeparator />
                    <MenubarItem>"Cut"</MenubarItem>
                    <MenubarItem>"Copy"</MenubarItem>
                    <MenubarItem>"Paste"</MenubarItem>
                </MenubarContent>
            </MenubarMenu>
            <MenubarMenu value="view">
                <MenubarTrigger>"View"</MenubarTrigger>
                <MenubarContent>
                    <MenubarItem>"Zoom In"</MenubarItem>
                    <MenubarItem>"Zoom Out"</MenubarItem>
                    <MenubarSeparator />
                    <MenubarItem>"Fullscreen"</MenubarItem>
                </MenubarContent>
            </MenubarMenu>
        </Menubar>
    }
    .into_any()
}

#[variant]
fn with_disabled_items() -> AnyView {
    view! {
        <Menubar>
            <MenubarMenu value="file">
                <MenubarTrigger>"File"</MenubarTrigger>
                <MenubarContent>
                    <MenubarItem>"New"</MenubarItem>
                    <MenubarItem>"Open"</MenubarItem>
                    <MenubarItem disabled=true>"Save (disabled)"</MenubarItem>
                    <MenubarSeparator />
                    <MenubarItem>"Exit"</MenubarItem>
                </MenubarContent>
            </MenubarMenu>
        </Menubar>
    }
    .into_any()
}

include!(concat!(env!("OUT_DIR"), "/stories/menubar_source.rs"));

#[story(id = "menubar", name = "Menubar", extra_docs = MENUBAR_SOURCE)]
/// A horizontal menu bar typically used at the top of an application
const MENUBAR_STORY: () = &[default, with_disabled_items];
