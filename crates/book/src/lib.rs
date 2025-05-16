mod ui;

use leptos::mount::mount_to_body;
use leptos::view;
use crate::ui::story::init_story_registry;
use crate::ui::app::App;

pub use crate::ui::story::{StoryAsView, Story, StoryNew, StoryMetadata};
pub use inventory::submit;

pub fn run_book() {
    init_story_registry();

    // Set up logging
    let _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! { <App /> }
    })
}
