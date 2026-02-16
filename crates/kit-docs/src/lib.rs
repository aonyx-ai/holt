pub mod stories;

#[cfg(feature = "csr")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn main() {
    holt_book::run_book();
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn hydrate() {
    holt_book::init_for_hydrate();

    console_error_panic_hook::set_once();

    leptos::mount::hydrate_body(move || {
        leptos::view! { <holt_book::App base="/kit" /> }
    });
}
