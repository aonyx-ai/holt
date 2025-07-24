use syn::{Expr, ItemConst, Lit, Meta};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum DocComments {
    Some(String),
    None,
}

impl DocComments {
    pub fn extract_from_item_const(body: &ItemConst) -> Self {
        let mut docs = Vec::new();

        for attr in body.attrs.iter() {
            if let Meta::NameValue(meta) = &attr.meta {
                if !attr.meta.path().is_ident("doc") {
                    continue;
                }

                if let Expr::Lit(expr) = &meta.value
                    && let Lit::Str(lit) = &expr.lit
                {
                    docs.push(lit.value().trim().to_string());
                }
            }
        }

        if docs.is_empty() {
            return Self::None;
        }

        Self::Some(docs.join("\n"))
    }
}

#[cfg(test)]
mod test {
    use syn::{ItemConst, parse_quote};

    use crate::doc_comments::DocComments;

    #[test]
    fn test_parse_doc_comments_with_single_line() {
        let const_item: ItemConst = parse_quote! {
            /// This is a test description
            const TEST_VARIANTS: () = &[];
        };

        let description = DocComments::extract_from_item_const(&const_item);
        assert_eq!(
            description,
            DocComments::Some("This is a test description".to_string())
        );
    }

    #[test]
    fn test_parse_doc_comments_with_multiple_lines() {
        let const_item: ItemConst = parse_quote! {
            /// First line of description
            /// Second line of description
            /// Third line of description
            const TEST_VARIANTS: () = &[];
        };

        let description = DocComments::extract_from_item_const(&const_item);
        assert_eq!(
            description,
            DocComments::Some(
                "First line of description\nSecond line of description\nThird line of description"
                    .to_string()
            )
        );
    }
}
