use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::{ParentRoute, Route, Router, Routes};
use leptos_router::path;

use crate::ui::components::{MobileHeader, StorybookLayout, StorybookStory, VisualTestStory};
#[cfg(feature = "ssr")]
use crate::ui::story::Story;

/// Context for the base path used in SSG
#[derive(Clone, Debug, Default)]
pub struct BasePath(pub &'static str);

/// Returns all story IDs for static site generation
#[cfg(feature = "ssr")]
pub fn get_all_story_ids() -> Vec<String> {
    inventory::iter::<&'static Story>
        .into_iter()
        .map(|story| story.id.to_string())
        .collect()
}

/// Returns all routes that need static generation: ("/", and "/story/{id}" for each story)
#[cfg(feature = "ssr")]
pub fn get_static_routes() -> Vec<String> {
    let mut routes = vec!["/".to_string()];
    for story in inventory::iter::<&'static Story> {
        routes.push(format!("/story/{}", story.id));
    }
    routes
}

#[component]
fn KitNavbar() -> impl IntoView {
    view! {
        <nav class="kit-navbar">
            <div class="kit-navbar-inner">
                <a href="/" class="kit-navbar-logo">
                    <img src="/img/logo.svg" alt="Holt" height="32" />
                </a>
                <div class="kit-navbar-items">
                    <a href="/docs/tutorials/" class="kit-navbar-link">
                        "Docs"
                    </a>
                    <a href="/kit/" class="kit-navbar-link kit-navbar-link--active">
                        "Kit"
                    </a>
                </div>
                <div class="kit-navbar-items kit-navbar-right">
                    <a
                        href="https://github.com/aonyx-labs/holt"
                        class="kit-navbar-link"
                        target="_blank"
                        rel="noopener noreferrer"
                    >
                        "GitHub"
                    </a>
                </div>
            </div>
        </nav>
    }
}

#[component]
pub fn App(#[prop(optional)] base: &'static str) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    // Provide base path context for components that need it
    provide_context(BasePath(base));

    let show_navbar = !base.is_empty();

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:data-theme="light" />

        // Set the document title
        <Title text="Holt UI Storybook" />

        // Inject metadata in the <head> of the page
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

        {show_navbar.then(|| view! { <KitNavbar /> })}

        <div class=if show_navbar { "kit-content" } else { "" }>
            <Router base=base>
                <Routes fallback=|| "not found">
                    <Route
                        path=path!("/visual-test/:story_id/:variant_index")
                        view=move || view! { <VisualTestStory /> }
                    />
                    <ParentRoute path=path!("/") view=move || view! { <StorybookLayout /> }>
                        <Route
                            path=path!("/")
                            view=|| {
                                view! {
                                    <MobileHeader />
                                    <div class="flex-1 overflow-auto p-4">"no story selected"</div>
                                }
                            }
                        />
                        <Route
                            path=path!("/story/:story_id")
                            view=move || view! { <StorybookStory /> }
                        />
                    </ParentRoute>
                </Routes>
            </Router>
        </div>
    }
}
