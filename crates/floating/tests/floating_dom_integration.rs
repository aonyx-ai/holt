use leptos_floating::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test(unsupported = test)]
#[cfg_attr(not(target_family = "wasm"), ignore)]
fn test_use_floating_with_real_dom_elements() {
    use leptos::wasm_bindgen::JsCast;
    use leptos::web_sys::*;

    let document = web_sys::window().unwrap().document().unwrap();
    let body = document.body().unwrap();

    // Create a reference button element
    let button = document
        .create_element("button")
        .unwrap()
        .dyn_into::<HtmlButtonElement>()
        .unwrap();

    button.set_inner_text("Test Button");

    // Set styles using web_sys CssStyleDeclaration
    let button_element: &HtmlElement = button.as_ref();
    let button_style = button_element.style();
    button_style.set_property("position", "absolute").unwrap();
    button_style.set_property("left", "100px").unwrap();
    button_style.set_property("top", "200px").unwrap();
    button_style.set_property("width", "120px").unwrap();
    button_style.set_property("height", "40px").unwrap();

    // Create a floating div element
    let div = document
        .create_element("div")
        .unwrap()
        .dyn_into::<HtmlDivElement>()
        .unwrap();

    div.set_inner_text("Floating Content");
    let div_element: &HtmlElement = div.as_ref();
    let div_style = div_element.style();
    div_style.set_property("position", "absolute").unwrap();

    // Add elements to DOM so getBoundingClientRect works
    body.append_child(&button).unwrap();
    body.append_child(&div).unwrap();

    // Test positioning calculation
    let options = FloatingOptions {
        side: Side::Bottom,
        align: Align::Start,
        side_offset: 4.0,
        align_offset: 0.0,
    };

    // Test the actual positioning calculation with real DOM element
    let position = calculate_position(&button, &div, options).unwrap();

    // Expected position should be:
    // x = button.left = 100
    // y = button.top + button.height + side_offset = 200 + 40 + 4 = 244

    assert_eq!(position.x, 100.0, "Floating should be positioned at x=100");
    assert_eq!(position.y, 244.0, "Floating should be positioned at y=244");
    assert_eq!(position.side, Side::Bottom);
    assert_eq!(position.align, Align::Start);

    // Clean up
    body.remove_child(&button).unwrap();
    body.remove_child(&div).unwrap();
}

#[wasm_bindgen_test(unsupported = test)]
#[cfg_attr(not(target_family = "wasm"), ignore)]
fn test_calculate_position_from_rect_in_browser() {
    // Test our positioning function with known values
    let options = FloatingOptions {
        side: Side::Bottom,
        align: Align::Start,
        side_offset: 4.0,
        align_offset: 0.0,
    };

    // floating_width=80, floating_height=30 (not used for Align::Start)
    let position =
        calculate_position_from_rect(100.0, 200.0, 120.0, 40.0, 80.0, 30.0, options).unwrap();

    assert_eq!(position.x, 100.0); // reference_x
    assert_eq!(position.y, 244.0); // reference_y + height + side_offset
    assert_eq!(position.side, Side::Bottom);
    assert_eq!(position.align, Align::Start);
}
