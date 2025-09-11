use holt_book::{story, variant};
use holt_ui::visual::{Badge, BadgeVariant};
use leptos::prelude::*;
use leptos_icons::Icon;

#[variant]
fn default() -> AnyView {
    view! { <Badge>Default</Badge> }.into_any()
}

#[variant]
fn secondary() -> AnyView {
    view! { <Badge variant=BadgeVariant::Secondary>Secondary</Badge> }.into_any()
}

#[variant]
fn destructive() -> AnyView {
    view! { <Badge variant=BadgeVariant::Destructive>Destructive</Badge> }.into_any()
}

#[variant]
fn outline() -> AnyView {
    view! { <Badge variant=BadgeVariant::Outline>Outline</Badge> }.into_any()
}

#[variant]
fn verified() -> AnyView {
    view! {
        <Badge variant=BadgeVariant::Secondary class="bg-blue-500 text-white dark:bg-blue-600">
            <Icon icon=icondata::LuBadgeCheck />
            Verified
        </Badge>
    }
    .into_any()
}

#[variant]
fn number_8() -> AnyView {
    view! { <Badge class="h-5 min-w-5 rounded-full px-1 font-mono tabular-nums">8</Badge> }
        .into_any()
}

#[variant]
fn number_99() -> AnyView {
    view! {
        <Badge
            variant=BadgeVariant::Destructive
            class="h-5 min-w-5 rounded-full px-1 font-mono tabular-nums"
        >
            99
        </Badge>
    }
    .into_any()
}

#[variant]
fn number_20_plus() -> AnyView {
    view! {
        <Badge
            variant=BadgeVariant::Outline
            class="h-5 min-w-5 rounded-full px-1 font-mono tabular-nums"
        >
            20+
        </Badge>
    }
    .into_any()
}

include!(concat!(env!("OUT_DIR"), "/stories/badge_source.rs"));

#[story(id = "badge", name = "Badge", extra_docs = BADGE_SOURCE)]
/// Badges are small status indicators
const BADGE_STORY: () = &[
    default,
    secondary,
    destructive,
    outline,
    verified,
    number_8,
    number_99,
    number_20_plus,
];
