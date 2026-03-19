use std::panic;

use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Expr, ExprArray, ItemConst};

use crate::{
    doc_comments::DocComments, story_macro_args::StoryMacroArgs,
    variant_generator::story_variant_fn_as_const_name,
};

pub(crate) const STORY_PREFIX: &str = "STORY_";
pub(crate) const STORY_DOCS_POSTFIX: &str = "_DOCUMENTATION";

pub(crate) struct StoryGenerator {
    pub(crate) original_body: ItemConst,
    args: StoryMacroArgs,
    documentation: DocComments,
    variant_names: Vec<Ident>,
}

impl StoryGenerator {
    pub fn new(args: StoryMacroArgs, original_body: ItemConst) -> Self {
        let documentation = DocComments::extract_from_item_const(&original_body);
        let variant_names = parse_variant_names(&original_body);

        Self {
            args,
            original_body,
            documentation,
            variant_names,
        }
    }

    pub fn full_story_const(&self) -> TokenStream {
        let full_story_name = self.story_const_name();
        let story_id = &self.args.id;
        let story_name = &self.args.name;
        let documentation_const_name = self.get_story_documentation_const_name();

        let variant_consts: Vec<Ident> = self
            .variant_names
            .iter()
            .map(story_variant_fn_as_const_name)
            .collect();

        let full_documentation = match (&self.documentation, &self.args.extra_docs) {
            (DocComments::Some(desc), Some(docs)) => quote! {
                Some(holt_book::concatcp!(#desc, "\n", #docs))
            },
            (DocComments::Some(desc), None) => quote! { Some(#desc) },
            (DocComments::None, Some(docs)) => quote! {
                Some(#docs)
            },
            (DocComments::None, None) => quote! { None },
        };

        quote! {
            const #documentation_const_name: Option<&'static str> = #full_documentation;

            const #full_story_name: &'static holt_book::Story = &holt_book::Story {
                id: #story_id,
                name: #story_name,
                description: #documentation_const_name,
                variants: &[
                    #(#variant_consts),*
                ],
            };
        }
    }

    pub fn story_const_name(&self) -> Ident {
        let const_name = &self.original_body.ident;
        Ident::new(&format!("{STORY_PREFIX}{const_name}"), const_name.span())
    }

    fn get_story_documentation_const_name(&self) -> Ident {
        Ident::new(
            &format!("{}{STORY_DOCS_POSTFIX}", self.story_const_name()),
            self.original_body.ident.span(),
        )
    }
}

/// Parse story variant names from a const expression
///
/// Takes a const expression representing a (reference to an) array of variant
/// names, and returns a vector of Idents of the variant names.
fn parse_variant_names(body: &ItemConst) -> Vec<Ident> {
    let array = match &*body.expr {
        Expr::Array(array) => array,
        Expr::Reference(ref_expr) => {
            let Expr::Array(array) = &*ref_expr.expr else {
                return Vec::new();
            };
            array
        }
        _ => return Vec::new(),
    };
    extract_variant_names_from_array(array)
}

fn extract_variant_names_from_array(array: &ExprArray) -> Vec<Ident> {
    let mut variant_names = Vec::new();

    for elem in &array.elems {
        // Each element should be a function name (identifier)
        if let Expr::Path(path) = elem
            && let Some(ident) = path.path.get_ident()
        {
            variant_names.push(ident.clone());
        } else {
            panic!("expected function name");
        }
    }

    variant_names
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_parse_variant_names_from_array() {
        let const_item: ItemConst = parse_quote! {
            const TEST_VARIANTS: () = &[
                default,
                secondary,
            ];
        };

        let variant_names = parse_variant_names(&const_item);
        assert_eq!(variant_names.len(), 2);
        assert_eq!(variant_names[0].to_string(), "default");
        assert_eq!(variant_names[1].to_string(), "secondary");
    }

    #[test]
    fn test_full_story_name() {
        let args = StoryMacroArgs {
            id: "test_id".to_string(),
            name: "Test Story".to_string(),
            extra_docs: None,
        };
        let body: ItemConst = parse_quote! {
            const TEST_VARIANTS: () = &[foobar];
        };

        let generator = StoryGenerator::new(args, body);
        let full_name = generator.story_const_name();

        assert_eq!(full_name.to_string(), "STORY_TEST_VARIANTS");
    }

    #[test]
    fn test_story_generator_new() {
        let args = StoryMacroArgs {
            id: "test_id".to_string(),
            name: "Test Story".to_string(),
            extra_docs: None,
        };
        let body: ItemConst = parse_quote! {
            /// Test description
            const TEST_VARIANTS: () = &[default];
        };

        let generator = StoryGenerator::new(args, body);

        assert_eq!(generator.args.id, "test_id");
        assert_eq!(generator.args.name, "Test Story");
        assert_eq!(
            generator.documentation,
            DocComments::Some("Test description".to_string())
        );
        assert_eq!(generator.original_body.ident, "TEST_VARIANTS");
        assert_eq!(generator.variant_names.len(), 1);
        assert_eq!(generator.variant_names[0].to_string(), "default");
    }
}
