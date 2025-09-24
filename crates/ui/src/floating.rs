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
    _floating: &U,
    options: FloatingOptions,
) -> Option<FloatingPosition>
where
    T: AsRef<leptos::web_sys::HtmlElement>,
{
    use leptos::wasm_bindgen::JsCast;
    use leptos::web_sys::*;

    // Get the bounding rectangle of the reference element
    let reference_element: &HtmlElement = reference.as_ref();
    let rect = reference_element
        .unchecked_ref::<Element>()
        .get_bounding_client_rect();

    let reference_x = rect.left();
    let reference_y = rect.top();
    let reference_width = rect.width();
    let reference_height = rect.height();

    calculate_position_from_rect(
        reference_x,
        reference_y,
        reference_width,
        reference_height,
        options,
    )
}

/// Calculate position from bounding rectangle values (testable without DOM)
pub fn calculate_position_from_rect(
    reference_x: f64,
    reference_y: f64,
    reference_width: f64,
    reference_height: f64,
    options: FloatingOptions,
) -> Option<FloatingPosition> {
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

    // Apply alignment offset (for now, just use the base position)
    // TODO: Apply align offset based on floating element dimensions
    let (x, y) = (base_x + options.align_offset, base_y);

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
        let _sides = [Side::Top, Side::Right, Side::Bottom, Side::Left];

        // Test Debug formatting
        assert_eq!(format!("{:?}", Side::Top), "Top");
        assert_eq!(format!("{:?}", Side::Right), "Right");
        assert_eq!(format!("{:?}", Side::Bottom), "Bottom");
        assert_eq!(format!("{:?}", Side::Left), "Left");

        // Test equality
        assert_eq!(Side::Top, Side::Top);
        assert_ne!(Side::Top, Side::Bottom);

        // Test Clone
        let cloned = Side::Top.clone();
        assert_eq!(cloned, Side::Top);
    }

    #[test]
    fn align_enum_all_variants() {
        let _aligns = [Align::Start, Align::Center, Align::End];

        // Test Debug formatting
        assert_eq!(format!("{:?}", Align::Start), "Start");
        assert_eq!(format!("{:?}", Align::Center), "Center");
        assert_eq!(format!("{:?}", Align::End), "End");

        // Test equality
        assert_eq!(Align::Start, Align::Start);
        assert_ne!(Align::Start, Align::Center);

        // Test Clone
        let cloned = Align::Center.clone();
        assert_eq!(cloned, Align::Center);
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
        // Test that we can build options incrementally
        let mut options = FloatingOptions::default();
        options.side = Side::Top;
        options.align = Align::End;
        options.side_offset = 16.0;

        assert_eq!(options.side, Side::Top);
        assert_eq!(options.align, Align::End);
        assert_eq!(options.side_offset, 16.0);
        assert_eq!(options.align_offset, 0.0); // Should remain default
    }

    #[test]
    fn side_enum_exhaustive_match() {
        // Ensures we handle all Side variants (will fail if new ones are added)
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
        // Ensures we handle all Align variants (will fail if new ones are added)
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
        // Test that negative offsets are handled correctly
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

        // Test with mock elements (we can't access DOM in unit tests)
        let position = calculate_position_from_rect(100.0, 200.0, 120.0, 40.0, options).unwrap();

        assert_eq!(position.x, 100.0); // reference_x
        assert_eq!(position.y, 244.0); // reference_y (200) + height (40) + side_offset (4.0)
        assert_eq!(position.side, Side::Bottom);
        assert_eq!(position.align, Align::Start);
    }

    #[test]
    fn calculate_position_top_side() {
        let options = FloatingOptions {
            side: Side::Top,
            align: Align::Center,
            side_offset: 8.0,
            align_offset: 0.0,
        };

        let position = calculate_position_from_rect(100.0, 200.0, 120.0, 40.0, options).unwrap();

        assert_eq!(position.x, 100.0); // reference_x
        assert_eq!(position.y, 192.0); // reference_y (200) - side_offset (8.0)
        assert_eq!(position.side, Side::Top);
        assert_eq!(position.align, Align::Center);
    }

    #[test]
    fn calculate_position_right_side() {
        let options = FloatingOptions {
            side: Side::Right,
            align: Align::End,
            side_offset: 12.0,
            align_offset: 0.0,
        };

        let position = calculate_position_from_rect(100.0, 200.0, 120.0, 40.0, options).unwrap();

        assert_eq!(position.x, 232.0); // reference_x (100) + width (120) + side_offset (12.0)
        assert_eq!(position.y, 200.0); // reference_y
        assert_eq!(position.side, Side::Right);
        assert_eq!(position.align, Align::End);
    }

    #[test]
    fn calculate_position_left_side() {
        let options = FloatingOptions {
            side: Side::Left,
            align: Align::Start,
            side_offset: 6.0,
            align_offset: 0.0,
        };

        let position = calculate_position_from_rect(100.0, 200.0, 120.0, 40.0, options).unwrap();

        assert_eq!(position.x, 94.0); // reference_x (100) - side_offset (6.0)
        assert_eq!(position.y, 200.0); // reference_y
        assert_eq!(position.side, Side::Left);
        assert_eq!(position.align, Align::Start);
    }

    #[test]
    fn calculate_position_zero_offset() {
        let options = FloatingOptions {
            side: Side::Bottom,
            align: Align::Start,
            side_offset: 0.0,
            align_offset: 0.0,
        };

        let position = calculate_position_from_rect(100.0, 200.0, 120.0, 40.0, options).unwrap();

        assert_eq!(position.x, 100.0); // reference_x
        assert_eq!(position.y, 240.0); // reference_y (200) + height (40) + side_offset (0.0)
    }

    #[test]
    fn calculate_position_negative_offset() {
        let options = FloatingOptions {
            side: Side::Top,
            align: Align::Start,
            side_offset: -10.0, // Negative offset
            align_offset: 0.0,
        };

        let position = calculate_position_from_rect(100.0, 200.0, 120.0, 40.0, options).unwrap();

        assert_eq!(position.x, 100.0); // reference_x
        assert_eq!(position.y, 210.0); // reference_y (200) - side_offset (-10.0) = 200 - (-10) = 210
    }

    #[test]
    fn calculate_position_large_offset() {
        let options = FloatingOptions {
            side: Side::Right,
            align: Align::Start,
            side_offset: 1000.0, // Large offset
            align_offset: 0.0,
        };

        let position = calculate_position_from_rect(100.0, 200.0, 120.0, 40.0, options).unwrap();

        assert_eq!(position.x, 1220.0); // reference_x (100) + width (120) + side_offset (1000.0)
        assert_eq!(position.y, 200.0); // reference_y
    }

    #[test]
    fn floating_position_all_combinations() {
        // Test all side/align combinations to ensure comprehensive coverage
        let sides = [Side::Top, Side::Right, Side::Bottom, Side::Left];
        let aligns = [Align::Start, Align::Center, Align::End];

        for side in sides.iter() {
            for align in aligns.iter() {
                let options = FloatingOptions {
                    side: *side,
                    align: *align,
                    side_offset: 10.0,
                    align_offset: 5.0,
                };

                let position = calculate_position_from_rect(100.0, 200.0, 120.0, 40.0, options);
                assert!(
                    position.is_some(),
                    "Position calculation should succeed for side {:?} and align {:?}",
                    side,
                    align
                );

                let pos = position.unwrap();
                assert_eq!(pos.side, *side);
                assert_eq!(pos.align, *align);

                // Verify positioning logic for each side (align_offset 5.0 is added to x for all sides currently)
                match side {
                    Side::Top => {
                        assert_eq!(pos.x, 105.0); // reference_x (100) + align_offset (5.0)
                        assert_eq!(pos.y, 190.0); // reference_y (200) - side_offset (10.0)
                    }
                    Side::Bottom => {
                        assert_eq!(pos.x, 105.0); // reference_x (100) + align_offset (5.0)
                        assert_eq!(pos.y, 250.0); // reference_y (200) + height (40) + side_offset (10.0)
                    }
                    Side::Right => {
                        assert_eq!(pos.x, 235.0); // reference_x (100) + width (120) + side_offset (10.0) + align_offset (5.0)
                        assert_eq!(pos.y, 200.0); // reference_y (200)
                    }
                    Side::Left => {
                        assert_eq!(pos.x, 95.0); // reference_x (100) - side_offset (10.0) + align_offset (5.0)
                        assert_eq!(pos.y, 200.0); // reference_y (200)
                    }
                }
            }
        }
    }
}
