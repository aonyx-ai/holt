use proc_macro2::TokenStream;
use quote::quote;
use std::io::Write;
use std::process::Command;
use syn::{Ident, Item, ItemFn};

pub(crate) const VARIANT_PREFIX: &str = "VARIANT_";

pub(crate) struct VariantGenerator {
    pub(crate) function: ItemFn,
    pub(crate) variant_name: String,
    pub(crate) const_name: Ident,
}

impl VariantGenerator {
    pub fn new(function: ItemFn) -> Self {
        let variant_name = Self::function_name_to_variant_name(&function.sig.ident);
        let const_name = story_variant_fn_as_const_name(&function.sig.ident);

        Self {
            function,
            variant_name,
            const_name,
        }
    }

    /// Converts a snake_case function name to a Title Case variant name
    fn function_name_to_variant_name(ident: &Ident) -> String {
        let name = ident.to_string();
        name.split('_')
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            })
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn generate_variant_const(&self) -> TokenStream {
        let const_name = &self.const_name;
        let variant_name = &self.variant_name;
        let function_body = &self.function.block;
        let source_code = self.generate_source_code();

        quote! {
            const #const_name: &holt_book::StoryVariant = &holt_book::StoryVariant::builder()
                .name(#variant_name)
                .render(|| #function_body)
                .source(#source_code)
                .build();
        }
    }

    /// Generates the source code of the variant
    ///
    /// Tries to format the source with `leptosfmt --rustfmt` and falls back to the original if that fails.
    fn generate_source_code(&self) -> String {
        let source = prettyplease::unparse(&syn::File {
            shebang: None,
            attrs: vec![],
            items: vec![Item::Fn(self.function.clone())],
        });

        self.format_with_leptosfmt(&source).unwrap_or(source)
    }

    /// Formats the source with `leptosfmt --rustfmt`
    ///
    /// Writes the source to stdin and waits for the command to complete.
    fn format_with_leptosfmt(&self, source: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut cmd = Command::new("leptosfmt");
        cmd.arg("--rustfmt").arg("--stdin");

        let mut child = cmd
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()?;

        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(source.as_bytes())?;
        }

        let output = child.wait_with_output()?;

        if output.status.success() {
            let formatted = String::from_utf8(output.stdout)?;
            Ok(formatted)
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(format!("leptosfmt failed: {}", stderr).into())
        }
    }
}

/// Converts function name to a constant name for a story variant
///
/// e.g. `default` becomes `VARIANT_DEFAULT`
pub(crate) fn story_variant_fn_as_const_name(ident: &Ident) -> Ident {
    let name = ident.to_string().to_uppercase();
    Ident::new(&format!("{VARIANT_PREFIX}{name}"), ident.span())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::not_send_sync::{AmbiguousIfSend, AmbiguousIfSync};
    use syn::parse_quote;

    fn destructive_count_generator() -> VariantGenerator {
        let function: ItemFn = parse_quote! {
            fn destructive_count() -> AnyView {
                view! { <Badge variant=BadgeVariant::Destructive>99</Badge> }.into_any()
            }
        };

        VariantGenerator::new(function)
    }

    fn generated_default_const() -> String {
        let function: ItemFn = parse_quote! {
            fn default() -> AnyView {
                view! { <Badge>Default</Badge> }.into_any()
            }
        };

        VariantGenerator::new(function)
            .generate_variant_const()
            .to_string()
    }

    /// Returns the source code of a badly formatted variant after formatting
    ///
    /// The source code should be formatted (no excessive whitespace). This verifies that leptosfmt
    /// is called, even if it falls back to prettyplease.
    fn formatted_source() -> String {
        let function: ItemFn = parse_quote! {
            fn badly_formatted() -> AnyView {
                let x    =    5;
                view! { <Button class="w-32"     on:click=move |_| {
                    println!("clicked");
                }>
                    "Test"
                </Button> }.into_any()
            }
        };

        VariantGenerator::new(function).generate_source_code()
    }

    fn variant_name_of(name: &str) -> String {
        let ident = Ident::new(name, proc_macro2::Span::call_site());

        VariantGenerator::function_name_to_variant_name(&ident)
    }

    fn const_name_of(name: &str) -> String {
        let ident = Ident::new(name, proc_macro2::Span::call_site());

        story_variant_fn_as_const_name(&ident).to_string()
    }

    /// Calls the `format_with_leptosfmt` method directly
    ///
    /// This either succeeds with leptosfmt, and then the source should be formatted, or fails
    /// gracefully. A failure is acceptable because `generate_source_code` falls back to prettyplease.
    #[test]
    fn test_format_with_leptosfmt_fallback() {
        let function: ItemFn = parse_quote! {
            fn simple() -> AnyView {
                view! { <div>"Hello"</div> }.into_any()
            }
        };
        let generator = VariantGenerator::new(function);
        let test_source = "fn test() { let x    =    5; }";

        let result = generator.format_with_leptosfmt(test_source);

        if let Ok(formatted) = result {
            assert!(!formatted.contains("let x    =    5") || formatted.contains("let x = 5"));
        }
    }

    #[test]
    fn test_function_name_to_const_name_converts_default() {
        let result = const_name_of("default");

        assert_eq!(result, "VARIANT_DEFAULT");
    }

    #[test]
    fn test_function_name_to_const_name_converts_destructive_count() {
        let result = const_name_of("destructive_count");

        assert_eq!(result, "VARIANT_DESTRUCTIVE_COUNT");
    }

    #[test]
    fn test_function_name_to_variant_name_converts_default() {
        let result = variant_name_of("default");

        assert_eq!(result, "Default");
    }

    #[test]
    fn test_function_name_to_variant_name_converts_destructive_count() {
        let result = variant_name_of("destructive_count");

        assert_eq!(result, "Destructive Count");
    }

    #[test]
    fn test_function_name_to_variant_name_converts_outline_with_icon() {
        let result = variant_name_of("outline_with_icon");

        assert_eq!(result, "Outline With Icon");
    }

    /// Checks the structure of the generated constant
    ///
    /// The tests below cannot easily check exact token equality because of formatting differences,
    /// but they can verify that the structure is correct.
    #[test]
    fn test_generate_variant_const_contains_const_name() {
        let result = generated_default_const();

        assert!(result.contains("VARIANT_DEFAULT"));
    }

    #[test]
    fn test_generate_variant_const_contains_render_closure() {
        let result = generated_default_const();

        assert!(result.contains("render (||"));
    }

    #[test]
    fn test_generate_variant_const_contains_variant_name() {
        let result = generated_default_const();

        assert!(result.contains("\"Default\""));
    }

    #[test]
    fn test_leptosfmt_formatting_collapses_attribute_spacing() {
        let source_code = formatted_source();

        assert!(!source_code.contains("class=\"w-32\"     on:click"));
    }

    #[test]
    fn test_leptosfmt_formatting_collapses_let_spacing() {
        let source_code = formatted_source();

        assert!(!source_code.contains("let x    =    5"));
    }

    #[test]
    fn test_leptosfmt_formatting_keeps_button() {
        let source_code = formatted_source();

        assert!(source_code.contains("Button"));
    }

    #[test]
    fn test_leptosfmt_formatting_keeps_function_name() {
        let source_code = formatted_source();

        assert!(source_code.contains("fn badly_formatted"));
    }

    #[test]
    fn test_leptosfmt_formatting_keeps_let_binding() {
        let source_code = formatted_source();

        assert!(source_code.contains("let x = 5"));
    }

    #[test]
    fn test_variant_generator_new_keeps_function() {
        let generator = destructive_count_generator();

        assert_eq!(generator.function.sig.ident, "destructive_count");
    }

    #[test]
    fn test_variant_generator_new_names_const() {
        let generator = destructive_count_generator();

        assert_eq!(
            generator.const_name.to_string(),
            "VARIANT_DESTRUCTIVE_COUNT"
        );
    }

    #[test]
    fn test_variant_generator_new_names_variant() {
        let generator = destructive_count_generator();

        assert_eq!(generator.variant_name, "Destructive Count");
    }

    /// `VariantGenerator` is not `Send`, because its fields hold `syn` and `proc_macro2` values, which are tied to the compiler thread
    #[test]
    fn trait_not_send_variant_generator() {
        <VariantGenerator as AmbiguousIfSend<_>>::assert_not_send();
    }

    /// `VariantGenerator` is not `Sync`, because its fields hold `syn` and `proc_macro2` values, which are tied to the compiler thread
    #[test]
    fn trait_not_sync_variant_generator() {
        <VariantGenerator as AmbiguousIfSync<_>>::assert_not_sync();
    }

    #[test]
    fn variant_generator_is_unpin() {
        fn assert_unpin<T: Unpin>() {}

        assert_unpin::<VariantGenerator>();
    }
}
