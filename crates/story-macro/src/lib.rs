mod registration_generator;
mod story_generator;

use proc_macro::TokenStream as TS1;
use quote::quote;
use syn::{ItemConst, Meta, Token, parse, parse::Parser, punctuated::Punctuated};

use crate::{registration_generator::RegistrationGenerator, story_generator::StoryGenerator};

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
/// use holt_story_macro::story;
///
/// /// Buttons are for clicking and doing button things
/// #[story(id = "my-story", name = "My Story")]
/// const MY_STORY: &[&StoryVariant] = &[
///     &StoryVariant {
///         name: "Default",
///         view: || view! { <button>"Click me!"</button> }.into_any(),
///     },
/// ];
/// ```
#[proc_macro_attribute]
pub fn story(args: TS1, body: TS1) -> TS1 {
    let parsed_body: ItemConst = parse(body.clone()).unwrap();
    let parsed_args: Punctuated<Meta, Token![,]> =
        Punctuated::parse_terminated.parse(args).unwrap();

    let story_generator = StoryGenerator::new(parsed_args, parsed_body);
    let inventory_generator = RegistrationGenerator::new(&story_generator);

    let original_const = &story_generator.const_item;
    let full_story_const = story_generator.full_story_const();
    let submit_story_to_inventory = inventory_generator.submit_story();

    let output = quote! {
        #original_const

        #full_story_const

        #submit_story_to_inventory
    };

    output.into()
}
