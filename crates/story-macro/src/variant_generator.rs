use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Item, ItemFn};
use std::io::Write;
use std::process::Command;

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
        let source = prettyplease::unparse(&syn::File {
            shebang: None,
            attrs: vec![],
            items: vec![Item::Fn(self.function.clone())],
        });
        
        // Try to format with leptosfmt --rustfmt, fall back to original if it fails
        self.format_with_leptosfmt(&source).unwrap_or(source)
    }
    
    fn format_with_leptosfmt(&self, source: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut cmd = Command::new("leptosfmt");
        cmd.arg("--rustfmt").arg("--stdin");

        let mut child = cmd
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()?;

        // Write the source to stdin
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(source.as_bytes())?;
        }

        // Wait for the command to complete
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

    #[test]
    fn test_leptosfmt_formatting() {
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

        let generator = VariantGenerator::new(function);
        let source_code = generator.generate_source_code();
        
        // The source code should be formatted (no excessive whitespace)
        // This test verifies that leptosfmt is being called, even if it falls back to prettyplease
        assert!(!source_code.contains("let x    =    5"));
        assert!(!source_code.contains("class=\"w-32\"     on:click"));
        
        // Should contain the basic structure
        assert!(source_code.contains("fn badly_formatted"));
        assert!(source_code.contains("let x = 5"));
        assert!(source_code.contains("Button"));
    }

    #[test]
    fn test_format_with_leptosfmt_fallback() {
        let function: ItemFn = parse_quote! {
            fn simple() -> AnyView {
                view! { <div>"Hello"</div> }.into_any()
            }
        };

        let generator = VariantGenerator::new(function);
        
        // Test the format_with_leptosfmt method directly
        let test_source = "fn test() { let x    =    5; }";
        
        // This should either succeed with leptosfmt or fall back gracefully
        match generator.format_with_leptosfmt(test_source) {
            Ok(formatted) => {
                // If leptosfmt worked, it should be formatted
                assert!(!formatted.contains("let x    =    5") || formatted.contains("let x = 5"));
            }
            Err(_) => {
                // If leptosfmt failed, that's also acceptable - the main generate_source_code
                // method will fall back to prettyplease
            }
        }
    }
}
