use leptos::prelude::*;
use story::init_story_registry;

use crate::app::App;

// The project template does not ship with any components. Feel free to remove this attribute when
// you've added your first component.
#[allow(unused_imports)]
use crate::components::*;

mod app;
mod components;
mod registry;
mod story;

fn main() {
    init_story_registry();

    // Set up logging
    let _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! { <App /> }
    })
}
