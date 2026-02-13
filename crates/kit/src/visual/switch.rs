use leptos::prelude::*;
use tailwind_fuse::*;

use crate::behavior::{SwitchRoot, SwitchThumb};

#[derive(TwClass)]
#[tw(
    class = "inline-flex shrink-0 cursor-pointer items-center rounded-full border-2 border-transparent transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:ring-offset-background disabled:cursor-not-allowed disabled:opacity-50 data-[state=checked]:bg-primary data-[state=unchecked]:bg-input"
)]
struct SwitchRootStyle {
    size: SwitchSize,
}

#[derive(TwClass)]
#[tw(
    class = "pointer-events-none block rounded-full bg-background shadow-lg ring-0 transition-transform data-[state=unchecked]:translate-x-0"
)]
struct SwitchThumbStyle {
    size: SwitchThumbSize,
}

#[derive(TwVariant)]
pub enum SwitchSize {
    #[tw(default, class = "h-6 w-11")]
    Default,
    #[tw(class = "h-5 w-9")]
    Sm,
    #[tw(class = "h-7 w-14")]
    Lg,
}

#[derive(TwVariant)]
enum SwitchThumbSize {
    #[tw(default, class = "h-5 w-5 data-[state=checked]:translate-x-5")]
    Default,
    #[tw(class = "h-4 w-4 data-[state=checked]:translate-x-4")]
    Sm,
    #[tw(class = "h-6 w-6 data-[state=checked]:translate-x-7")]
    Lg,
}

/// A styled switch component for toggling between checked and unchecked states
#[component]
pub fn Switch(
    #[prop(optional, into)] class: String,
    #[prop(optional)] size: SwitchSize,
    #[prop(optional)] checked: RwSignal<bool>,
    #[prop(into, default = Signal::stored(false))] disabled: Signal<bool>,
    #[prop(optional_no_strip, into)] id: Option<&'static str>,
    #[prop(optional_no_strip, into)] name: Option<&'static str>,
    #[prop(optional_no_strip)] on_change: Option<Callback<bool>>,
) -> impl IntoView {
    let final_class = SwitchRootStyle { size }.with_class(&class);

    let thumb_size = match size {
        SwitchSize::Sm => SwitchThumbSize::Sm,
        SwitchSize::Default => SwitchThumbSize::Default,
        SwitchSize::Lg => SwitchThumbSize::Lg,
    };

    let thumb_classes = SwitchThumbStyle { size: thumb_size }.to_class();

    view! {
        <SwitchRoot
            checked=checked
            disabled=disabled
            id=id
            name=name
            class=final_class
            on_change=on_change
        >
            <SwitchThumb class=thumb_classes />
        </SwitchRoot>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn class_prop_accepts_str_and_string() {
        assert_class_prop!(SwitchProps);
    }
}
