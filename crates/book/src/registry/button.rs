use crate::story::build_story;
use holt_ui::component::Button;
use leptos::prelude::*;

build_story!(
    ButtonStory,
    "Button",
    view! {
        <Button>"Click me!"</Button>
    }
);
