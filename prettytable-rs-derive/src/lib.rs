extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_derive(TableElem)]
pub fn derive_table_elem(input: TokenStream) -> TokenStream {
    let parsed_input = parse_macro_input!(input as ItemStruct);

    let struct_name = &parsed_input.ident;
    let field = &parsed_input.fields;

    // Get struct field name
    let f_name: Vec<syn::Ident> = field.iter().map(|f| f.ident.clone().unwrap()).collect();
    let f_name_str: Vec<String> = f_name.iter().map(|f| f.to_string()).collect();

    TokenStream::from(quote! {
        impl prettytable::TableElem for #struct_name {
            fn get_field_name() -> Vec<&'static str> {
                vec![#(#f_name_str),*]
            }

            fn get_field(self) -> Vec<String> {
                vec![#(self.#f_name.into()),*]
            }
        }
    })
}