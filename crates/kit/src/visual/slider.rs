use leptos::prelude::*;
use tailwind_fuse::*;

use crate::behavior::{SliderRange, SliderRoot, SliderThumb, SliderTrack};

#[derive(TwClass)]
#[tw(class = "relative flex w-full touch-none select-none items-center")]
struct SliderRootStyle {}

#[derive(TwClass)]
#[tw(class = "relative h-1.5 w-full grow overflow-hidden rounded-full bg-primary/20")]
struct SliderTrackStyle {}

#[derive(TwClass)]
#[tw(class = "absolute h-full bg-primary")]
struct SliderRangeStyle {}

#[derive(TwClass)]
#[tw(
    class = "block h-4 w-4 rounded-full border border-primary/50 bg-background shadow transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50"
)]
struct SliderThumbStyle {}

/// A styled slider component for selecting a value from a range
#[component]
pub fn Slider(
    #[prop(optional, into)] class: String,
    #[prop(optional)] value: RwSignal<f64>,
    #[prop(into, default = 0.0)] min: f64,
    #[prop(into, default = 100.0)] max: f64,
    #[prop(into, default = 1.0)] step: f64,
    #[prop(into, default = Signal::stored(false))] disabled: Signal<bool>,
    #[prop(optional_no_strip)] on_change: Option<Callback<f64>>,
) -> impl IntoView {
    let root_class = SliderRootStyle {}.with_class(&class);
    let track_class = SliderTrackStyle {}.to_class();
    let range_class = SliderRangeStyle {}.to_class();
    let thumb_class = SliderThumbStyle {}.to_class();

    view! {
        <SliderRoot
            class=root_class
            value=value
            min=min
            max=max
            step=step
            disabled=disabled
            on_change=on_change
        >
            <SliderTrack class=track_class>
                <SliderRange class=range_class />
                <SliderThumb class=thumb_class />
            </SliderTrack>
        </SliderRoot>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn class_prop_accepts_str_and_string() {
        assert_class_prop!(SliderProps);
    }
}
