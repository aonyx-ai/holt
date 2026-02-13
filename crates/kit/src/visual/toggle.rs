use leptos::children::Children;
use leptos::prelude::*;
use tailwind_fuse::*;

use crate::behavior::ToggleRoot;

#[derive(TwClass)]
#[tw(
    class = "inline-flex items-center justify-center gap-2 rounded-md text-sm font-medium ring-offset-background transition-colors hover:bg-muted hover:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 data-[state=on]:bg-accent data-[state=on]:text-accent-foreground [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0"
)]
struct ToggleStyle {
    variant: ToggleVariant,
    size: ToggleSize,
}

#[derive(TwVariant)]
pub enum ToggleVariant {
    #[tw(default, class = "bg-transparent")]
    Default,
    #[tw(class = "border border-input bg-transparent hover:bg-accent hover:text-accent-foreground")]
    Outline,
}

#[derive(TwVariant)]
pub enum ToggleSize {
    #[tw(default, class = "h-10 px-3 min-w-10")]
    Default,
    #[tw(class = "h-9 px-2.5 min-w-9")]
    Sm,
    #[tw(class = "h-11 px-5 min-w-11")]
    Lg,
}

#[component]
pub fn Toggle(
    #[prop(optional)] class: &'static str,
    #[prop(optional)] variant: ToggleVariant,
    #[prop(optional)] size: ToggleSize,
    #[prop(optional)] pressed: RwSignal<bool>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional_no_strip, into)] aria_label: Option<&'static str>,
    #[prop(optional_no_strip)] on_change: Option<Callback<bool>>,
    children: Children,
) -> impl IntoView {
    let final_class = ToggleStyle { variant, size }.with_class(class);

    view! {
        <ToggleRoot
            pressed=pressed
            disabled=disabled
            aria_label=aria_label
            class=final_class
            on_change=on_change
        >
            {children()}
        </ToggleRoot>
    }
}
