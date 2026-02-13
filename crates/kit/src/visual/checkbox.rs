use leptos::prelude::*;
use leptos_icons::Icon;
use tailwind_fuse::*;

use crate::behavior::{CheckboxIndicator, CheckboxRoot};

#[derive(TwClass)]
#[tw(
    class = "peer border-input dark:bg-input/30 data-[state=checked]:bg-primary data-[state=checked]:text-primary-foreground dark:data-[state=checked]:bg-primary data-[state=checked]:border-primary focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive size-4 shrink-0 rounded-[4px] border shadow-xs transition-shadow outline-none focus-visible:ring-[3px] disabled:cursor-not-allowed disabled:opacity-50"
)]
struct CheckboxStyle {
    size: CheckboxSize,
}

#[derive(TwVariant)]
pub enum CheckboxSize {
    #[tw(default, class = "size-4")]
    Default,
    #[tw(class = "size-3")]
    Sm,
    #[tw(class = "size-5")]
    Lg,
}

#[component]
pub fn Checkbox(
    #[prop(optional, into)] class: String,
    #[prop(optional)] size: CheckboxSize,
    #[prop(optional)] checked: RwSignal<bool>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional_no_strip, into)] id: Option<&'static str>,
    #[prop(optional_no_strip, into)] name: Option<&'static str>,
    #[prop(optional_no_strip)] on_change: Option<Callback<bool>>,
) -> impl IntoView {
    let final_class = CheckboxStyle { size }.with_class(&class);

    view! {
        <CheckboxRoot
            checked=checked
            disabled=disabled
            id=id
            name=name
            class=final_class
            on_change=on_change
        >
            <CheckboxIndicator class="flex items-center justify-center text-current transition-none">
                <Icon
                    icon=icondata::LuCheck
                    attr:class=match size {
                        CheckboxSize::Sm => "size-2.5",
                        CheckboxSize::Default => "size-3.5",
                        CheckboxSize::Lg => "size-4",
                    }
                />
            </CheckboxIndicator>
        </CheckboxRoot>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn class_prop_accepts_str_and_string() {
        assert_class_prop!(CheckboxProps);
    }
}
