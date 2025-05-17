use holt_ui::behavior::*;
use holt_ui::visual::*;
use leptos::prelude::*;
use leptos_router::components::{A, Route, Routes};
use leptos_router::hooks::use_params;
use leptos_router::params::Params;
use leptos_router::path;
use phf::Map;

use crate::ui::story::Story;

#[derive(Params, PartialEq)]
struct StoryParams {
    story_id: Option<String>,
}

/// Main storybook layout component
#[component]
pub fn Storybook(docs: &'static Map<&'static str, &'static str>) -> impl IntoView {
    view! {
        <div class="flex h-screen w-screen overflow-hidden">
            <SidebarProvider>
                <Sidebar collapsible=SidebarCollapsible::Icon variant=SidebarVariant::Sidebar>
                    <SidebarHeader>
                        <H1>H</H1>
                    </SidebarHeader>
                    <SidebarContent>
                        <StorybookNavigation />
                    </SidebarContent>
                </Sidebar>

                <SidebarInset>
                    <header class="flex h-16 shrink-0 items-center gap-2 border-b px-4">
                        <SidebarTrigger class="-ml-1" />
                        "Components"
                    </header>
                    <div class="flex flex-1 flex-col gap-4 p-4 overflow-auto">
                        <Routes fallback=|| "not found">
                            <Route path=path!("/") view=|| "no story selected" />
                            <Route path=path!("/story/:story_id") view=move || view! { <StorybookStory docs={&docs} /> } />
                        </Routes>
                    </div>
                </SidebarInset>
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
fn StorybookStory(docs: &'static Map<&'static str, &'static str>) -> impl IntoView {
    let params = use_params::<StoryParams>();

    move || {
        let id = params
            .read()
            .as_ref()
            .ok()
            .and_then(|params| params.story_id.clone());

        if let Some(id) = id {
            let docs = docs.get(&id).cloned();

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
                    |story| {
                        view! {
                            <p>{docs.as_ref().map_or("No docs", |docs| docs)}</p>

                            {story.as_view()}
                        }
                        .into_any()
                    },
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
