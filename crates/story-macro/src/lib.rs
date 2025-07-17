use proc_macro::TokenStream as TS1;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Expr, Ident, ItemConst, Lit, Meta, Token, parse_macro_input, parse2, punctuated::Punctuated,
};

#[proc_macro_attribute]
pub fn story(args: TS1, body: TS1) -> TS1 {
    let parsed_body: ItemConst = parse2(body.clone().into()).unwrap();
    let args = parse_macro_input!(args with Punctuated::<Meta, Token![,]>::parse_terminated);

    let const_name = &parsed_body.ident;

    // Extract id and name from attribute arguments
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

    // Extract description from doc comments
    let mut docs = Vec::new();

    for attr in parsed_body.attrs.iter() {
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

    let description = if docs.is_empty() {
        quote! { None }
    } else {
        let combined_docs = docs.join("\n");
        quote! { Some(#combined_docs) }
    };

    // Generate a unique name for the full story
    let full_story_name = Ident::new(&format!("{const_name}_FULL"), const_name.span());

    let ts2_attrs: TokenStream = body.into();

    quote! {
        #ts2_attrs

        const #full_story_name: &'static holt_book::Story = &holt_book::Story {
            id: #story_id,
            name: #story_name,
            description: #description,
            variants: #const_name,
        };

        holt_book::submit!(#full_story_name);
    }
    .into()
}
