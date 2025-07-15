use holt_book::{Story, StoryVariant};
use holt_ui::visual::{Badge, BadgeVariant};
use leptos::prelude::*;
use leptos_icons::Icon;

const BADGE_STORY: Story = Story {
    id: "badge",
    name: "Badge",
    variants: &[
        &StoryVariant {
            name: "Default",
            view: || view! { <Badge>Default</Badge> }.into_any(),
        },
        &StoryVariant {
            name: "Secondary",
            view: || view! { <Badge variant=BadgeVariant::Secondary>Secondary</Badge> }.into_any(),
        },
        &StoryVariant {
            name: "Destructive",
            view: || view! { <Badge variant=BadgeVariant::Destructive>Destructive</Badge> }.into_any(),
        },
        &StoryVariant {
            name: "Outline",
            view: || view! { <Badge variant=BadgeVariant::Outline>Outline</Badge> }.into_any(),
        },
        &StoryVariant {
            name: "With Icon",
            view: || view! { <Badge
                variant=BadgeVariant::Secondary
                class="bg-blue-500 text-white dark:bg-blue-600"
            >
                <Icon icon=icondata::LuBadgeCheck />
                Verified
            </Badge> }
            .into_any(),
        },
        &StoryVariant {
            name: "Counter",
            view:  || view! { <Badge class="h-5 min-w-5 rounded-full px-1 font-mono tabular-nums">
                8
            </Badge> }
            .into_any(),
        },
        &StoryVariant {
            name: "Counter Destructive",
            view:  || view! { <Badge
                variant=BadgeVariant::Destructive
                class="h-5 min-w-5 rounded-full px-1 font-mono tabular-nums"
            >
                99
            </Badge> }
            .into_any(),
        },
        &StoryVariant {
            name: "Counter Outline",
            view:  || view! { <Badge
                variant=BadgeVariant::Outline
                class="h-5 min-w-5 rounded-full px-1 font-mono tabular-nums"
            >
                20+
            </Badge> }
            .into_any(),
        },
    ],
};

holt_book::submit!(BADGE_STORY);
