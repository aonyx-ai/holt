use holt_book::{story, variant};
use leptos::prelude::*;

#[variant]
fn red() -> AnyView {
    view! { <div style="width:100px;height:100px;background:red"></div> }.into_any()
}

#[variant]
fn blue() -> AnyView {
    view! { <div style="width:100px;height:100px;background:blue"></div> }.into_any()
}

#[story(id = "square-a", name = "Square A")]
const SQUARE_A: () = &[red, blue];
