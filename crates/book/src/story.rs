use leptos::prelude::AnyView;

pub trait StoryNew {
    fn new() -> Self
    where
        Self: Sized;
}

pub trait StoryAsView: Send + Sync {
    fn as_view(&self) -> AnyView;
}

pub trait StoryTitle: Send + Sync {
    fn title() -> &'static str;
}

pub trait Story: Send + Sync {
    fn new() -> Self
    where
        Self: Sized;
    fn title(&self) -> &str;
    fn as_view(&self) -> AnyView;
}

impl<T> Story for T
where
    T: StoryNew + StoryTitle + StoryAsView,
{
    fn new() -> Self
    where
        Self: Sized,
    {
        T::new()
    }

    #[inline(always)]
    fn title(&self) -> &str {
        T::title()
    }

    fn as_view(&self) -> AnyView {
        T::as_view(self)
    }
}

inventory::collect!(&'static dyn Story);

macro_rules! register_story {
    ($name:ident, $title:expr) => {
        impl crate::story::StoryNew for $name {
            fn new() -> Self
            where
                Self: Sized,
            {
                $name
            }
        }

        impl crate::story::StoryTitle for $name {
            fn title() -> &'static str {
                $title
            }
        }

        inventory::submit!(&$name as &dyn crate::story::Story);
    };
}

pub(crate) use register_story;

extern "C" {
    fn __wasm_call_ctors();
}

pub fn init_story_registry() {
    unsafe {
        __wasm_call_ctors();
    }
}
