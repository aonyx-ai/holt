use holt_kit::behavior::*;
use holt_kit::visual::*;
use leptos::prelude::*;
use leptos_router::components::{A, Route, Routes};
use leptos_router::hooks::use_params;
use leptos_router::params::Params;
use leptos_router::path;

use crate::ui::components::markdown::Markdown;
use crate::ui::story::Story;

#[derive(Params, PartialEq)]
struct StoryParams {
    story_id: Option<String>,
}

#[derive(Params, PartialEq)]
struct VisualTestParams {
    story_id: Option<String>,
    variant_index: Option<String>,
}

/// Main storybook layout component
#[component]
pub fn Storybook() -> impl IntoView {
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
                    <div class="flex flex-1 flex-col overflow-hidden">
                        <div class="flex-1 overflow-auto p-4">
                            <Routes fallback=|| "not found">
                                <Route path=path!("/") view=|| "no story selected" />
                                <Route
                                    path=path!("/story/:story_id")
                                    view=move || view! { <StorybookStory /> }
                                />
                            </Routes>
                        </div>
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
                {inventory::iter::<&'static Story>
                    .into_iter()
                    .map(|story| {
                        view! {
                            <li>
                                <A
                                    href=move || format!("/story/{}", story.id)
                                    {..}
                                    class="block px-2 py-1 rounded hover:bg-muted"
                                >
                                    {story.name}
                                </A>
                            </li>
                        }
                    })
                    .collect_view()}
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
            inventory::iter::<&'static Story>
                .into_iter()
                .find(|story| story.id == id)
                .map_or_else(
                    || {
                        view! {
                            <div class="flex flex-col items-center justify-center h-full">
                                <p class="text-center">Unknown story</p>
                            </div>
                        }
                        .into_any()
                    },
                    |story| view! { <StoryVariantDisplay story=story /> }.into_any(),
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

/// Component for displaying story variants with selection
#[component]
fn StoryVariantDisplay(story: &'static Story) -> impl IntoView {
    let (selected_variant, set_selected_variant) = signal(0);
    let (active_tab, set_active_tab) = signal("preview");

    let variants = story.variants;

    view! {
        <div class="flex flex-col space-y-6 max-w-full">
            <div class="space-y-2">
                <H1 class="text-2xl font-bold">{story.name}</H1>
            </div>

            <div class="flex items-center space-x-4">
                <select
                    class="px-3 py-2 border rounded-md bg-background"
                    on:change=move |ev| {
                        let value = event_target_value(&ev);
                        if let Ok(index) = value.parse::<usize>() {
                            set_selected_variant.set(index);
                        }
                    }
                >
                    {variants
                        .iter()
                        .enumerate()
                        .map(|(i, variant)| {
                            view! {
                                <option
                                    value=i.to_string()
                                    selected=move || selected_variant.get() == i
                                >
                                    {variant.name}
                                </option>
                            }
                        })
                        .collect::<Vec<_>>()}
                </select>
            </div>

            <div class="border rounded-lg">
                <div class="flex border-b">
                    <button
                        class=move || {
                            format!(
                                "px-4 py-2 text-sm font-medium border-b-2 transition-colors {}",
                                if active_tab.get() == "preview" {
                                    "border-primary text-primary bg-muted/50"
                                } else {
                                    "border-transparent text-muted-foreground hover:text-foreground"
                                },
                            )
                        }
                        on:click=move |_| set_active_tab.set("preview")
                    >
                        "Preview"
                    </button>
                    <button
                        class=move || {
                            format!(
                                "px-4 py-2 text-sm font-medium border-b-2 transition-colors {}",
                                if active_tab.get() == "code" {
                                    "border-primary text-primary bg-muted/50"
                                } else {
                                    "border-transparent text-muted-foreground hover:text-foreground"
                                },
                            )
                        }
                        on:click=move |_| set_active_tab.set("code")
                    >
                        "Code"
                    </button>
                </div>

                <div class="p-6">
                    {move || {
                        let index = selected_variant.get();
                        if let Some(variant) = variants.get(index) {
                            if active_tab.get() == "preview" {
                                view! {
                                    <div class="flex items-center justify-center min-h-[200px] bg-muted/20 rounded-lg">
                                        {(variant.render)()}
                                    </div>
                                }
                                    .into_any()
                            } else {
                                view! {
                                    <div class="bg-muted/30 rounded-lg p-4">
                                        <pre class="text-sm overflow-x-auto">
                                            <code>{variant.source}</code>
                                        </pre>
                                    </div>
                                }
                                    .into_any()
                            }
                        } else {
                            view! { <div>"No variant selected"</div> }.into_any()
                        }
                    }}
                </div>
            </div>

            {story
                .description
                .map(|desc| {
                    view! {
                        <div class="max-w-full">
                            <H2>Documentation</H2>
                            <div class="max-w-full overflow-hidden">
                                <Markdown content=desc.to_string() />
                            </div>
                        </div>
                    }
                })}
        </div>
    }
}

/// Minimal component for visual regression testing - renders just the story variant
#[component]
pub fn VisualTestStory() -> impl IntoView {
    let params = use_params::<VisualTestParams>();

    move || {
        let story_id = params
            .read()
            .as_ref()
            .ok()
            .and_then(|params| params.story_id.clone());

        let variant_index = params
            .read()
            .as_ref()
            .ok()
            .and_then(|params| params.variant_index.clone())
            .and_then(|idx| idx.parse::<usize>().ok());

        match (story_id, variant_index) {
            (Some(id), Some(index)) => inventory::iter::<&'static Story>
                .into_iter()
                .find(|story| story.id == id)
                .and_then(|story| story.variants.get(index))
                .map_or_else(
                    || view! { <div>"Story variant not found"</div> }.into_any(),
                    |variant| {
                        view! {
                            <div
                                class="flex items-center justify-center min-h-screen bg-white p-8"
                                data-story-id=id.clone()
                                data-variant-index=index.to_string()
                            >
                                {(variant.render)()}
                            </div>
                        }
                        .into_any()
                    },
                ),
            _ => view! { <div>"Invalid parameters"</div> }.into_any(),
        }
    }
}
