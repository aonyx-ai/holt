use holt_ui::component::*;
use holt_ui::container::*;
use leptos::prelude::*;

use crate::story::Story;

/// Main storybook layout component
#[component]
pub fn Storybook() -> impl IntoView {
    let (selected_story, write_selected_story) = signal::<Option<&'static dyn Story>>(None);

    view! {
        <div class="flex h-screen w-screen overflow-hidden">
            <SidebarProvider>
                <Sidebar>
                    <SidebarHeader class="w-64 bg-background p-4">
                        <h1 class="text-xl font-bold mb-4">Holt UI</h1>
                    </SidebarHeader>
                    <SidebarContent>
                        <StorybookNavigation write_selected_story={write_selected_story} />
                    </SidebarContent>
                    </Sidebar>
                <div class="w-screen">
                    <header class="flex h-16 shrink-0 items-center gap-2 border-b px-4">
                        <SidebarTrigger class="-ml-1" />
                        "Components"
                    </header>
                    <div class="flex flex-1 flex-col gap-4 p-4 overflow-auto">
                        <ComponentDisplay selected_story />
                    </div>
                </div>
            </SidebarProvider>
        </div>
    }
}

/// Navigation component for the storybook sidebar
#[component]
fn StorybookNavigation(
    write_selected_story: WriteSignal<Option<&'static dyn Story>>,
) -> impl IntoView {
    view! {
        <nav class="space-y-1">
            <h2 class="mb-2 text-lg font-semibold">Components</h2>
            <ul class="space-y-1">
                { inventory::iter::<&'static dyn Story>.into_iter().map(|story| {
                    view! {
                        <li>
                            <a
                                href="#"
                                on:click={move |_| write_selected_story.set(Some(*story))}
                                class="block px-2 py-1 rounded hover:bg-muted"
                            >
                                {story.title()}
                            </a>
                        </li>
                    }
                }).collect_view() }
            </ul>
        </nav>
    }
}

/// Component display area that shows the selected component and its variants
#[component]
fn ComponentDisplay(selected_story: ReadSignal<Option<&'static dyn Story>>) -> impl IntoView {
    move || match selected_story.get() {
        Some(story) => story.as_view(),
        None => view! {
            <div class="flex flex-col items-center justify-center h-full">
                <p class="text-muted">No component selected</p>
            </div>
        }
        .into_any(),
    }
}
