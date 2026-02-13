use holt_kit::visual::Badge;
use leptos::prelude::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test(unsupported = test)]
#[cfg_attr(not(target_family = "wasm"), ignore)]
fn badge_renders_as_span_not_button() {
    let document = web_sys::window().unwrap().document().unwrap();
    let container = document.create_element("div").unwrap();
    document.body().unwrap().append_child(&container).unwrap();

    let _owner = Owner::new();
    _owner.with(|| {
        let view = view! { <Badge>"Test"</Badge> };
        let mut mounted = view.build();
        use leptos::tachys::view::Mountable;
        mounted.mount(&container, None);
    });

    let first_child = container
        .first_element_child()
        .expect("Badge should render an element");

    assert_eq!(
        first_child.tag_name().to_lowercase(),
        "span",
        "Badge root element should be a <span>, not a <button>"
    );

    // Clean up
    document.body().unwrap().remove_child(&container).unwrap();
}
