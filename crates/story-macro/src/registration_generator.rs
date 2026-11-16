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
        let full_story_name = self.story_generator.story_const_name();

        quote! {
            holt_book::submit!(#full_story_name);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::not_send_sync::{AmbiguousIfSend, AmbiguousIfSync};

    #[test]
    fn registration_generator_is_unpin() {
        fn assert_unpin<T: Unpin>() {}

        assert_unpin::<RegistrationGenerator<'static>>();
    }

    /// `RegistrationGenerator` is not `Send`, because it borrows a `StoryGenerator`, and that holds `syn` and `proc_macro2` values tied to the compiler thread
    #[test]
    fn trait_not_send_registration_generator() {
        <RegistrationGenerator<'static> as AmbiguousIfSend<_>>::assert_not_send();
    }

    /// `RegistrationGenerator` is not `Sync`, because it borrows a `StoryGenerator`, and that holds `syn` and `proc_macro2` values tied to the compiler thread
    #[test]
    fn trait_not_sync_registration_generator() {
        <RegistrationGenerator<'static> as AmbiguousIfSync<_>>::assert_not_sync();
    }
}
