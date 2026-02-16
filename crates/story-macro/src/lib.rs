mod doc_comments;
mod registration_generator;
mod story_generator;
mod story_macro_args;
mod variant_generator;

use proc_macro::TokenStream as TS1;
use quote::quote;
use syn::{ItemConst, ItemFn, Meta, Token, parse, parse::Parser, punctuated::Punctuated};

use crate::{
    registration_generator::RegistrationGenerator, story_generator::StoryGenerator,
    story_macro_args::StoryMacroArgs, variant_generator::VariantGenerator,
};

/// Story generation macro
///
/// This macro is used to generate a [Story][holt_book::Story] from a constant
/// expression, as well as registering the story with the inventory.
///
/// Define a `const` that's a list of variants (defined with the [variant]
/// macro). You can set the const type to `()`, since we overwrite the whole
/// const to be a [Story][holt_book::Story].
///
/// You have to pass the following args:
///
/// - `id`: The unique identifier for the story, URL-safe.
/// - `name`: The UI name of the story.
///
/// You can optionally document the story using regular Rust doc comments.
/// Additionally, you can set extra documentation (appended at the end) using
/// the `extra_docs` argument. This should be a reference to a `const &'static
/// str`. This is useful for generated documentation: you can generate a static
/// string in `build.rs` and include it with the `include!` macro.
///
/// # Examples
///
/// ```
/// # use leptos::prelude::*;
/// # use holt_book::StoryVariant;
/// use holt_macros::{story, variant};
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
///
/// Or with extra documentation:
///
/// ```
/// # use leptos::prelude::*;
/// # use holt_book::StoryVariant;
/// # use holt_macros::{story, variant};
/// #
/// # #[variant]
/// # fn default() {
/// #     view! { <button>"Click me!"</button> }.into_any()
/// # }
/// #
/// const EXTRA: &str = "Extra documentation for my story";
///
/// /// Buttons are for clicking and doing button things
/// #[story(id = "my-story", name = "My Story", extra_docs = EXTRA)]
/// const MY_STORY: () = &[
///     default,
/// ];
/// ```
#[proc_macro_attribute]
pub fn story(args: TS1, body: TS1) -> TS1 {
    let parsed_body: ItemConst = parse(body).expect("failed to parse body");
    let parsed_args: Punctuated<Meta, Token![,]> = Punctuated::parse_terminated
        .parse(args)
        .expect("failed to parse args");

    let args = StoryMacroArgs::new(parsed_args);

    let story_generator = StoryGenerator::new(args, parsed_body);
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
/// This macro converts a function into a [holt_book::StoryVariant] constant,
/// extracting the variant name from the function name and inlining the function
/// body into the render closure.
///
/// # Examples
///
/// ```
/// # use leptos::prelude::*;
/// use holt_macros::variant;
///
/// #[variant]
/// fn default() -> AnyView {
///     view! { <button>Default</button> }.into_any()
/// }
/// ```
///
/// If you want more control over the generated variant, you should create the
/// [holt_book::StoryVariant] struct directly. Note that you'll need to name the
/// variant `xxx_VARIANT` (like `DEFAULT_VARIANT` for the above) so that you can
/// use it in the [story] macro as `default`.
#[proc_macro_attribute]
pub fn variant(_args: TS1, body: TS1) -> TS1 {
    let parsed_body: ItemFn = parse(body).expect("failed to parse function");

    let variant_generator = VariantGenerator::new(parsed_body);
    let variant_const = variant_generator.generate_variant_const();

    variant_const.into()
}
