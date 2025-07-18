use holt_book::StoryVariant;
use holt_story_macro::story;
use holt_ui::visual::{Button, ButtonVariant};
use leptos::prelude::*;

/// Buttons are for clicking and doing button things
#[story(id = "button", name = "Button")]
const BUTTON_STORY: &[&StoryVariant] = &[
    &StoryVariant {
        name: "Default",
        view: || view! { <Button class="w-32">"Click me!"</Button> }.into_any(),
    },
    &StoryVariant {
        name: "Outline",
        view: || {
            view! { <Button class="w-32" variant=ButtonVariant::Outline>"Click me!"</Button> }
                .into_any()
        },
    },
    &StoryVariant {
        name: "Destructive",
        view: || {
            view! { <Button class="w-32" variant=ButtonVariant::Destructive>"Click me!"</Button> }
                .into_any()
        },
    },
    &StoryVariant {
        name: "Secondary",
        view: || {
            view! { <Button class="w-32" variant=ButtonVariant::Secondary>"Click me!"</Button> }
                .into_any()
        },
    },
    &StoryVariant {
        name: "Ghost",
        view: || {
            view! { <Button class="w-32" variant=ButtonVariant::Ghost>"Click me!"</Button> }
                .into_any()
        },
    },
    &StoryVariant {
        name: "Link",
        view: || {
            view! { <Button class="w-32" variant=ButtonVariant::Link>"Click me!"</Button> }
                .into_any()
        },
    },
];
