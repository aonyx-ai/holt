// @component Alert
#[cfg(feature = "e2e")]
mod e2e;

use holt_book::{story, variant};
use holt_kit::visual::{Alert, AlertDescription, AlertTitle, AlertVariant};
use leptos::prelude::*;
use leptos_icons::Icon;

#[variant]
fn default() -> AnyView {
    view! {
        <Alert>
            <AlertTitle>Heads up!</AlertTitle>
            <AlertDescription>You can add components to your app using the CLI.</AlertDescription>
        </Alert>
    }
    .into_any()
}

#[variant]
fn destructive() -> AnyView {
    view! {
        <Alert variant=AlertVariant::Destructive>
            <AlertTitle>Error</AlertTitle>
            <AlertDescription>Your session has expired. Please log in again.</AlertDescription>
        </Alert>
    }
    .into_any()
}

#[variant]
fn with_icon() -> AnyView {
    view! {
        <Alert>
            <Icon icon=icondata::LuTerminal />
            <AlertTitle>Heads up!</AlertTitle>
            <AlertDescription>You can add components to your app using the CLI.</AlertDescription>
        </Alert>
    }
    .into_any()
}

#[variant]
fn destructive_with_icon() -> AnyView {
    view! {
        <Alert variant=AlertVariant::Destructive>
            <Icon icon=icondata::LuCircleAlert />
            <AlertTitle>Error</AlertTitle>
            <AlertDescription>Your session has expired. Please log in again.</AlertDescription>
        </Alert>
    }
    .into_any()
}

include!(concat!(env!("OUT_DIR"), "/stories/alert_source.rs"));

#[story(id = "alert", name = "Alert", extra_docs = ALERT_SOURCE)]
/// Displays a callout for important information
const ALERT_STORY: () = &[default, destructive, with_icon, destructive_with_icon];
