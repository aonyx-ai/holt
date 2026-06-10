use leptos::prelude::*;
use tailwind_fuse::*;

use crate::behavior::{RadioGroupIndicator, RadioGroupItemRoot, RadioGroupRoot};

#[derive(TwClass)]
#[tw(class = "grid gap-3")]
struct RadioGroupStyle {}

#[derive(TwClass)]
#[tw(
    class = "border-input text-primary focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:bg-input/30 aspect-square size-4 shrink-0 rounded-full border shadow-xs transition-shadow outline-none focus-visible:ring-[3px] disabled:cursor-not-allowed disabled:opacity-50"
)]
struct RadioGroupItemStyle {}

/// A styled radio group component for selecting one option from a set
#[component]
pub fn RadioGroup(
    #[prop(optional, into)] class: String,
    #[prop(optional)] value: RwSignal<Option<String>>,
    #[prop(into, default = Signal::stored(false))] disabled: Signal<bool>,
    #[prop(optional_no_strip)] on_change: Option<Callback<String>>,
    children: Children,
) -> impl IntoView {
    let final_class = RadioGroupStyle {}.with_class(&class);

    view! {
        <RadioGroupRoot value=value disabled=disabled class=final_class on_change=on_change>
            {children()}
        </RadioGroupRoot>
    }
}

/// A styled radio group item
#[component]
pub fn RadioGroupItem(
    #[prop(optional, into)] class: String,
    #[prop(into)] value: String,
    #[prop(into, default = Signal::stored(false))] disabled: Signal<bool>,
) -> impl IntoView {
    let final_class = RadioGroupItemStyle {}.with_class(&class);
    let indicator_value = value.clone();

    view! {
        <RadioGroupItemRoot value=value disabled=disabled class=final_class>
            <RadioGroupIndicator value=indicator_value class="flex items-center justify-center">
                <span class="bg-primary size-2 rounded-full" />
            </RadioGroupIndicator>
        </RadioGroupItemRoot>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn class_prop_accepts_str_and_string() {
        assert_class_prop!(RadioGroupProps, RadioGroupItemProps);
    }
}
