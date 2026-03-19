use leptos::prelude::*;
use tailwind_fuse::*;

#[derive(TwClass)]
#[tw(class = "relative flex shrink-0 overflow-hidden rounded-full")]
struct AvatarStyle {
    size: AvatarSize,
}

#[derive(TwVariant)]
pub enum AvatarSize {
    #[tw(class = "h-8 w-8 text-xs")]
    Sm,
    #[tw(default, class = "h-10 w-10 text-sm")]
    Default,
    #[tw(class = "h-12 w-12 text-base")]
    Lg,
}

#[component]
pub fn Avatar(
    #[prop(optional, into)] class: String,
    #[prop(optional)] size: AvatarSize,
    children: Children,
) -> impl IntoView {
    let final_class = AvatarStyle { size }.with_class(class);
    view! { <span class=final_class>{children()}</span> }
}

#[component]
pub fn AvatarImage(
    #[prop(optional, into)] class: String,
    #[prop(into)] src: String,
    #[prop(optional, into)] alt: String,
) -> impl IntoView {
    let classes = tw_merge!("aspect-square h-full w-full", &class);
    view! { <img class=classes src=src alt=alt /> }
}

#[component]
pub fn AvatarFallback(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let classes = tw_merge!(
        "flex h-full w-full items-center justify-center rounded-full bg-muted",
        &class
    );
    view! { <span class=classes>{children()}</span> }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn class_prop_accepts_str_and_string() {
        assert_class_prop!(AvatarProps, AvatarImageProps, AvatarFallbackProps,);
    }
}
