use crate::story::{StoryAsView, register_story};
use holt_ui::component::Button;
use leptos::prelude::*;

struct ButtonStory;

impl StoryAsView for ButtonStory {
    fn as_view(&self) -> AnyView {
        view! { <Button>"Click me!"</Button> }.into_any()
    }
}

register_story!(ButtonStory, "Button");
