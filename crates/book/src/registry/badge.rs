use crate::story::{register_story, StoryAsView};
use holt_ui::visual::{Badge, BadgeVariant, H1, H2};
use leptos::prelude::*;
use leptos_icons::Icon;

struct BadgeStory;

impl StoryAsView for BadgeStory {
    fn as_view(&self) -> AnyView {
        view! {
            <>
                <H1>Badge</H1>

                <H2>Variants</H2>

                <div class="flex flex-col items-center gap-2">
                    <div class="flex w-full flex-wrap gap-2">
                        <Badge>Default</Badge>
                        <Badge variant=BadgeVariant::Secondary>Secondary</Badge>
                        <Badge variant=BadgeVariant::Destructive>Destructive</Badge>
                        <Badge variant=BadgeVariant::Outline>Outline</Badge>
                    </div>
                </div>

                <H2>Customization</H2>

                <div class="flex flex-col items-center gap-2">
                    <div class="flex w-full flex-wrap gap-2">
                        <Badge
                            variant=BadgeVariant::Secondary
                            class="bg-blue-500 text-white dark:bg-blue-600"
                        >
                            <Icon icon=icondata::LuBadgeCheck />
                            Verified
                        </Badge>
                        <Badge class="h-5 min-w-5 rounded-full px-1 font-mono tabular-nums">
                            8
                        </Badge>
                        <Badge
                            variant=BadgeVariant::Destructive
                            class="h-5 min-w-5 rounded-full px-1 font-mono tabular-nums"
                        >
                            99
                        </Badge>
                        <Badge
                            variant=BadgeVariant::Outline
                            class="h-5 min-w-5 rounded-full px-1 font-mono tabular-nums"
                        >
                            20+
                        </Badge>
                    </div>
                </div>
            </>
        }
        .into_any()
    }
}

register_story!(BadgeStory, "Badge");
