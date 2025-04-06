use super::story::Story;
use holt_ui::component::Button;
use leptos::prelude::*;

#[derive(Clone)]
pub struct ButtonStory();

impl Story for ButtonStory {
    fn new() -> Self {
        ButtonStory()
    }
    // fn title(&self) -> &str { "Button" }

    fn as_view(&self) -> AnyView {
        view! {
            <Button>"Click me!"</Button>
        }
        .into_any()
    }
}

#[derive(Clone)]
pub enum AllStories {
    ButtonStory(ButtonStory),
}

impl AllStories {
    // pub fn title(&self) -> &str { match self {
    //     AllStories::ButtonStory(_) => "Button",
    // }}

    pub fn as_view(&self) -> AnyView {
        match self {
            AllStories::ButtonStory(story) => story.as_view(),
        }
    }
}
