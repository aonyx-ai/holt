use leptos::children::Children;
use leptos::prelude::*;
use tailwind_fuse::*;

/// The main breadcrumb container
#[component]
pub fn Breadcrumb(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let classes = tw_merge!("flex", &class);

    view! {
        <nav class=classes aria-label="breadcrumb">
            {children()}
        </nav>
    }
}

/// Contains all breadcrumb items
#[component]
pub fn BreadcrumbList(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let classes = tw_merge!(
        "flex flex-wrap items-center gap-1 overflow-hidden text-sm text-muted-foreground",
        &class
    );

    view! { <ol class=classes>{children()}</ol> }
}

/// A single breadcrumb item
#[component]
pub fn BreadcrumbItem(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let classes = tw_merge!("inline-flex items-center gap-1", &class);

    view! { <li class=classes>{children()}</li> }
}

/// A link-styled breadcrumb item
#[component]
pub fn BreadcrumbLink(
    #[prop(optional, into)] class: String,
    #[prop(optional)] href: &'static str,
    children: Children,
) -> impl IntoView {
    let classes = tw_merge!("transition-colors hover:text-foreground", &class);

    view! {
        <a class=classes href=href>
            {children()}
        </a>
    }
}

/// The current/active breadcrumb item
#[component]
pub fn BreadcrumbPage(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let classes = tw_merge!("font-medium text-foreground", &class);

    view! {
        <span class=classes aria-current="page">
            {children()}
        </span>
    }
}

/// The separator between breadcrumb items
#[component]
pub fn BreadcrumbSeparator(#[prop(optional, into)] class: String) -> impl IntoView {
    let classes = tw_merge!("mx-1 text-muted-foreground", &class);

    view! { <li class=classes>"/"</li> }
}
