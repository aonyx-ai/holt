mod ui;

use crate::ui::app::App;
use crate::ui::story::init_story_registry;
use leptos::mount::mount_to_body;
use leptos::view;

pub use crate::ui::story::{Story, StoryAsView, StoryVariant};
pub use inventory::submit;
pub use phf::Map;

pub fn run_book() {
    init_story_registry();

    // Set up logging
    let _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(move || {
        view! { <App /> }
    })
}
