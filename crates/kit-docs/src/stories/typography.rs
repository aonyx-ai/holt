// @component Typography
use holt_book::{story, variant};
use holt_kit::visual::{Blockquote, H1, H2, H3, H4, Large, Lead, Muted, P, Small};
use leptos::prelude::*;

#[variant]
fn headings() -> AnyView {
    view! {
        <div class="space-y-4">
            <H1>"This is H1"</H1>
            <H2>"This is H2"</H2>
            <H3>"This is H3"</H3>
            <H4>"This is H4"</H4>
        </div>
    }
    .into_any()
}

#[variant]
fn body_text() -> AnyView {
    view! {
        <div class="space-y-4">
            <P>"This is a paragraph. The quick brown fox jumps over the lazy dog."</P>
            <Lead>"This is lead text."</Lead>
            <Large>"This is large text."</Large>
            <Small>"This is small text."</Small>
            <Muted>"This is muted text."</Muted>
        </div>
    }
    .into_any()
}

#[variant]
fn blockquote() -> AnyView {
    view! { <Blockquote>"After all, everyone enjoys a good quote now and then."</Blockquote> }
        .into_any()
}

include!(concat!(env!("OUT_DIR"), "/stories/typography_source.rs"));

#[story(id = "typography", name = "Typography", extra_docs = TYPOGRAPHY_SOURCE)]
/// Typography components for consistent text styling
const TYPOGRAPHY_STORY: () = &[headings, body_text, blockquote];
