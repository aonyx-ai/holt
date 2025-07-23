use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Item, ItemFn};

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

    fn function_name_to_variant_name(ident: &Ident) -> String {
        // Convert snake_case to Title Case
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
            const #const_name: &holt_book::StoryVariant = &holt_book::StoryVariant {
                name: #variant_name,
                render: || #function_body,
                source: #source_code,
            };
        }
    }

    fn generate_source_code(&self) -> String {
        prettyplease::unparse(&syn::File {
            shebang: None,
            attrs: vec![],
            items: vec![Item::Fn(self.function.clone())],
        })
    }
}

/// Converts function name to a constant name for a story variant.
///
/// e.g. `default` becomes `VARIANT_DEFAULT`
pub(crate) fn story_variant_fn_as_const_name(ident: &Ident) -> Ident {
    let name = ident.to_string().to_uppercase();
    Ident::new(&format!("{VARIANT_PREFIX}{name}"), ident.span())
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_function_name_to_variant_name() {
        let ident = Ident::new("default", proc_macro2::Span::call_site());
        let result = VariantGenerator::function_name_to_variant_name(&ident);
        assert_eq!(result, "Default");

        let ident = Ident::new("destructive_count", proc_macro2::Span::call_site());
        let result = VariantGenerator::function_name_to_variant_name(&ident);
        assert_eq!(result, "Destructive Count");

        let ident = Ident::new("outline_with_icon", proc_macro2::Span::call_site());
        let result = VariantGenerator::function_name_to_variant_name(&ident);
        assert_eq!(result, "Outline With Icon");
    }

    #[test]
    fn test_function_name_to_const_name() {
        let ident = Ident::new("default", proc_macro2::Span::call_site());
        let result = story_variant_fn_as_const_name(&ident);
        assert_eq!(result.to_string(), "VARIANT_DEFAULT");

        let ident = Ident::new("destructive_count", proc_macro2::Span::call_site());
        let result = story_variant_fn_as_const_name(&ident);
        assert_eq!(result.to_string(), "VARIANT_DESTRUCTIVE_COUNT");
    }

    #[test]
    fn test_generate_variant_const() {
        let function: ItemFn = parse_quote! {
            fn default() -> AnyView {
                view! { <Badge>Default</Badge> }.into_any()
            }
        };

        let generator = VariantGenerator::new(function);
        let result = generator.generate_variant_const().to_string();

        // Note: We can't easily test exact token equality due to formatting differences,
        // but we can verify the structure is correct
        assert!(result.contains("VARIANT_DEFAULT"));
        assert!(result.contains("\"Default\""));
        assert!(result.contains("render : ||"));
    }

    #[test]
    fn test_variant_generator_new() {
        let function: ItemFn = parse_quote! {
            fn destructive_count() -> AnyView {
                view! { <Badge variant=BadgeVariant::Destructive>99</Badge> }.into_any()
            }
        };

        let generator = VariantGenerator::new(function);

        assert_eq!(generator.variant_name, "Destructive Count");
        assert_eq!(
            generator.const_name.to_string(),
            "VARIANT_DESTRUCTIVE_COUNT"
        );
        assert_eq!(generator.function.sig.ident, "destructive_count");
    }
}
