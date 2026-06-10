use leptos::children::Children;
use leptos::prelude::*;
use tailwind_fuse::*;

use super::toggle::{ToggleSize, ToggleVariant};
use crate::behavior::{ToggleGroupRoot, ToggleGroupType};

#[derive(TwClass)]
#[tw(class = "flex items-center gap-1")]
struct ToggleGroupStyle {}

#[derive(TwClass)]
#[tw(
    class = "inline-flex items-center justify-center gap-2 rounded-md text-sm font-medium ring-offset-background transition-colors hover:bg-muted hover:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 data-[state=on]:bg-accent data-[state=on]:text-accent-foreground [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0"
)]
struct ToggleGroupItemStyle {
    variant: ToggleVariant,
    size: ToggleSize,
}

/// A styled toggle group wrapping the behavior primitive.
#[component]
pub fn ToggleGroup(
    #[prop(optional, into)] class: String,
    #[prop(optional)] value: RwSignal<Vec<String>>,
    #[prop(optional)] group_type: ToggleGroupType,
    #[prop(optional)] variant: ToggleVariant,
    #[prop(optional)] size: ToggleSize,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional_no_strip)] on_value_change: Option<Callback<Vec<String>>>,
    children: Children,
) -> impl IntoView {
    let final_class = ToggleGroupStyle {}.with_class(&class);

    provide_context(ToggleGroupItemVisualContext { variant, size });

    view! {
        <ToggleGroupRoot
            value=value
            group_type=group_type
            disabled=disabled
            class=final_class
            on_value_change=on_value_change
        >
            {children()}
        </ToggleGroupRoot>
    }
}

/// Visual context passed from `ToggleGroup` to `ToggleGroupItem` for shared variant/size.
#[derive(Clone, Copy)]
struct ToggleGroupItemVisualContext {
    variant: ToggleVariant,
    size: ToggleSize,
}

/// A styled toggle group item.
#[component]
pub fn ToggleGroupItem(
    #[prop(optional, into)] class: String,
    #[prop(into)] value: String,
    #[prop(optional)] variant: Option<ToggleVariant>,
    #[prop(optional)] size: Option<ToggleSize>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional_no_strip, into)] aria_label: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    let visual_ctx = use_context::<ToggleGroupItemVisualContext>();
    let resolved_variant = variant.unwrap_or_else(|| {
        visual_ctx
            .map(|ctx| ctx.variant)
            .unwrap_or(ToggleVariant::Default)
    });
    let resolved_size = size.unwrap_or_else(|| {
        visual_ctx
            .map(|ctx| ctx.size)
            .unwrap_or(ToggleSize::Default)
    });

    let final_class = ToggleGroupItemStyle {
        variant: resolved_variant,
        size: resolved_size,
    }
    .with_class(&class);

    view! {
        <crate::behavior::ToggleGroupItem
            value=value
            disabled=disabled
            class=final_class
            aria_label=aria_label
        >
            {children()}
        </crate::behavior::ToggleGroupItem>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn class_prop_accepts_str_and_string() {
        assert_class_prop!(ToggleGroupProps, ToggleGroupItemProps);
    }
}
