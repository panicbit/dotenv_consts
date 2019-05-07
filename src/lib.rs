extern crate proc_macro;
use proc_macro::{TokenStream};
use proc_macro2::{Ident, Span};
use quote::quote;
use std::collections::HashMap;

#[proc_macro]
pub fn dotenv_consts(_item: TokenStream) -> TokenStream {
    let entries = dotenv::dotenv_iter()
        .unwrap()
        .map(|kv| kv.unwrap())
        .collect::<HashMap<_, _>>();
    
    let keys = entries.keys().map(|k| Ident::new(k, Span::call_site()));
    let values = entries.values();

    let output = quote! {
        #(
            pub const #keys: &str = #values;
        )*
    };

    output.into()
}
