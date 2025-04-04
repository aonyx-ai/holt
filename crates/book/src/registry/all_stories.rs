use leptos::prelude::*;
use holt_ui::component::Button;
use super::story::Story;

#[derive(Clone)]
pub struct ButtonStory();

impl Story for ButtonStory {
    fn new() -> Self { ButtonStory() }
    fn title(&self) -> &str { "Button" }

    fn into_view(&self) -> AnyView {
        view! {
            <Button>"Click me!"</Button>
        }.into_any()
    }
}

#[derive(Clone)]
pub enum AllStories {
    ButtonStory(ButtonStory),
}

impl AllStories {
    pub fn title(&self) -> &str { match self {
        AllStories::ButtonStory(_) => "Button",
    }}

    pub fn into_view(&self) -> AnyView {
        match self {
            AllStories::ButtonStory(story) => story.into_view(),
        }
    }
}
