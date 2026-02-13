use leptos::children::Children;
use leptos::prelude::*;
use tailwind_fuse::tw_merge;

#[component]
pub fn Card(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let classes = tw_merge!(
        "rounded-lg border bg-card text-card-foreground shadow-sm",
        &class
    );

    view! { <div class=classes>{children()}</div> }
}

#[component]
pub fn CardHeader(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let classes = tw_merge!("flex flex-col space-y-1.5 p-6", &class);

    view! { <div class=classes>{children()}</div> }
}

#[component]
pub fn CardTitle(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let classes = tw_merge!("text-2xl font-semibold leading-none tracking-tight", &class);

    view! { <h3 class=classes>{children()}</h3> }
}

#[component]
pub fn CardDescription(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let classes = tw_merge!("text-sm text-muted-foreground", &class);

    view! { <p class=classes>{children()}</p> }
}

#[component]
pub fn CardContent(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let classes = tw_merge!("p-6 pt-0", &class);

    view! { <div class=classes>{children()}</div> }
}

#[component]
pub fn CardFooter(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let classes = tw_merge!("flex items-center p-6 pt-0", &class);

    view! { <div class=classes>{children()}</div> }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn class_prop_accepts_str_and_string() {
        assert_class_prop!(
            CardProps,
            CardHeaderProps,
            CardTitleProps,
            CardDescriptionProps,
            CardContentProps,
            CardFooterProps,
        );
    }
}
