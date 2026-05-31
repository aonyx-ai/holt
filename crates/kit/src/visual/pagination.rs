use leptos::prelude::*;
use leptos_icons::Icon;
use tailwind_fuse::*;

/// Navigation container for paginated content.
#[component]
pub fn Pagination(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let classes = tw_merge!("mx-auto flex w-full justify-center", &class);

    view! {
        <nav class=classes role="navigation" aria-label="pagination">
            {children()}
        </nav>
    }
}

/// Flex container for pagination items.
#[component]
pub fn PaginationContent(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let classes = tw_merge!("flex flex-row items-center gap-1", &class);

    view! { <ul class=classes>{children()}</ul> }
}

/// List item wrapper for a single pagination element.
#[component]
pub fn PaginationItem(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <li class=class>{children()}</li> }
}

#[derive(TwClass)]
#[tw(
    class = "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50"
)]
struct PaginationLinkStyle {
    variant: PaginationLinkVariant,
    size: PaginationLinkSize,
}

#[derive(TwVariant)]
enum PaginationLinkVariant {
    #[tw(default, class = "hover:bg-accent hover:text-accent-foreground")]
    Ghost,
    #[tw(class = "border border-input bg-background hover:bg-accent hover:text-accent-foreground")]
    Outline,
}

#[derive(TwVariant)]
enum PaginationLinkSize {
    #[tw(default, class = "h-10 w-10")]
    Default,
}

/// Clickable page number button. When `is_active` is true, renders with an
/// outline style to indicate the current page.
#[component]
pub fn PaginationLink(
    #[prop(optional, into)] class: String,
    #[prop(optional)] is_active: bool,
    #[prop(optional, into)] href: String,
    children: Children,
) -> impl IntoView {
    let variant = if is_active {
        PaginationLinkVariant::Outline
    } else {
        PaginationLinkVariant::Ghost
    };
    let final_class = PaginationLinkStyle {
        variant,
        size: PaginationLinkSize::Default,
    }
    .with_class(class);

    view! {
        <a class=final_class href=href aria-current=is_active.then_some("page")>
            {children()}
        </a>
    }
}

/// Previous page navigation button with a left chevron icon.
#[component]
pub fn PaginationPrevious(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] href: String,
) -> impl IntoView {
    let final_class = PaginationLinkStyle {
        variant: PaginationLinkVariant::Ghost,
        size: PaginationLinkSize::Default,
    }
    .with_class(tw_merge!("w-auto gap-1 pl-2.5 pr-4", &class));

    view! {
        <a class=final_class href=href aria-label="Go to previous page">
            <Icon icon=icondata::LuChevronLeft attr:class="h-4 w-4" />
            <span>Previous</span>
        </a>
    }
}

/// Next page navigation button with a right chevron icon.
#[component]
pub fn PaginationNext(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] href: String,
) -> impl IntoView {
    let final_class = PaginationLinkStyle {
        variant: PaginationLinkVariant::Ghost,
        size: PaginationLinkSize::Default,
    }
    .with_class(tw_merge!("w-auto gap-1 pl-4 pr-2.5", &class));

    view! {
        <a class=final_class href=href aria-label="Go to next page">
            <span>Next</span>
            <Icon icon=icondata::LuChevronRight attr:class="h-4 w-4" />
        </a>
    }
}

/// Ellipsis indicator showing that pages have been omitted.
#[component]
pub fn PaginationEllipsis(#[prop(optional, into)] class: String) -> impl IntoView {
    let classes = tw_merge!("flex h-10 w-10 items-center justify-center", &class);

    view! {
        <span class=classes aria-hidden="true">
            <Icon icon=icondata::LuEllipsis attr:class="h-4 w-4" />
            <span class="sr-only">More pages</span>
        </span>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn class_prop_accepts_str_and_string() {
        assert_class_prop!(
            PaginationProps,
            PaginationContentProps,
            PaginationItemProps,
            PaginationLinkProps,
            PaginationPreviousProps,
            PaginationNextProps,
            PaginationEllipsisProps,
        );
    }
}
