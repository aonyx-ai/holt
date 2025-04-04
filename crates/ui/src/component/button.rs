use leptos::children::Children;
use leptos::web_sys::MouseEvent;
use leptos::prelude::*;
use leptos::html;
use tailwind_fuse::*;

#[derive(TwClass)]
#[tw(
    class = "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50"
)]
struct ButtonStyle {
    variant: ButtonVariant,
    size: ButtonSize,
}

#[derive(TwVariant)]
pub enum ButtonVariant {
    #[tw(
        default,
        class = "bg-primary text-primary-foreground hover:bg-primary/90"
    )]
    Default,
    #[tw(class = "bg-destructive text-destructive-foreground hover:bg-destructive/90")]
    Destructive,
    #[tw(class = "border border-input bg-background hover:bg-accent hover:text-accent-foreground")]
    Outline,
    #[tw(class = "bg-secondary text-secondary-foreground hover:bg-secondary/80")]
    Secondary,
    #[tw(class = "hover:bg-accent hover:text-accent-foreground")]
    Ghost,
    #[tw(class = "text-primary underline-offset-4 hover:underline")]
    Link,
}

#[derive(TwVariant)]
pub enum ButtonSize {
    #[tw(default, class = "h-10 px-4 py-2")]
    Default,
    #[tw(class = "h-8 px-3 py-1")]
    Sm,
    #[tw(class = "h-12 px-6 py-3")]
    Lg,
    #[tw(class = "h-10 w-10 p-2")]
    Icon,
}

#[component]
pub fn Button(
    #[prop(optional)] class: &'static str,
    #[prop(optional)] variant: ButtonVariant,
    #[prop(optional)] size: ButtonSize,
    // TODO: add support for behaviour like @radix-ui/react-slot?
    // #[prop(optional)] as_child: bool,
    children: Children,
) -> impl IntoView {
    let final_class = ButtonStyle { variant, size }.with_class(class);
    let element: NodeRef<html::Button> = NodeRef::new();

    let on_click = move |e: MouseEvent| {
        element.get().map(|el| el.dispatch_event(&e));
    };

    view! { <button on:click=on_click node_ref=element class=final_class>{children()}</button> }
}
