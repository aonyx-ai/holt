use leptos::prelude::*;
use leptos::children::Children;

/// The main breadcrumb container
#[component]
pub fn Breadcrumb(
    #[prop(optional)] class: &'static str,
    children: Children,
) -> impl IntoView {
    let classes = move || {
        if class.is_empty() {
            "flex".to_string()
        } else {
            format!("flex {}", class)
        }
    };

    view! {
        <nav class=classes aria-label="breadcrumb">
            {children()}
        </nav>
    }
}

/// Contains all breadcrumb items
#[component]
pub fn BreadcrumbList(
    #[prop(optional)] class: &'static str,
    children: Children,
) -> impl IntoView {
    let classes = move || {
        if class.is_empty() {
            "flex flex-wrap items-center gap-1 overflow-hidden text-sm text-muted-foreground".to_string()
        } else {
            format!("flex flex-wrap items-center gap-1 overflow-hidden text-sm text-muted-foreground {}", class)
        }
    };

    view! {
        <ol class=classes>
            {children()}
        </ol>
    }
}

/// A single breadcrumb item
#[component]
pub fn BreadcrumbItem(
    #[prop(optional)] class: &'static str,
    children: Children,
) -> impl IntoView {
    let classes = move || {
        if class.is_empty() {
            "inline-flex items-center gap-1".to_string()
        } else {
            format!("inline-flex items-center gap-1 {}", class)
        }
    };

    view! {
        <li class=classes>
            {children()}
        </li>
    }
}

/// A link-styled breadcrumb item
#[component]
pub fn BreadcrumbLink(
    #[prop(optional)] class: &'static str,
    #[prop(optional)] href: &'static str,
    children: Children,
) -> impl IntoView {
    let classes = move || {
        if class.is_empty() {
            "transition-colors hover:text-foreground".to_string()
        } else {
            format!("transition-colors hover:text-foreground {}", class)
        }
    };

    view! {
        <a class=classes href=href>
            {children()}
        </a>
    }
}

/// The current/active breadcrumb item
#[component]
pub fn BreadcrumbPage(
    #[prop(optional)] class: &'static str,
    children: Children,
) -> impl IntoView {
    let classes = move || {
        if class.is_empty() {
            "font-medium text-foreground".to_string()
        } else {
            format!("font-medium text-foreground {}", class)
        }
    };

    view! {
        <span class=classes aria-current="page">
            {children()}
        </span>
    }
}

/// The separator between breadcrumb items
#[component]
pub fn BreadcrumbSeparator(
    #[prop(optional)] class: &'static str,
) -> impl IntoView {
    let classes = move || {
        if class.is_empty() {
            "mx-1 text-muted-foreground".to_string()
        } else {
            format!("mx-1 text-muted-foreground {}", class)
        }
    };

    view! {
        <li class=classes>
            "/"
        </li>
    }
}
