use leptos::prelude::AnyView;

pub trait Story {
    fn new() -> Self;
    fn title(&self) -> &str;
    fn into_view(&self) -> AnyView;
}
