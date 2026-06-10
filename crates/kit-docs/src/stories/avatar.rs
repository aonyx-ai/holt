// @component Avatar
#[cfg(feature = "e2e")]
mod e2e;

use holt_book::{story, variant};
use holt_kit::visual::{Avatar, AvatarFallback, AvatarImage, AvatarSize};
use leptos::prelude::*;

#[variant]
fn default() -> AnyView {
    view! {
        <Avatar>
            <AvatarImage src="https://github.com/shadcn.png" alt="@shadcn" />
            <AvatarFallback>CN</AvatarFallback>
        </Avatar>
    }
    .into_any()
}

#[variant]
fn fallback_only() -> AnyView {
    view! {
        <Avatar>
            <AvatarFallback>AB</AvatarFallback>
        </Avatar>
    }
    .into_any()
}

#[variant]
fn small() -> AnyView {
    view! {
        <Avatar size=AvatarSize::Sm>
            <AvatarFallback>SM</AvatarFallback>
        </Avatar>
    }
    .into_any()
}

#[variant]
fn large() -> AnyView {
    view! {
        <Avatar size=AvatarSize::Lg>
            <AvatarFallback>LG</AvatarFallback>
        </Avatar>
    }
    .into_any()
}

include!(concat!(env!("OUT_DIR"), "/stories/avatar_source.rs"));

#[story(id = "avatar", name = "Avatar", extra_docs = AVATAR_SOURCE)]
/// An image element with a fallback for representing the user.
const AVATAR_STORY: () = &[default, fallback_only, small, large];
