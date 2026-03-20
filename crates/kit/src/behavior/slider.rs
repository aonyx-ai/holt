use leptos::html::Div;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use leptos::wasm_bindgen::closure::Closure;

/// Slider behavior context that manages value, range, and drag state
#[derive(Clone)]
pub struct SliderContext {
    pub value: RwSignal<f64>,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub disabled: Signal<bool>,
    pub on_change: Option<Callback<f64>>,
    pub track_ref: NodeRef<Div>,
    pub dragging: RwSignal<bool>,
}

impl SliderContext {
    pub fn new(
        value: RwSignal<f64>,
        min: f64,
        max: f64,
        step: f64,
        disabled: Signal<bool>,
        on_change: Option<Callback<f64>>,
        track_ref: NodeRef<Div>,
    ) -> Self {
        Self {
            value,
            min,
            max,
            step,
            disabled,
            on_change,
            track_ref,
            dragging: RwSignal::new(false),
        }
    }

    pub fn is_disabled(&self) -> bool {
        self.disabled.get()
    }

    /// Set the value, clamping to [min, max] and snapping to step
    pub fn set_value(&self, new_value: f64) {
        if self.is_disabled() {
            return;
        }
        let snapped = snap_to_step(new_value, self.min, self.max, self.step);
        self.value.set(snapped);
        if let Some(cb) = &self.on_change {
            cb.run(snapped);
        }
    }

    /// Returns the current value as a percentage of the range [0.0, 100.0]
    pub fn percentage(&self) -> f64 {
        value_to_percentage(self.value.get(), self.min, self.max)
    }
}

/// Calculate a value from a position along the track
pub fn value_from_position(
    client_x: f64,
    track_left: f64,
    track_width: f64,
    min: f64,
    max: f64,
) -> f64 {
    if track_width == 0.0 {
        return min;
    }
    let ratio = (client_x - track_left) / track_width;
    let ratio = ratio.clamp(0.0, 1.0);
    min + ratio * (max - min)
}

/// Convert a value to a percentage of the range
pub fn value_to_percentage(value: f64, min: f64, max: f64) -> f64 {
    if (max - min).abs() < f64::EPSILON {
        return 0.0;
    }
    ((value - min) / (max - min) * 100.0).clamp(0.0, 100.0)
}

/// Snap a value to the nearest step, clamped within [min, max]
pub fn snap_to_step(value: f64, min: f64, max: f64, step: f64) -> f64 {
    let clamped = value.clamp(min, max);
    if step <= 0.0 {
        return clamped;
    }
    let steps_from_min = ((clamped - min) / step).round();
    let snapped = min + steps_from_min * step;
    snapped.clamp(min, max)
}

/// Root slider primitive that provides context and handles drag interactions
#[component]
pub fn SliderRoot(
    #[prop(optional, into)] class: String,
    #[prop(optional)] value: RwSignal<f64>,
    #[prop(into, default = 0.0)] min: f64,
    #[prop(into, default = 100.0)] max: f64,
    #[prop(into, default = 1.0)] step: f64,
    #[prop(into, default = Signal::stored(false))] disabled: Signal<bool>,
    #[prop(optional_no_strip)] on_change: Option<Callback<f64>>,
    children: Children,
) -> impl IntoView {
    let track_ref = NodeRef::<Div>::new();
    let context = SliderContext::new(value, min, max, step, disabled, on_change, track_ref);
    let ctx = context.clone();
    provide_context(context);

    // Handle pointer down on the track area — sets value and starts drag
    let ctx_down = ctx.clone();
    let on_pointerdown = move |ev: web_sys::PointerEvent| {
        if ctx_down.is_disabled() {
            return;
        }
        ev.prevent_default();
        if let Some(track_el) = ctx_down.track_ref.get() {
            let el: &web_sys::Element = &track_el;
            let rect = el.get_bounding_client_rect();
            let new_value =
                value_from_position(ev.client_x() as f64, rect.left(), rect.width(), min, max);
            ctx_down.set_value(new_value);
            ctx_down.dragging.set(true);
        }
    };

    // Set up window-level mousemove and mouseup listeners for drag
    let ctx_effect = ctx.clone();
    Effect::new(move |_| {
        if !ctx_effect.dragging.get() {
            return;
        }

        let ctx_move = ctx_effect.clone();
        let ctx_up = ctx_effect.clone();

        let on_pointermove: Closure<dyn Fn(web_sys::PointerEvent)> =
            Closure::new(move |ev: web_sys::PointerEvent| {
                if let Some(track_el) = ctx_move.track_ref.get() {
                    let el: &web_sys::Element = &track_el;
                    let rect = el.get_bounding_client_rect();
                    let new_value = value_from_position(
                        ev.client_x() as f64,
                        rect.left(),
                        rect.width(),
                        min,
                        max,
                    );
                    ctx_move.set_value(new_value);
                }
            });

        // We need to share the move closure ref to remove it later
        let move_fn = on_pointermove
            .as_ref()
            .unchecked_ref::<web_sys::js_sys::Function>()
            .clone();

        let on_pointerup: Closure<dyn FnMut(web_sys::PointerEvent)> =
            Closure::once(move |_ev: web_sys::PointerEvent| {
                ctx_up.dragging.set(false);
                if let Some(win) = web_sys::window() {
                    let _ = win.remove_event_listener_with_callback("pointermove", &move_fn);
                }
                // Note: pointerup listener removes itself via Closure::once drop
            });

        if let Some(win) = web_sys::window() {
            let _ = win.add_event_listener_with_callback(
                "pointermove",
                on_pointermove.as_ref().unchecked_ref(),
            );
            let _ = win.add_event_listener_with_callback(
                "pointerup",
                on_pointerup.as_ref().unchecked_ref(),
            );
            // Leak closures so they survive past this scope — they clean themselves up
            on_pointermove.forget();
            on_pointerup.forget();
        }
    });

    view! {
        <div
            class=class
            role="slider"
            aria-valuemin=min
            aria-valuemax=max
            aria-valuenow=move || value.get()
            aria-disabled=move || disabled.get()
            data-disabled=move || disabled.get().then_some("")
            on:pointerdown=on_pointerdown
        >
            {children()}
        </div>
    }
}

/// Hook to access slider context
pub fn use_slider() -> SliderContext {
    use_context::<SliderContext>().expect("use_slider must be called within SliderRoot")
}

/// The track element — typically wraps the range and thumb
#[component]
pub fn SliderTrack(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let context = use_slider();

    view! {
        <div class=class node_ref=context.track_ref>
            {children()}
        </div>
    }
}

/// The filled range portion of the slider
#[component]
pub fn SliderRange(#[prop(optional, into)] class: String) -> impl IntoView {
    let context = use_slider();

    view! { <span class=class style:width=move || format!("{}%", context.percentage()) /> }
}

/// The draggable thumb
#[component]
pub fn SliderThumb(#[prop(optional, into)] class: String) -> impl IntoView {
    let context = use_slider();

    view! {
        <span
            class=class
            tabindex="0"
            role="slider"
            aria-valuemin=context.min
            aria-valuemax=context.max
            aria-valuenow=move || context.value.get()
            style:left=move || format!("{}%", context.percentage())
            style:position="absolute"
            style:transform="translateX(-50%)"
        />
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{reactive_scope, track_callback};

    #[test]
    fn class_prop_accepts_str_and_string() {
        assert_class_prop!(
            SliderRootProps,
            SliderTrackProps,
            SliderRangeProps,
            SliderThumbProps,
        );
    }

    #[test]
    fn value_from_position_calculates_correctly() {
        // Left edge
        assert!((value_from_position(0.0, 0.0, 100.0, 0.0, 100.0) - 0.0).abs() < f64::EPSILON);
        // Right edge
        assert!((value_from_position(100.0, 0.0, 100.0, 0.0, 100.0) - 100.0).abs() < f64::EPSILON);
        // Middle
        assert!((value_from_position(50.0, 0.0, 100.0, 0.0, 100.0) - 50.0).abs() < f64::EPSILON);
        // With offset track
        assert!((value_from_position(150.0, 100.0, 200.0, 0.0, 100.0) - 25.0).abs() < f64::EPSILON);
        // Clamped below
        assert!((value_from_position(-10.0, 0.0, 100.0, 0.0, 100.0) - 0.0).abs() < f64::EPSILON);
        // Clamped above
        assert!((value_from_position(200.0, 0.0, 100.0, 0.0, 100.0) - 100.0).abs() < f64::EPSILON);
    }

    #[test]
    fn value_from_position_zero_width_returns_min() {
        assert!((value_from_position(50.0, 0.0, 0.0, 10.0, 90.0) - 10.0).abs() < f64::EPSILON);
    }

    #[test]
    fn value_to_percentage_calculates_correctly() {
        assert!((value_to_percentage(0.0, 0.0, 100.0) - 0.0).abs() < f64::EPSILON);
        assert!((value_to_percentage(50.0, 0.0, 100.0) - 50.0).abs() < f64::EPSILON);
        assert!((value_to_percentage(100.0, 0.0, 100.0) - 100.0).abs() < f64::EPSILON);
        // Custom range
        assert!((value_to_percentage(5.0, 0.0, 10.0) - 50.0).abs() < f64::EPSILON);
    }

    #[test]
    fn value_to_percentage_equal_min_max() {
        assert!((value_to_percentage(5.0, 5.0, 5.0) - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn snap_to_step_snaps_correctly() {
        assert!((snap_to_step(0.0, 0.0, 100.0, 10.0) - 0.0).abs() < f64::EPSILON);
        assert!((snap_to_step(3.0, 0.0, 100.0, 10.0) - 0.0).abs() < f64::EPSILON);
        assert!((snap_to_step(7.0, 0.0, 100.0, 10.0) - 10.0).abs() < f64::EPSILON);
        assert!((snap_to_step(15.0, 0.0, 100.0, 10.0) - 20.0).abs() < f64::EPSILON);
        assert!((snap_to_step(99.0, 0.0, 100.0, 10.0) - 100.0).abs() < f64::EPSILON);
    }

    #[test]
    fn snap_to_step_clamps_to_range() {
        assert!((snap_to_step(-10.0, 0.0, 100.0, 10.0) - 0.0).abs() < f64::EPSILON);
        assert!((snap_to_step(110.0, 0.0, 100.0, 10.0) - 100.0).abs() < f64::EPSILON);
    }

    #[test]
    fn snap_to_step_zero_step_returns_clamped() {
        assert!((snap_to_step(55.0, 0.0, 100.0, 0.0) - 55.0).abs() < f64::EPSILON);
    }

    #[test]
    fn set_value_clamps_and_snaps() {
        reactive_scope(|| {
            let value = RwSignal::new(0.0);
            let disabled = Signal::stored(false);
            let track_ref = NodeRef::<Div>::new();

            let context = SliderContext::new(value, 0.0, 100.0, 10.0, disabled, None, track_ref);

            context.set_value(33.0);
            assert!((value.get() - 30.0).abs() < f64::EPSILON);

            context.set_value(77.0);
            assert!((value.get() - 80.0).abs() < f64::EPSILON);

            context.set_value(150.0);
            assert!((value.get() - 100.0).abs() < f64::EPSILON);

            context.set_value(-10.0);
            assert!((value.get() - 0.0).abs() < f64::EPSILON);
        });
    }

    #[test]
    fn on_change_fires_on_set_value() {
        reactive_scope(|| {
            let (on_change, last) = track_callback::<f64>();
            let value = RwSignal::new(0.0);
            let disabled = Signal::stored(false);
            let track_ref = NodeRef::<Div>::new();

            let context =
                SliderContext::new(value, 0.0, 100.0, 1.0, disabled, Some(on_change), track_ref);

            context.set_value(42.0);
            assert!((value.get() - 42.0).abs() < f64::EPSILON);
            assert_eq!(last.get(), Some(42.0));
        });
    }

    #[test]
    fn on_change_not_fired_when_disabled() {
        reactive_scope(|| {
            let (on_change, last) = track_callback::<f64>();
            let value = RwSignal::new(0.0);
            let disabled = Signal::stored(true);
            let track_ref = NodeRef::<Div>::new();

            let context =
                SliderContext::new(value, 0.0, 100.0, 1.0, disabled, Some(on_change), track_ref);

            context.set_value(42.0);
            assert!((value.get() - 0.0).abs() < f64::EPSILON);
            assert_eq!(last.get(), None);
        });
    }
}
