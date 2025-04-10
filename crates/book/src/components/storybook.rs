use holt_ui::component::*;
use holt_ui::container::*;
use leptos::prelude::*;
use leptos_router::components::{Route, Routes, A};
use leptos_router::hooks::use_params;
use leptos_router::params::Params;
use leptos_router::path;

use crate::story::Story;

#[derive(Params, PartialEq)]
struct StoryParams {
    story_id: Option<String>,
}

/// Main storybook layout component
#[component]
pub fn Storybook() -> impl IntoView {
    view! {
        <div class="flex h-screen w-screen overflow-hidden">
            <SidebarProvider>
                <Sidebar>
                    <SidebarHeader class="w-64 bg-background p-4">
                        <h1 class="text-xl font-bold mb-4">Holt UI</h1>
                    </SidebarHeader>
                    <SidebarContent>
                        <StorybookNavigation />
                    </SidebarContent>
                </Sidebar>
                <div class="w-screen">
                    <header class="flex h-16 shrink-0 items-center gap-2 border-b px-4">
                        <SidebarTrigger class="-ml-1" />
                        "Components"
                    </header>
                    <div class="flex flex-1 flex-col gap-4 p-4 overflow-auto">
                        <Routes fallback=|| "not found">
                            <Route path=path!("/") view=|| "no story selected" />
                            <Route path=path!("/story/:story_id") view=StorybookStory />
                        </Routes>
                    </div>
                </div>
            </SidebarProvider>
        </div>
    }
}

/// Navigation component for the storybook sidebar
#[component]
fn StorybookNavigation() -> impl IntoView {
    view! {
        <nav class="space-y-1">
            <h2 class="mb-2 text-lg font-semibold">Stories</h2>
            <ul class="space-y-1">
                { inventory::iter::<&'static dyn Story>.into_iter().map(|story| {
                    view! {
                        <li>
                            <A
                                href=move || format!("/story/{}", story.id())
                                {..}
                                class="block px-2 py-1 rounded hover:bg-muted"
                            >
                                {story.title()}
                            </A>
                        </li>
                    }
                }).collect_view() }
            </ul>
        </nav>
    }
}

/// Component display area that shows the selected component and its variants
#[component]
fn StorybookStory() -> impl IntoView {
    let params = use_params::<StoryParams>();

    move || {
        let id = params
            .read()
            .as_ref()
            .ok()
            .and_then(|params| params.story_id.clone());

        if let Some(id) = id {
            inventory::iter::<&'static dyn Story>
                .into_iter()
                .find(|story| story.id() == id)
                .map_or_else(
                    || {
                        view! {
                            <div class="flex flex-col items-center justify-center h-full">
                                <p class="text-center">Unknown story</p>
                            </div>
                        }
                        .into_any()
                    },
                    |story| story.as_view().into_any(),
                )
        } else {
            view! {
                <div class="flex flex-col items-center justify-center h-full">
                    <p class="text-center">No story selected</p>
                </div>
            }
            .into_any()
        }
    }
}
