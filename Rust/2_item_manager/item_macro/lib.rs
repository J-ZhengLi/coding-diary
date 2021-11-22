use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Ident};
use quote::quote;

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput {ident, data, .. } = parse_macro_input!(input);
    println!("{:?}", data);
    
    TokenStream::new()
}