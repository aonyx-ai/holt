use holt_ui::component::*;
use holt_ui::container::*;
use leptos::prelude::*;

use crate::registry::AllStories;
use crate::registry::ButtonStory;
use crate::registry::Story;

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
                        // <Separator orientation=Orientation::Vertical class="mr-2 h-4" />
                        "Components"
                    </header>
                    <div class="flex flex-1 flex-col gap-4 p-4 overflow-auto">
                        <ComponentDisplay />
                    </div>
                </div>
            </SidebarProvider>
        </div>
    }
}

/// Navigation component for the storybook sidebar
#[component]
fn StorybookNavigation() -> impl IntoView {
    // In a real implementation, this would be generated from a registry of components
    view! {
        <nav class="space-y-1">
            <h2 class="mb-2 text-lg font-semibold">Components</h2>
            <ul class="space-y-1">
                <li>
                    <a href="#" class="block px-2 py-1 rounded hover:bg-muted">
                        "Button"
                    </a>
                </li>
                <li>
                    <a href="#" class="block px-2 py-1 rounded hover:bg-muted">
                        "Card"
                    </a>
                </li>
                <li>
                    <a href="#" class="block px-2 py-1 rounded hover:bg-muted">
                        "Separator"
                    </a>
                </li>
                <li>
                    <a href="#" class="block px-2 py-1 rounded hover:bg-muted">
                        "Typography"
                    </a>
                </li>
            </ul>
        </nav>
    }
}

/// Component display area that shows the selected component and its variants
#[component]
fn ComponentDisplay() -> impl IntoView {
    let button_story = StoredValue::new(AllStories::ButtonStory(ButtonStory::new()));

    button_story.read_value().as_view()
}
