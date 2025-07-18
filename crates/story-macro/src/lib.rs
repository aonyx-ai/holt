mod registration_generator;
mod story_generator;
mod variant_generator;

use proc_macro::TokenStream as TS1;
use quote::quote;
use syn::{ItemConst, ItemFn, Meta, Token, parse, parse::Parser, punctuated::Punctuated};

use crate::{
    registration_generator::RegistrationGenerator, story_generator::StoryGenerator,
    variant_generator::VariantGenerator,
};

/// Story generation macro
///
/// This macro is used to generate a Holt-Book story from a constant expression,
/// as well as registering the story with the inventory.
///
/// # Examples
///
/// ```
/// # use leptos::prelude::*;
/// # use holt_book::StoryVariant;
/// use holt_story_macro::{story, variant};
///
/// #[variant]
/// fn default() {
///     view! { <button>"Click me!"</button> }.into_any()
/// }
///
/// /// Buttons are for clicking and doing button things
/// #[story(id = "my-story", name = "My Story")]
/// const MY_STORY: () = &[
///     default,
/// ];
/// ```
#[proc_macro_attribute]
pub fn story(args: TS1, body: TS1) -> TS1 {
    let parsed_body: ItemConst = parse(body.clone()).expect("failed to parse body");
    let parsed_args: Punctuated<Meta, Token![,]> = Punctuated::parse_terminated
        .parse(args)
        .expect("failed to parse args");

    let story_generator = StoryGenerator::new(parsed_args, parsed_body);
    let inventory_generator = RegistrationGenerator::new(&story_generator);

    let full_story_const = story_generator.full_story_const();
    let submit_story_to_inventory = inventory_generator.submit_story();

    let output = quote! {
        #full_story_const

        #submit_story_to_inventory
    };

    output.into()
}

/// Story variant generation macro
///
/// This macro converts a function into a [StoryVariant] constant, extracting
/// the variant name from the function name and inlining the function body into
/// the render closure.
///
/// # Examples
///
/// ```
/// # use leptos::prelude::*;
/// use holt_story_macro::variant;
///
/// #[variant]
/// fn default() -> AnyView {
///     view! { <button>Default</button> }.into_any()
/// }
/// ```
///
/// If you want more control over the generated variant, you should create the
/// [StoryVariant] struct directly. Note that you'll need to name the variant
/// `xxx_VARIANT` (like `DEFAULT_VARIANT` for the above) so that you can use it
/// in the [story] macro as `default`.
#[proc_macro_attribute]
pub fn variant(_args: TS1, body: TS1) -> TS1 {
    let parsed_body: ItemFn = parse(body.clone()).expect("failed to parse function");

    let variant_generator = VariantGenerator::new(parsed_body);
    let variant_const = variant_generator.generate_variant_const();

    variant_const.into()
}
