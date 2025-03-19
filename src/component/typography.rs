use leptos::prelude::*;
use tailwind_fuse::tw_merge;

#[component]
pub fn H1(#[prop(optional)] class: &'static str, children: Children) -> impl IntoView {
    let classes = tw_merge!(
        "scroll-m-20 text-4xl font-extrabold tracking-tight lg:text-5xl",
        class
    );

    view! { <h1 class=classes>{children()}</h1> }
}

#[component]
pub fn H2(#[prop(optional)] class: &'static str, children: Children) -> impl IntoView {
    let classes = tw_merge!(
        "mt-10 scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight first:mt-0",
        class
    );

    view! { <h2 class=classes>{children()}</h2> }
}

#[component]
pub fn H3(#[prop(optional)] class: &'static str, children: Children) -> impl IntoView {
    let classes = tw_merge!(
        "mt-8 scroll-m-20 text-2xl font-semibold tracking-tight",
        class
    );

    view! { <h3 class=classes>{children()}</h3> }
}

#[component]
pub fn H4(#[prop(optional)] class: &'static str, children: Children) -> impl IntoView {
    let classes = tw_merge!("scroll-m-20 text-xl font-semibold tracking-tight", class);

    view! { <h4 class=classes>{children()}</h4> }
}

#[component]
pub fn P(#[prop(optional)] class: &'static str, children: Children) -> impl IntoView {
    let classes = tw_merge!("leading-7 [&:not(:first-child)]:mt-6", class);

    view! { <p class=classes>{children()}</p> }
}

#[component]
pub fn Blockquote(#[prop(optional)] class: &'static str, children: Children) -> impl IntoView {
    let classes = tw_merge!("mt-6 border-l-2 pl-6 italic", class);

    view! { <blockquote class=classes>{children()}</blockquote> }
}

#[component]
pub fn Lead(#[prop(optional)] class: &'static str, children: Children) -> impl IntoView {
    let classes = tw_merge!("text-xl text-muted-foreground", class);

    view! { <p class=classes>{children()}</p> }
}

#[component]
pub fn Large(#[prop(optional)] class: &'static str, children: Children) -> impl IntoView {
    let classes = tw_merge!("text-lg font-semibold", class);

    view! { <div class=classes>{children()}</div> }
}

#[component]
pub fn Small(#[prop(optional)] class: &'static str, children: Children) -> impl IntoView {
    let classes = tw_merge!("text-sm font-medium leading-none", class);

    view! { <small class=classes>{children()}</small> }
}

#[component]
pub fn Muted(#[prop(optional)] class: &'static str, children: Children) -> impl IntoView {
    let classes = tw_merge!("text-sm text-muted-foreground", class);

    view! { <p class=classes>{children()}</p> }
}
