use leptos::prelude::*;

pub struct StoryVariant {
    pub name: &'static str,
    pub view: fn() -> AnyView,
}

pub struct Story {
    pub id: &'static str,
    pub name: &'static str,
    pub description: Option<&'static str>,
    pub variants: &'static [&'static StoryVariant],
}

inventory::collect!(&'static Story);

unsafe extern "C" {
    fn __wasm_call_ctors();
}

pub fn init_story_registry() {
    #[cfg(target_family = "wasm")]
    unsafe {
        __wasm_call_ctors();
    }
}

#[cfg(test)]
mod test {
    use crate::Story;
    use crate::StoryVariant;

    #[test]
    fn ensure_story_send_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Story>();

        fn assert_send<T: Send>() {}
        assert_send::<Story>();
    }

    #[test]
    fn ensure_send_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<StoryVariant>();

        fn assert_send<T: Send>() {}
        assert_send::<StoryVariant>();
    }
}
