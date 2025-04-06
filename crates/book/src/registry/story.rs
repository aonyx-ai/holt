use leptos::prelude::AnyView;

pub trait Story {
    fn new() -> Self;
    // fn title(&self) -> &str;
    fn as_view(&self) -> AnyView;
}
