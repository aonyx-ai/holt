use leptos::prelude::*;
use tailwind_fuse::*;

#[derive(TwClass)]
#[tw(
    class = "inline-flex items-center justify-center rounded-md border px-2 py-0.5 text-xs font-medium w-fit whitespace-nowrap shrink-0 [&>svg]:size-3 gap-1 [&>svg]:pointer-events-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive transition-[color,box-shadow] overflow-hidden"
)]
struct BadgeStyle {
    variant: BadgeVariant,
}

#[derive(TwVariant)]
pub enum BadgeVariant {
    #[tw(
        default,
        class = "border-transparent bg-primary text-primary-foreground [a&]:hover:bg-primary/90"
    )]
    Default,
    #[tw(
        class = "border-transparent bg-secondary text-secondary-foreground [a&]:hover:bg-secondary/90"
    )]
    Secondary,
    #[tw(
        class = "border-transparent bg-destructive text-white [a&]:hover:bg-destructive/90 focus-visible:ring-destructive/20 dark:focus-visible:ring-destructive/40 dark:bg-destructive/60"
    )]
    Destructive,
    #[tw(class = "text-foreground [a&]:hover:bg-accent [a&]:hover:text-accent-foreground")]
    Outline,
}

#[component]
pub fn Badge(
    #[prop(optional)] class: &'static str,
    #[prop(optional)] variant: BadgeVariant,
    // TODO: add support for behaviour like @radix-ui/react-slot?
    // #[prop(optional)] as_child: bool,
    children: Children,
) -> impl IntoView {
    let final_class = BadgeStyle { variant }.with_class(class);
    view! { <span class=final_class>{children()}</span> }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test(unsupported = test)]
    #[cfg_attr(not(target_family = "wasm"), ignore)]
    fn badge_renders_as_span() {
        let document = web_sys::window().unwrap().document().unwrap();
        let container = document.create_element("div").unwrap();
        document.body().unwrap().append_child(&container).unwrap();

        let _owner = Owner::new();
        _owner.with(|| {
            let view = view! { <Badge>"Test"</Badge> };
            let mut mounted = view.build();
            use leptos::tachys::view::Mountable;
            mounted.mount(&container, None);
        });

        let first_child = container
            .first_element_child()
            .expect("Badge should render an element");

        assert_eq!(
            first_child.tag_name().to_lowercase(),
            "span",
            "Badge root element should be a <span>"
        );

        document.body().unwrap().remove_child(&container).unwrap();
    }
}
