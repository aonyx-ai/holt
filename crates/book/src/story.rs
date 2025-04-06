use leptos::prelude::AnyView;

pub trait Story: Send + Sync {
    fn new() -> Self
    where
        Self: Sized;
    fn title(&self) -> &str;
    fn as_view(&self) -> AnyView;
}

inventory::collect!(&'static dyn Story);

macro_rules! build_story {
    ($name:ident, $title:expr, $view:expr) => {
        #[derive(Debug, Clone, Copy)]
        struct $name;

        impl crate::story::Story for $name {
            fn new() -> Self
            where
                Self: Sized,
            {
                $name
            }

            fn title(&self) -> &str {
                $title
            }

            fn as_view(&self) -> AnyView {
                leptos::prelude::IntoAny::into_any($view)
            }
        }

        inventory::submit!(&$name as &dyn crate::story::Story);
    };
}

pub(crate) use build_story;
