#![feature(proc_macro_hygiene)]

extern crate proc_macro;
use quote::quote;
use syn::DeriveInput;

#[proc_macro_derive(ToData)]
pub fn to_data_macro_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let name = match syn::parse(input)
        .map(|ast: DeriveInput| ast.ident.clone())
        .map_err(|e| e.to_compile_error())
    {
        Ok(name) => name,
        Err(err) => return err.into(),
    };
    quote! {
        use serde::{Deserialize, Serialize};

        impl pqpfs::traits::PlainBytes for #name {}
        impl pqpfs::data::ToData for #name {}
    }
    .into()
}
