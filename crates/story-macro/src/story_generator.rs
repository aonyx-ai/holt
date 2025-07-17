use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Expr, ItemConst, Lit, Meta, Token, punctuated::Punctuated};

pub(crate) struct StoryGenerator {
    pub(crate) const_item: ItemConst,
    story_id: String,
    story_name: String,
    description: Option<String>,
}

impl StoryGenerator {
    pub fn new(args: Punctuated<Meta, Token![,]>, body: ItemConst) -> Self {
        let (story_id, story_name) = Self::parse_attributes(args);
        let description = parse_doc_comments(&body);

        Self {
            const_item: body,
            story_id,
            story_name,
            description,
        }
    }

    fn parse_attributes(args: Punctuated<Meta, Token![,]>) -> (String, String) {
        let mut story_id: Option<String> = None;
        let mut story_name: Option<String> = None;

        for arg in args {
            if let Meta::NameValue(nv) = arg {
                if nv.path.is_ident("id") {
                    if let Expr::Lit(expr) = &nv.value {
                        if let Lit::Str(lit) = &expr.lit {
                            story_id = Some(lit.value());
                        }
                    }
                } else if nv.path.is_ident("name") {
                    if let Expr::Lit(expr) = &nv.value {
                        if let Lit::Str(lit) = &expr.lit {
                            story_name = Some(lit.value());
                        }
                    }
                }
            }
        }

        let story_id = story_id.expect("story macro requires id attribute");
        let story_name = story_name.expect("story macro requires name attribute");

        (story_id, story_name)
    }

    pub fn full_story_const(&self) -> TokenStream {
        let full_story_name = self.full_story_name();
        let story_id = &self.story_id;
        let story_name = &self.story_name;
        let variants = &self.const_item.ident;

        let description = match &self.description {
            Some(desc) => quote! { Some(#desc) },
            _ => quote! { None },
        };

        quote! {
            const #full_story_name: &'static holt_book::Story = &holt_book::Story {
                id: #story_id,
                name: #story_name,
                description: #description,
                variants: #variants,
            };
        }
    }

    pub fn full_story_name(&self) -> Ident {
        let const_name = &self.const_item.ident;
        Ident::new(&format!("{const_name}_FULL"), const_name.span())
    }
}

fn parse_doc_comments(body: &ItemConst) -> Option<String> {
    let mut docs = Vec::new();

    for attr in body.attrs.iter() {
        if let Meta::NameValue(meta) = &attr.meta {
            if !attr.meta.path().is_ident("doc") {
                continue;
            }

            if let Expr::Lit(expr) = &meta.value {
                if let Lit::Str(lit) = &expr.lit {
                    docs.push(lit.value().trim().to_string());
                }
            }
        }
    }

    if docs.is_empty() {
        None
    } else {
        Some(docs.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;
    use syn::parse_quote;

    #[test]
    fn test_parse_attributes_with_valid_id_and_name() {
        let args = parse_quote! { id = "test_id", name = "Test Story" };
        let (story_id, story_name) = StoryGenerator::parse_attributes(args);

        assert_eq!(story_id, "test_id");
        assert_eq!(story_name, "Test Story");
    }

    #[test]
    #[should_panic(expected = "story macro requires id attribute")]
    fn test_parse_attributes_missing_id() {
        let args = parse_quote! { name = "Test Story" };
        StoryGenerator::parse_attributes(args);
    }

    #[test]
    #[should_panic(expected = "story macro requires name attribute")]
    fn test_parse_attributes_missing_name() {
        let args = parse_quote! { id = "test_id" };
        StoryGenerator::parse_attributes(args);
    }

    #[test]
    fn test_parse_attributes_with_extra_attributes() {
        let args = parse_quote! { id = "test_id", name = "Test Story", extra = "ignored" };
        let (story_id, story_name) = StoryGenerator::parse_attributes(args);

        assert_eq!(story_id, "test_id");
        assert_eq!(story_name, "Test Story");
    }

    #[test]
    fn test_parse_doc_comments_with_single_line() {
        let const_item: ItemConst = parse_quote! {
            /// This is a test description
            const TEST_VARIANTS: &[&str] = &["variant1", "variant2"];
        };

        let description = parse_doc_comments(&const_item);
        assert_eq!(description, Some("This is a test description".to_string()));
    }

    #[test]
    fn test_parse_doc_comments_with_multiple_lines() {
        let const_item: ItemConst = parse_quote! {
            /// First line of description
            /// Second line of description
            /// Third line of description
            const TEST_VARIANTS: &[&str] = &["variant1", "variant2"];
        };

        let description = parse_doc_comments(&const_item);
        assert_eq!(
            description,
            Some(
                "First line of description\nSecond line of description\nThird line of description"
                    .to_string()
            )
        );
    }

    #[test]
    fn test_parse_doc_comments_with_no_docs() {
        let const_item: ItemConst = parse_quote! {
            const TEST_VARIANTS: &[&str] = &["variant1", "variant2"];
        };

        let description = parse_doc_comments(&const_item);
        assert_eq!(description, None);
    }

    #[test]
    fn test_parse_doc_comments_with_mixed_attributes() {
        let const_item: ItemConst = parse_quote! {
            /// This is documentation
            #[allow(dead_code)]
            /// More documentation
            const TEST_VARIANTS: &[&str] = &["variant1", "variant2"];
        };

        let description = parse_doc_comments(&const_item);
        assert_eq!(
            description,
            Some("This is documentation\nMore documentation".to_string())
        );
    }

    #[test]
    fn test_parse_doc_comments_with_whitespace() {
        let const_item: ItemConst = parse_quote! {
            ///   Whitespace at beginning
            ///	Tab and trailing space
            const TEST_VARIANTS: &[&str] = &["variant1", "variant2"];
        };

        let description = parse_doc_comments(&const_item);
        assert_eq!(
            description,
            Some("Whitespace at beginning\nTab and trailing space".to_string())
        );
    }

    #[test]
    fn test_story_generator_new() {
        let args = parse_quote! { id = "test_id", name = "Test Story" };
        let body: ItemConst = parse_quote! {
            /// Test description
            const TEST_VARIANTS: &[&str] = &["variant1", "variant2"];
        };

        let generator = StoryGenerator::new(args, body);

        assert_eq!(generator.story_id, "test_id");
        assert_eq!(generator.story_name, "Test Story");
        assert_eq!(generator.description, Some("Test description".to_string()));
        assert_eq!(generator.const_item.ident, "TEST_VARIANTS");
    }

    #[test]
    fn test_story_generator_new_without_description() {
        let args = parse_quote! { id = "test_id", name = "Test Story" };
        let body: ItemConst = parse_quote! {
            const TEST_VARIANTS: &[&str] = &["variant1", "variant2"];
        };

        let generator = StoryGenerator::new(args, body);

        assert_eq!(generator.story_id, "test_id");
        assert_eq!(generator.story_name, "Test Story");
        assert_eq!(generator.description, None);
    }

    #[test]
    fn test_full_story_name() {
        let args = parse_quote! { id = "test_id", name = "Test Story" };
        let body: ItemConst = parse_quote! {
            const TEST_VARIANTS: &[&str] = &["variant1", "variant2"];
        };

        let generator = StoryGenerator::new(args, body);
        let full_name = generator.full_story_name();

        assert_eq!(full_name.to_string(), "TEST_VARIANTS_FULL");
    }

    #[test]
    fn test_full_story_const_with_description() {
        let args = parse_quote! { id = "test_id", name = "Test Story" };
        let body: ItemConst = parse_quote! {
            /// This is a test story
            const TEST_VARIANTS: &[&str] = &["variant1", "variant2"];
        };

        let generator = StoryGenerator::new(args, body);
        let const_tokens = generator.full_story_const();

        let expected = quote! {
            const TEST_VARIANTS_FULL: &'static holt_book::Story = &holt_book::Story {
                id: "test_id",
                name: "Test Story",
                description: Some("This is a test story"),
                variants: TEST_VARIANTS,
            };
        };

        assert_eq!(const_tokens.to_string(), expected.to_string());
    }

    #[test]
    fn test_full_story_const_without_description() {
        let args = parse_quote! { id = "test_id", name = "Test Story" };
        let body: ItemConst = parse_quote! {
            const TEST_VARIANTS: &[&str] = &["variant1", "variant2"];
        };

        let generator = StoryGenerator::new(args, body);
        let const_tokens = generator.full_story_const();

        let expected = quote! {
            const TEST_VARIANTS_FULL: &'static holt_book::Story = &holt_book::Story {
                id: "test_id",
                name: "Test Story",
                description: None,
                variants: TEST_VARIANTS,
            };
        };

        assert_eq!(const_tokens.to_string(), expected.to_string());
    }

    #[test]
    fn test_full_story_const_with_special_characters() {
        let args = parse_quote! { id = "special-id_123", name = "Story with \"quotes\" & symbols" };
        let body: ItemConst = parse_quote! {
            /// Description with "quotes" and \n newlines
            const SPECIAL_VARIANTS: &[&str] = &["variant1"];
        };

        let generator = StoryGenerator::new(args, body);
        let const_tokens = generator.full_story_const();

        // Test that it compiles without panicking and contains expected parts
        let token_string = dbg!(const_tokens.to_string());
        assert!(token_string.contains("special-id_123"));
        assert!(token_string.contains("Story with \\\"quotes\\\" & symbols"));
        assert!(token_string.contains("SPECIAL_VARIANTS_FULL"));
    }

    #[test]
    fn test_empty_doc_comments() {
        let const_item: ItemConst = parse_quote! {
            ///
            ///
            ///
            const TEST_VARIANTS: &[&str] = &["variant1"];
        };

        let description = parse_doc_comments(&const_item);
        assert_eq!(description, Some("\n\n".to_string()));
    }
}
