use holt_book::{register_story, StoryAsView};
use holt_ui::visual::{Button, ButtonVariant, H1};
use leptos::prelude::*;

/// Represents a story for the Button component.
///
/// Buttons are for clicking
struct ButtonStory;

impl StoryAsView for ButtonStory {
    fn as_view(&self) -> AnyView {
        view! {
            <>
                <H1>Variants</H1>

                <p>Default</p>
                <Button class="w-32">"Click me!"</Button>

                <p>Destructive</p>
                <Button class="w-32" variant=ButtonVariant::Destructive>"Click me!"</Button>

                <p>Outline</p>
                <Button class="w-32" variant=ButtonVariant::Outline>"Click me!"</Button>

                <p>Secondary</p>
                <Button class="w-32" variant=ButtonVariant::Secondary>"Click me!"</Button>

                <p>Ghost</p>
                <Button class="w-32" variant=ButtonVariant::Ghost>"Click me!"</Button>

                <p>Link</p>
                <Button class="w-32" variant=ButtonVariant::Link>"Click me!"</Button>
            </>
        }
        .into_any()
    }
}

register_story!(ButtonStory, "Button");
