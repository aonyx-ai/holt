use crate::StoryGenerator;
use proc_macro2::TokenStream;
use quote::quote;

pub struct RegistrationGenerator<'a> {
    story_generator: &'a StoryGenerator,
}

impl<'a> RegistrationGenerator<'a> {
    pub fn new(story_generator: &'a StoryGenerator) -> Self {
        Self { story_generator }
    }

    pub fn submit_story(&self) -> TokenStream {
        let full_story_name = self.story_generator.full_story_name();

        quote! {
            holt_book::submit!(#full_story_name);
        }
    }
}
