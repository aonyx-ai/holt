use leptos::*;
use tailwind_fuse::tw_merge;

#[component]
pub fn Card(#[prop(optional)] class: &'static str, children: Children) -> impl IntoView {
    let classes = tw_merge!(
        "rounded-lg border bg-card text-card-foreground shadow-sm",
        class
    );

    view! { <div class=classes>{children()}</div> }
}

#[component]
pub fn CardHeader(#[prop(optional)] class: &'static str, children: Children) -> impl IntoView {
    let classes = tw_merge!("flex flex-col space-y-1.5 p-6", class);

    view! { <div class=classes>{children()}</div> }
}

#[component]
pub fn CardTitle(#[prop(optional)] class: &'static str, children: Children) -> impl IntoView {
    let classes = tw_merge!("text-2xl font-semibold leading-none tracking-tight", class);

    view! { <h3 class=classes>{children()}</h3> }
}

#[component]
pub fn CardDescription(#[prop(optional)] class: &'static str, children: Children) -> impl IntoView {
    let classes = tw_merge!("text-sm text-muted-foreground", class);

    view! { <p class=classes>{children()}</p> }
}

#[component]
pub fn CardContent(#[prop(optional)] class: &'static str, children: Children) -> impl IntoView {
    let classes = tw_merge!("p-6 pt-0", class);

    view! { <div class=classes>{children()}</div> }
}

#[component]
pub fn CardFooter(#[prop(optional)] class: &'static str, children: Children) -> impl IntoView {
    let classes = tw_merge!("flex items-center p-6 pt-0", class);

    view! { <div class=classes>{children()}</div> }
}
