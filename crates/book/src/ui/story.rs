use leptos::{prelude::*, view};

pub trait StoryAsView {
    fn as_view(&self) -> AnyView;
}

pub struct StoryVariant {
    pub name: &'static str,
    pub view: fn() -> AnyView,
}

pub struct Story {
    pub id: &'static str,
    pub name: &'static str,
    pub variants: &'static [&'static StoryVariant],
}

impl StoryAsView for Story {
    fn as_view(&self) -> AnyView {
        let (selected_variant, set_selected_variant) = signal(0);

        let variants = self.variants;

        view! {
            <div>
                <h1>{self.name}</h1>
                <div>
                    <select on:change=move |ev| {
                        let value = event_target_value(&ev);
                        if let Ok(index) = value.parse::<usize>() {
                            set_selected_variant.set(index);
                        }
                    }>
                        {variants.iter().enumerate().map(|(i, variant)| {
                            view! {
                                <option value=i.to_string() selected=move || selected_variant.get() == i>
                                    {variant.name}
                                </option>
                            }
                        }).collect::<Vec<_>>()}
                    </select>
                </div>
                <div>
                    {move || {
                        let index = selected_variant.get();
                        if let Some(variant) = variants.get(index) {
                            (variant.view)()
                        } else {
                            view! { <div>"No variant selected"</div> }.into_any()
                        }
                    }}
                </div>
            </div>
        }
        .into_any()
    }
}

inventory::collect!(Story);

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
