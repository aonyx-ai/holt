use leptos::children::Children;
use leptos::prelude::*;
use tailwind_fuse::tw_merge;

/// A label component that renders an accessible label associated with form controls
#[component]
pub fn Label(
    #[prop(optional, into)] class: String,
    #[prop(optional_no_strip, into)] r#for: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    let classes = tw_merge!(
        "text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70 select-none",
        &class
    );

    view! {
        <label class=classes r#for=r#for>
            {children()}
        </label>
    }
}
