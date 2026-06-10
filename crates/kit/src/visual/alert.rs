use leptos::prelude::*;
use tailwind_fuse::*;

#[derive(TwClass)]
#[tw(
    class = "relative w-full rounded-lg border px-4 py-3 text-sm grid has-[>svg]:grid-cols-[calc(var(--spacing)*4)_1fr] grid-cols-[0_1fr] has-[>svg]:gap-x-3 gap-y-0.5 items-start [&>svg]:size-4 [&>svg]:translate-y-0.5 [&>svg]:text-current"
)]
struct AlertStyle {
    variant: AlertVariant,
}

#[derive(TwVariant)]
pub enum AlertVariant {
    #[tw(default, class = "bg-card text-foreground [&>svg]:text-current")]
    Default,
    #[tw(
        class = "text-destructive bg-card [&>svg]:text-current *:data-[slot=alert-description]:text-destructive/90"
    )]
    Destructive,
}

#[component]
pub fn Alert(
    #[prop(optional, into)] class: String,
    #[prop(optional)] variant: AlertVariant,
    children: Children,
) -> impl IntoView {
    let final_class = AlertStyle { variant }.with_class(class);
    view! {
        <div class=final_class role="alert">
            {children()}
        </div>
    }
}

#[component]
pub fn AlertTitle(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let classes = tw_merge!(
        "col-start-2 font-medium leading-none tracking-tight",
        &class
    );
    view! {
        <h5 class=classes data-slot="alert-title">
            {children()}
        </h5>
    }
}

#[component]
pub fn AlertDescription(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let classes = tw_merge!(
        "col-start-2 text-sm text-muted-foreground [&_p]:leading-relaxed",
        &class
    );
    view! {
        <div class=classes data-slot="alert-description">
            {children()}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn class_prop_accepts_str_and_string() {
        assert_class_prop!(AlertProps, AlertTitleProps, AlertDescriptionProps);
    }
}
