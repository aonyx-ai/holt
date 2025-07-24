use syn::{Expr, Lit, Meta, Path, Token, punctuated::Punctuated};

pub(crate) struct StoryMacroArgs {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) extra_docs: Option<Path>,
}

impl StoryMacroArgs {
    pub(crate) fn new(args: Punctuated<Meta, Token![,]>) -> Self {
        let mut story_id: Option<String> = None;
        let mut story_name: Option<String> = None;
        let mut extra_docs: Option<Path> = None;

        for arg in args {
            if let Meta::NameValue(nv) = arg {
                if let Expr::Lit(expr) = &nv.value
                    && let Lit::Str(lit) = &expr.lit
                    && nv.path.is_ident("id")
                {
                    story_id = Some(lit.value());
                } else if let Expr::Lit(expr) = &nv.value
                    && let Lit::Str(lit) = &expr.lit
                    && nv.path.is_ident("name")
                {
                    story_name = Some(lit.value());
                } else if let Expr::Path(path) = &nv.value
                    && nv.path.is_ident("extra_docs")
                {
                    extra_docs = Some(path.path.clone());
                }
            }
        }

        Self {
            id: story_id.expect("story macro requires id attribute"),
            name: story_name.expect("story macro requires name attribute"),
            extra_docs,
        }
    }
}

#[cfg(test)]
mod test {
    use syn::parse_quote;

    use crate::StoryMacroArgs;

    #[test]
    fn test_parse_attributes_with_valid_id_and_name() {
        let args = parse_quote! { id = "test_id", name = "Test Story" };
        let StoryMacroArgs {
            id,
            name,
            extra_docs,
        } = StoryMacroArgs::new(args);

        assert_eq!(id, "test_id");
        assert_eq!(name, "Test Story");
        assert!(extra_docs.is_none())
    }

    #[test]
    fn test_parse_attributes_with_extra_docs() {
        let args = parse_quote! { id = "test_id", name = "Test Story", extra_docs = FULL };
        let StoryMacroArgs {
            id,
            name,
            extra_docs,
        } = StoryMacroArgs::new(args);

        assert_eq!(id, "test_id");
        assert_eq!(name, "Test Story");
        assert!(
            extra_docs.is_some_and(|p| { p.segments.get(0).is_some_and(|s| s.ident == "FULL") })
        )
    }

    #[test]
    #[should_panic(expected = "story macro requires id attribute")]
    fn test_parse_attributes_missing_id() {
        let args = parse_quote! { name = "Test Story" };
        StoryMacroArgs::new(args);
    }

    #[test]
    #[should_panic(expected = "story macro requires name attribute")]
    fn test_parse_attributes_missing_name() {
        let args = parse_quote! { id = "test_id" };
        StoryMacroArgs::new(args);
    }
}
