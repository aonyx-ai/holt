use holt_book::{story, variant};
use leptos::prelude::*;

#[variant]
fn green() -> AnyView {
    view! { <div style="width:100px;height:100px;background:green"></div> }.into_any()
}

#[variant]
fn yellow() -> AnyView {
    view! { <div style="width:100px;height:100px;background:yellow"></div> }.into_any()
}

#[story(id = "square-b", name = "Square B")]
const SQUARE_B: () = &[green, yellow];
