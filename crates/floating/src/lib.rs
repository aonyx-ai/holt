use leptos::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Side {
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Align {
    Start,
    Center,
    End,
}

#[derive(Debug, Clone)]
pub struct FloatingOptions {
    pub side: Side,
    pub align: Align,
    pub side_offset: f64,
    pub align_offset: f64,
}

impl Default for FloatingOptions {
    fn default() -> Self {
        Self {
            side: Side::Bottom,
            align: Align::Start,
            side_offset: 0.0,
            align_offset: 0.0,
        }
    }
}

/// Calculated position for floating elements
#[derive(Debug, Clone)]
pub struct FloatingPosition {
    pub x: f64,
    pub y: f64,
    pub side: Side,
    pub align: Align,
}

/// Returns dynamic positioning data for floating elements
pub struct UseFloatingReturn {
    pub x: RwSignal<f64>,
    pub y: RwSignal<f64>,
    pub side: Signal<Side>,
    pub align: Signal<Align>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FloatingX(f64);

impl Default for FloatingX {
    fn default() -> Self {
        Self(0.0)
    }
}

impl From<FloatingX> for f64 {
    fn from(val: FloatingX) -> Self {
        val.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FloatingY(f64);

impl Default for FloatingY {
    fn default() -> Self {
        Self(0.0)
    }
}

impl From<FloatingY> for f64 {
    fn from(val: FloatingY) -> Self {
        val.0
    }
}

pub fn use_floating(
    reference_ref: NodeRef<leptos::html::Button>,
    floating_ref: NodeRef<leptos::html::Div>,
    options: FloatingOptions,
) -> UseFloatingReturn {
    let x = RwSignal::new(FloatingX::default().into());
    let y = RwSignal::new(FloatingY::default().into());
    let side = RwSignal::new(options.side);
    let align = RwSignal::new(options.align);

    // Calculate position whenever elements change
    Effect::new({
        let options = options.clone();

        move |_| {
            // Track NodeRefs so effect re-runs when they get populated
            let _ = (reference_ref.get(), floating_ref.get());

            request_animation_frame({
                let x = x;
                let y = y;
                let side = side;
                let align = align;
                let options = options.clone();

                move || {
                    // Use untracked access inside the animation frame to avoid context warnings
                    if let (Some(reference), Some(floating)) =
                        (reference_ref.get_untracked(), floating_ref.get_untracked())
                        && let Some(position) = calculate_position(&reference, &floating, options)
                    {
                        x.set(position.x);
                        y.set(position.y);
                        side.set(position.side);
                        align.set(position.align);
                    }
                }
            });
        }
    });

    UseFloatingReturn {
        x,
        y,
        side: side.into(),
        align: align.into(),
    }
}

/// Calculate the optimal position for a floating element
pub fn calculate_position<T, U>(
    reference: &T,
    floating: &U,
    options: FloatingOptions,
) -> Option<FloatingPosition>
where
    T: AsRef<leptos::web_sys::HtmlElement>,
    U: AsRef<leptos::web_sys::HtmlElement>,
{
    use leptos::wasm_bindgen::JsCast;
    use leptos::web_sys::*;

    // Get the bounding rectangle of the reference element
    let reference_element: &HtmlElement = reference.as_ref();
    let ref_rect = reference_element
        .unchecked_ref::<Element>()
        .get_bounding_client_rect();

    // Get the bounding rectangle of the floating element
    let floating_element: &HtmlElement = floating.as_ref();
    let float_rect = floating_element
        .unchecked_ref::<Element>()
        .get_bounding_client_rect();

    calculate_position_from_rect(
        ref_rect.left(),
        ref_rect.top(),
        ref_rect.width(),
        ref_rect.height(),
        float_rect.width(),
        float_rect.height(),
        options,
    )
}

/// Calculate position from bounding rectangle values (testable without DOM)
pub fn calculate_position_from_rect(
    reference_x: f64,
    reference_y: f64,
    reference_width: f64,
    reference_height: f64,
    floating_width: f64,
    floating_height: f64,
    options: FloatingOptions,
) -> Option<FloatingPosition> {
    // Determine whether the cross-axis is horizontal (x) or vertical (y)
    let is_horizontal_side = matches!(options.side, Side::Top | Side::Bottom);

    // Calculate base position based on side
    let (base_x, base_y) = match options.side {
        Side::Top => (reference_x, reference_y - options.side_offset),
        Side::Right => (
            reference_x + reference_width + options.side_offset,
            reference_y,
        ),
        Side::Bottom => (
            reference_x,
            reference_y + reference_height + options.side_offset,
        ),
        Side::Left => (reference_x - options.side_offset, reference_y),
    };

    // Calculate alignment shift on the cross-axis
    let align_shift = match options.align {
        Align::Start => 0.0,
        Align::Center => {
            if is_horizontal_side {
                (reference_width - floating_width) / 2.0
            } else {
                (reference_height - floating_height) / 2.0
            }
        }
        Align::End => {
            if is_horizontal_side {
                reference_width - floating_width
            } else {
                reference_height - floating_height
            }
        }
    };

    // Apply alignment shift and align_offset on the cross-axis
    let (x, y) = if is_horizontal_side {
        (base_x + align_shift + options.align_offset, base_y)
    } else {
        (base_x, base_y + align_shift + options.align_offset)
    };

    Some(FloatingPosition {
        x,
        y,
        side: options.side,
        align: options.align,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn floating_options_default_values() {
        let options = FloatingOptions::default();
        assert_eq!(options.side, Side::Bottom);
        assert_eq!(options.align, Align::Start);
        assert_eq!(options.side_offset, 0.0);
        assert_eq!(options.align_offset, 0.0);
    }

    #[test]
    fn floating_options_custom_values() {
        let options = FloatingOptions {
            side: Side::Top,
            align: Align::Center,
            side_offset: 8.0,
            align_offset: 4.0,
        };
        assert_eq!(options.side, Side::Top);
        assert_eq!(options.align, Align::Center);
        assert_eq!(options.side_offset, 8.0);
        assert_eq!(options.align_offset, 4.0);
    }

    #[test]
    fn side_enum_all_variants() {
        assert_eq!(format!("{:?}", Side::Top), "Top");
        assert_eq!(format!("{:?}", Side::Right), "Right");
        assert_eq!(format!("{:?}", Side::Bottom), "Bottom");
        assert_eq!(format!("{:?}", Side::Left), "Left");

        assert_eq!(Side::Top, Side::Top);
        assert_ne!(Side::Top, Side::Bottom);
    }

    #[test]
    fn align_enum_all_variants() {
        assert_eq!(format!("{:?}", Align::Start), "Start");
        assert_eq!(format!("{:?}", Align::Center), "Center");
        assert_eq!(format!("{:?}", Align::End), "End");

        assert_eq!(Align::Start, Align::Start);
        assert_ne!(Align::Start, Align::Center);
    }

    #[test]
    fn floating_options_clone_trait() {
        let original = FloatingOptions {
            side: Side::Right,
            align: Align::End,
            side_offset: 10.0,
            align_offset: 5.0,
        };

        let cloned = original.clone();
        assert_eq!(cloned.side, original.side);
        assert_eq!(cloned.align, original.align);
        assert_eq!(cloned.side_offset, original.side_offset);
        assert_eq!(cloned.align_offset, original.align_offset);
    }

    #[test]
    fn floating_position_structure() {
        let position = FloatingPosition {
            x: 100.0,
            y: 200.0,
            side: Side::Bottom,
            align: Align::Center,
        };

        assert_eq!(position.x, 100.0);
        assert_eq!(position.y, 200.0);
        assert_eq!(position.side, Side::Bottom);
        assert_eq!(position.align, Align::Center);
    }

    #[test]
    fn floating_x_default_is_zero() {
        assert_eq!(f64::from(FloatingX::default()), 0.0);
    }

    #[test]
    fn floating_y_default_is_zero() {
        assert_eq!(f64::from(FloatingY::default()), 0.0);
    }

    #[test]
    fn floating_options_builder_pattern() {
        let options = FloatingOptions {
            side: Side::Top,
            align: Align::End,
            side_offset: 16.0,
            ..Default::default()
        };

        assert_eq!(options.side, Side::Top);
        assert_eq!(options.align, Align::End);
        assert_eq!(options.side_offset, 16.0);
        assert_eq!(options.align_offset, 0.0);
    }

    #[test]
    fn side_enum_exhaustive_match() {
        let test_side = Side::Bottom;
        let result = match test_side {
            Side::Top => "top",
            Side::Right => "right",
            Side::Bottom => "bottom",
            Side::Left => "left",
        };
        assert_eq!(result, "bottom");
    }

    #[test]
    fn align_enum_exhaustive_match() {
        let test_align = Align::Center;
        let result = match test_align {
            Align::Start => "start",
            Align::Center => "center",
            Align::End => "end",
        };
        assert_eq!(result, "center");
    }

    #[test]
    fn floating_options_negative_offsets() {
        let options = FloatingOptions {
            side: Side::Bottom,
            align: Align::Start,
            side_offset: -5.0,
            align_offset: -10.0,
        };

        assert_eq!(options.side_offset, -5.0);
        assert_eq!(options.align_offset, -10.0);
    }

    #[test]
    fn floating_options_zero_offsets() {
        let options = FloatingOptions {
            side: Side::Top,
            align: Align::Start,
            side_offset: 0.0,
            align_offset: 0.0,
        };

        assert_eq!(options.side_offset, 0.0);
        assert_eq!(options.align_offset, 0.0);
    }

    #[test]
    fn floating_position_clone() {
        let original = FloatingPosition {
            x: 150.0,
            y: 250.0,
            side: Side::Right,
            align: Align::End,
        };

        let cloned = original.clone();
        assert_eq!(cloned.x, original.x);
        assert_eq!(cloned.y, original.y);
        assert_eq!(cloned.side, original.side);
        assert_eq!(cloned.align, original.align);
    }

    #[test]
    fn calculate_position_bottom_side() {
        let options = FloatingOptions {
            side: Side::Bottom,
            align: Align::Start,
            side_offset: 4.0,
            align_offset: 0.0,
        };

        let position =
            calculate_position_from_rect(100.0, 200.0, 120.0, 40.0, 80.0, 30.0, options).unwrap();

        assert_eq!(position.x, 100.0);
        assert_eq!(position.y, 244.0);
        assert_eq!(position.side, Side::Bottom);
        assert_eq!(position.align, Align::Start);
    }

    #[test]
    fn calculate_position_top_side() {
        let options = FloatingOptions {
            side: Side::Top,
            align: Align::Start,
            side_offset: 8.0,
            align_offset: 0.0,
        };

        let position =
            calculate_position_from_rect(100.0, 200.0, 120.0, 40.0, 80.0, 30.0, options).unwrap();

        assert_eq!(position.x, 100.0);
        assert_eq!(position.y, 192.0);
        assert_eq!(position.side, Side::Top);
    }

    #[test]
    fn calculate_position_right_side() {
        let options = FloatingOptions {
            side: Side::Right,
            align: Align::Start,
            side_offset: 12.0,
            align_offset: 0.0,
        };

        let position =
            calculate_position_from_rect(100.0, 200.0, 120.0, 40.0, 80.0, 30.0, options).unwrap();

        assert_eq!(position.x, 232.0);
        assert_eq!(position.y, 200.0);
        assert_eq!(position.side, Side::Right);
    }

    #[test]
    fn calculate_position_left_side() {
        let options = FloatingOptions {
            side: Side::Left,
            align: Align::Start,
            side_offset: 6.0,
            align_offset: 0.0,
        };

        let position =
            calculate_position_from_rect(100.0, 200.0, 120.0, 40.0, 80.0, 30.0, options).unwrap();

        assert_eq!(position.x, 94.0);
        assert_eq!(position.y, 200.0);
        assert_eq!(position.side, Side::Left);
    }

    #[test]
    fn calculate_position_zero_offset() {
        let options = FloatingOptions {
            side: Side::Bottom,
            align: Align::Start,
            side_offset: 0.0,
            align_offset: 0.0,
        };

        let position =
            calculate_position_from_rect(100.0, 200.0, 120.0, 40.0, 80.0, 30.0, options).unwrap();

        assert_eq!(position.x, 100.0);
        assert_eq!(position.y, 240.0);
    }

    #[test]
    fn calculate_position_negative_offset() {
        let options = FloatingOptions {
            side: Side::Top,
            align: Align::Start,
            side_offset: -10.0,
            align_offset: 0.0,
        };

        let position =
            calculate_position_from_rect(100.0, 200.0, 120.0, 40.0, 80.0, 30.0, options).unwrap();

        assert_eq!(position.x, 100.0);
        assert_eq!(position.y, 210.0);
    }

    #[test]
    fn calculate_position_large_offset() {
        let options = FloatingOptions {
            side: Side::Right,
            align: Align::Start,
            side_offset: 1000.0,
            align_offset: 0.0,
        };

        let position =
            calculate_position_from_rect(100.0, 200.0, 120.0, 40.0, 80.0, 30.0, options).unwrap();

        assert_eq!(position.x, 1220.0);
        assert_eq!(position.y, 200.0);
    }
}
