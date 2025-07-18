use std::panic;

use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Expr, ExprArray, ItemConst, Lit, Meta, Token, punctuated::Punctuated};

pub(crate) struct StoryGenerator {
    pub(crate) const_item: ItemConst,
    story_id: String,
    story_name: String,
    description: Option<String>,
    variant_names: Vec<Ident>,
}

impl StoryGenerator {
    pub fn new(args: Punctuated<Meta, Token![,]>, body: ItemConst) -> Self {
        let (story_id, story_name) = Self::parse_attributes(args);
        let description = parse_doc_comments(&body);
        let variant_names = Self::parse_variant_names(&body);

        Self {
            const_item: body,
            story_id,
            story_name,
            description,
            variant_names,
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

    fn parse_variant_names(body: &ItemConst) -> Vec<Ident> {
        let mut variant_names = Vec::new();

        // Extract the array expression from the const
        if let Expr::Array(array) = &*body.expr {
            variant_names = Self::extract_variant_names_from_array(array);
        } else if let Expr::Reference(ref_expr) = &*body.expr {
            if let Expr::Array(array) = &*ref_expr.expr {
                variant_names = Self::extract_variant_names_from_array(array);
            }
        }

        variant_names
    }

    fn extract_variant_names_from_array(array: &ExprArray) -> Vec<Ident> {
        let mut variant_names = Vec::new();

        for elem in &array.elems {
            // Each element should be a function name (identifier)
            if let Expr::Path(path) = elem {
                if let Some(ident) = path.path.get_ident() {
                    variant_names.push(ident.clone());
                } else {
                    panic!("expected function name");
                }
            } else {
                panic!("expected function name");
            }
        }

        variant_names
    }

    fn function_name_to_const_name(ident: &Ident) -> Ident {
        // Convert snake_case to SCREAMING_SNAKE_CASE_VARIANT
        let name = ident.to_string().to_uppercase();
        Ident::new(&format!("{name}_VARIANT"), ident.span())
    }

    pub fn full_story_const(&self) -> TokenStream {
        let full_story_name = self.full_story_name();
        let story_id = &self.story_id;
        let story_name = &self.story_name;

        let description = match &self.description {
            Some(desc) => quote! { Some(#desc) },
            _ => quote! { None },
        };

        let variant_consts: Vec<Ident> = self
            .variant_names
            .iter()
            .map(Self::function_name_to_const_name)
            .collect();

        quote! {
            const #full_story_name: &'static holt_book::Story = &holt_book::Story {
                id: #story_id,
                name: #story_name,
                description: #description,
                variants: &[
                    #(#variant_consts),*
                ],
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

// Note: This function is no longer used since we're not extracting doc comments from array elements
// fn extract_doc_comment_from_expr(expr: &Expr) -> Option<String> {
//     // In the AST, doc comments on array elements are typically attached
//     // to the expression itself. We need to look for attributes on the expression.
//     match expr {
//         Expr::Closure(closure) => {
//             // Check if the closure has attributes (doc comments)
//             for attr in &closure.attrs {
//                 if let Meta::NameValue(meta) = &attr.meta {
//                     if attr.meta.path().is_ident("doc") {
//                         if let Expr::Lit(expr) = &meta.value {
//                             if let Lit::Str(lit) = &expr.lit {
//                                 return Some(lit.value().trim().to_string());
//                             }
//                         }
//                     }
//                 }
//             }
//             None
//         }
//         _ => None,
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
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
    fn test_parse_doc_comments_with_single_line() {
        let const_item: ItemConst = parse_quote! {
            /// This is a test description
            const TEST_VARIANTS: () = &[|| {}];
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
            const TEST_VARIANTS: () = &[|| {}];
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
    fn test_parse_variant_names_from_array() {
        let const_item: ItemConst = parse_quote! {
            const TEST_VARIANTS: () = &[
                default,
                secondary,
            ];
        };

        let variant_names = StoryGenerator::parse_variant_names(&const_item);
        assert_eq!(variant_names.len(), 2);
        assert_eq!(variant_names[0].to_string(), "default");
        assert_eq!(variant_names[1].to_string(), "secondary");
    }

    #[test]
    fn test_full_story_name() {
        let args = parse_quote! { id = "test_id", name = "Test Story" };
        let body: ItemConst = parse_quote! {
            const TEST_VARIANTS: () = &[foobar];
        };

        let generator = StoryGenerator::new(args, body);
        let full_name = generator.full_story_name();

        assert_eq!(full_name.to_string(), "TEST_VARIANTS_FULL");
    }

    #[test]
    fn test_story_generator_new() {
        let args = parse_quote! { id = "test_id", name = "Test Story" };
        let body: ItemConst = parse_quote! {
            /// Test description
            const TEST_VARIANTS: () = &[default];
        };

        let generator = StoryGenerator::new(args, body);

        assert_eq!(generator.story_id, "test_id");
        assert_eq!(generator.story_name, "Test Story");
        assert_eq!(generator.description, Some("Test description".to_string()));
        assert_eq!(generator.const_item.ident, "TEST_VARIANTS");
        assert_eq!(generator.variant_names.len(), 1);
        assert_eq!(generator.variant_names[0].to_string(), "default");
    }

    #[test]
    fn test_function_name_to_const_name() {
        let ident = Ident::new("default", proc_macro2::Span::call_site());
        let result = StoryGenerator::function_name_to_const_name(&ident);
        assert_eq!(result.to_string(), "DEFAULT_VARIANT");

        let ident = Ident::new("destructive_count", proc_macro2::Span::call_site());
        let result = StoryGenerator::function_name_to_const_name(&ident);
        assert_eq!(result.to_string(), "DESTRUCTIVE_COUNT_VARIANT");
    }
}
