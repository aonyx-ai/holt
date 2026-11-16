use syn::{Expr, Lit, Meta, MetaNameValue, Path, Token, punctuated::Punctuated};

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
            if let Meta::NameValue(MetaNameValue {
                path: key, value, ..
            }) = arg
            {
                if let Expr::Lit(expr) = &value
                    && let Lit::Str(lit) = &expr.lit
                    && key.is_ident("id")
                {
                    story_id = Some(lit.value());
                } else if let Expr::Lit(expr) = &value
                    && let Lit::Str(lit) = &expr.lit
                    && key.is_ident("name")
                {
                    story_name = Some(lit.value());
                } else if let Expr::Path(path) = &value
                    && key.is_ident("extra_docs")
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
    use crate::not_send_sync::{AmbiguousIfSend, AmbiguousIfSync};
    use syn::parse_quote;

    use crate::StoryMacroArgs;

    fn parse_with_extra_docs() -> StoryMacroArgs {
        let args = parse_quote! { id = "test_id", name = "Test Story", extra_docs = FULL };

        StoryMacroArgs::new(args)
    }

    fn parse_with_id_and_name() -> StoryMacroArgs {
        let args = parse_quote! { id = "test_id", name = "Test Story" };

        StoryMacroArgs::new(args)
    }

    #[test]
    fn story_macro_args_is_unpin() {
        fn assert_unpin<T: Unpin>() {}

        assert_unpin::<StoryMacroArgs>();
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

    #[test]
    fn test_parse_attributes_with_extra_docs_keeps_extra_docs() {
        let args = parse_with_extra_docs();

        assert!(
            args.extra_docs
                .is_some_and(|p| { p.segments.first().is_some_and(|s| s.ident == "FULL") })
        )
    }

    #[test]
    fn test_parse_attributes_with_extra_docs_keeps_id() {
        let args = parse_with_extra_docs();

        assert_eq!(args.id, "test_id");
    }

    #[test]
    fn test_parse_attributes_with_extra_docs_keeps_name() {
        let args = parse_with_extra_docs();

        assert_eq!(args.name, "Test Story");
    }

    #[test]
    fn test_parse_attributes_with_valid_id_and_name_has_no_extra_docs() {
        let args = parse_with_id_and_name();

        assert!(args.extra_docs.is_none())
    }

    #[test]
    fn test_parse_attributes_with_valid_id_and_name_keeps_id() {
        let args = parse_with_id_and_name();

        assert_eq!(args.id, "test_id");
    }

    #[test]
    fn test_parse_attributes_with_valid_id_and_name_keeps_name() {
        let args = parse_with_id_and_name();

        assert_eq!(args.name, "Test Story");
    }

    /// `StoryMacroArgs` is not `Send`, because its `extra_docs` path holds `syn` and `proc_macro2` values, which are tied to the compiler thread
    #[test]
    fn trait_not_send_story_macro_args() {
        <StoryMacroArgs as AmbiguousIfSend<_>>::assert_not_send();
    }

    /// `StoryMacroArgs` is not `Sync`, because its `extra_docs` path holds `syn` and `proc_macro2` values, which are tied to the compiler thread
    #[test]
    fn trait_not_sync_story_macro_args() {
        <StoryMacroArgs as AmbiguousIfSync<_>>::assert_not_sync();
    }
}
