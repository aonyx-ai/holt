use leptos_floating::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test(unsupported = test)]
#[cfg_attr(not(target_family = "wasm"), ignore)]
fn basic_floating_creation() {
    // Test basic floating data structures work in WASM
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

    let cloned = position.clone();
    assert_eq!(cloned.x, position.x);
}

#[wasm_bindgen_test(unsupported = test)]
#[cfg_attr(not(target_family = "wasm"), ignore)]
fn floating_options_creation() {
    // Test FloatingOptions can be created with different values
    let options = FloatingOptions {
        side: Side::Top,
        align: Align::End,
        side_offset: 15.0,
        align_offset: 8.0,
    };

    assert_eq!(options.side, Side::Top);
    assert_eq!(options.align, Align::End);
    assert_eq!(options.side_offset, 15.0);
    assert_eq!(options.align_offset, 8.0);

    // Test default values
    let default_options = FloatingOptions::default();
    assert_eq!(default_options.side, Side::Bottom);
    assert_eq!(default_options.align, Align::Start);
    assert_eq!(default_options.side_offset, 0.0);
    assert_eq!(default_options.align_offset, 0.0);
}

#[wasm_bindgen_test(unsupported = test)]
#[cfg_attr(not(target_family = "wasm"), ignore)]
fn side_align_enums_work() {
    // Test that all enum variants work in WASM
    let sides = [Side::Top, Side::Right, Side::Bottom, Side::Left];
    let aligns = [Align::Start, Align::Center, Align::End];

    for side in sides.iter() {
        for align in aligns.iter() {
            let position = FloatingPosition {
                x: 0.0,
                y: 0.0,
                side: *side,
                align: *align,
            };

            assert_eq!(position.side, *side);
            assert_eq!(position.align, *align);
        }
    }
}
