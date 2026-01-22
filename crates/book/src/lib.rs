mod ui;

pub use crate::ui::app::{App, AppProps};
use crate::ui::story::init_story_registry;
pub use crate::ui::story::{Story, StoryVariant};
pub use const_format::concatcp;
pub use holt_story_macro::{story, variant};
pub use inventory::submit;

#[cfg(feature = "ssr")]
pub use crate::ui::app::{get_all_story_ids, get_static_routes};

#[cfg(feature = "csr")]
pub fn run_book() {
    use leptos::mount::mount_to_body;
    use leptos::view;

    init_story_registry();

    // Set up logging
    let _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(move || {
        view! { <App /> }
    })
}

#[cfg(feature = "ssr")]
pub fn init_for_ssr() {
    init_story_registry();
}
