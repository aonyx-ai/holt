use leptos::prelude::*;

#[derive(bon::Builder, getset::CopyGetters)]
#[builder(const)]
pub struct StoryVariant {
    #[getset(get_copy = "pub")]
    name: &'static str,
    #[getset(get_copy = "pub")]
    render: fn() -> AnyView,
    #[getset(get_copy = "pub")]
    source: &'static str,
}

#[derive(bon::Builder, getset::CopyGetters)]
#[builder(const)]
pub struct Story {
    #[getset(get_copy = "pub")]
    id: &'static str,
    #[getset(get_copy = "pub")]
    name: &'static str,
    #[getset(get_copy = "pub")]
    description: Option<&'static str>,
    #[getset(get_copy = "pub")]
    variants: &'static [&'static StoryVariant],
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
mod tests {
    use super::*;

    #[test]
    fn ensure_send_sync_is_send() {
        fn assert_send<T: Send>() {}

        assert_send::<StoryVariant>();
    }

    #[test]
    fn ensure_send_sync_is_sync() {
        fn assert_sync<T: Sync>() {}

        assert_sync::<StoryVariant>();
    }

    #[test]
    fn ensure_story_send_sync_is_send() {
        fn assert_send<T: Send>() {}

        assert_send::<Story>();
    }

    #[test]
    fn ensure_story_send_sync_is_sync() {
        fn assert_sync<T: Sync>() {}

        assert_sync::<Story>();
    }

    #[test]
    fn story_is_send_sync_and_unpin() {
        fn assert_send_sync_and_unpin<T: Send + Sync + Unpin>() {}

        assert_send_sync_and_unpin::<Story>();
    }

    #[test]
    fn story_variant_is_send_sync_and_unpin() {
        fn assert_send_sync_and_unpin<T: Send + Sync + Unpin>() {}

        assert_send_sync_and_unpin::<StoryVariant>();
    }
}
