use proc_macro::TokenStream as TS1;
use proc_macro2::{TokenStream, TokenTree};
use quote::quote;
use venial::{AttributeValue, Constant, Item, parse_item};

#[proc_macro_attribute]
pub fn story(whatever: TS1, body: TS1) -> TS1 {
    let parsed_body = parse_item(body.clone().into()).unwrap();

    let const_name = if let Item::Constant(Constant { name, .. }) = parsed_body.clone() {
        name
    } else {
        panic!("Expected a constant")
    };

    let mut docs = Vec::new();

    parsed_body.attributes().iter().for_each(|attr| {
        if let Some(TokenTree::Ident(x)) = attr.path.first()
            && *x != "doc"
        {
            return;
        }

        if let AttributeValue::Equals(_, tt) = &attr.value {
            if let Some(TokenTree::Literal(lit)) = tt.first() {
                docs.push(lit.to_string());
            }
        }
    });

    let ts2_ts: TokenStream = whatever.into();
    let ts2_attrs: TokenStream = body.into();

    quote! {
        #ts2_attrs
        #ts2_ts

        holt_book::submit!(#const_name);
    }
    .into()
}
