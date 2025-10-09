use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

use crate::ui::components::{Storybook, VisualTestStory};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:data-theme="light" />

        // Set the document title
        <Title text="Holt UI Storybook" />

        // Inject metadata in the <head> of the page
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

        <Router>
            <Routes fallback=|| "not found">
                <Route
                    path=path!("/visual-test/:story_id/:variant_index")
                    view=move || view! { <VisualTestStory /> }
                />
                <Route path=path!("/*any") view=move || view! { <Storybook /> } />
            </Routes>
        </Router>
    }
}
