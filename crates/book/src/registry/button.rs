use crate::story::{register_story, StoryAsView};
use holt_ui::visual::Button;
use leptos::prelude::*;

struct ButtonStory;

impl StoryAsView for ButtonStory {
    fn as_view(&self) -> AnyView {
        view! { <Button>"Click me!"</Button> }.into_any()
    }
}

register_story!(ButtonStory, "Button");
