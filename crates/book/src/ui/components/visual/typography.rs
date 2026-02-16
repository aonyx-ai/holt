use leptos::children::Children;
use leptos::prelude::*;
use tailwind_fuse::tw_merge;

#[component]
pub fn H1(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let classes = tw_merge!(
        "scroll-m-20 text-4xl font-extrabold tracking-tight lg:text-5xl",
        &class
    );

    view! { <h1 class=classes>{children()}</h1> }
}

#[component]
pub fn H2(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let classes = tw_merge!(
        "mt-10 scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight first:mt-0",
        &class
    );

    view! { <h2 class=classes>{children()}</h2> }
}
