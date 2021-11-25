use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Fields, FieldsNamed, Ident, DataStruct, TypePath};
use quote::quote;

/// Check if provided type input is wrapped in Option<> or not
fn check_is_option(ty: &syn::Type) -> bool {
    if let syn::Type::Path(TypePath {path, ..}) = ty {
        if path.segments.len() == 1 && path.segments[0].ident == "Option" {
            return true;
        }
    }
    false
}

/// Unwrap option type if the field is already known
/// if the given type is not an Option type, do nothing
fn unwrap_option(ty: &syn::Type) -> &syn::Type {
    if check_is_option(ty) {
        println!("{:?}", ty);
    }
    ty
}

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput {ident, data, .. } = parse_macro_input!(input);
    let builder_name = format!("{}Builder", ident);
    let builder_ident = Ident::new(&builder_name, ident.span());
    
    let named_fields = match data {
        syn::Data::Struct(
            DataStruct {fields: Fields::Named(FieldsNamed {ref named, .. }), .. }
        ) => {
            named
        },
        _ => unimplemented!()
    };

    let fields = named_fields.iter().map(|n| {
        let name = &n.ident;
        let ty = &n.ty;
        if check_is_option(ty) {
            quote! {#name: #ty}
        } else {
            quote! {#name: std::option::Option<#ty>}
        }
    });
    let setters = named_fields.iter().map(|n| {
        let name = &n.ident;
        let ty = &n.ty;
        if check_is_option(ty) {
            quote! {
                pub fn #name(&mut self, #name: #ty) -> &mut Self {
                    self.#name = #name;
                    self
                }
            }
        } else {
            quote! {
                pub fn #name(&mut self, #name: #ty) -> &mut Self {
                    self.#name = Some(#name);
                    self
                }
            }
        }
    });
    let build_fields = named_fields.iter().map(|n| {
        let name = &n.ident;
        if check_is_option(&n.ty) {
            quote! { #name: self.#name.clone() }
        } else {
            quote! { #name: self.#name.clone().ok_or("Fail")? }
        }
    });
    
    let expended = quote! {
        pub struct #builder_ident {
            #(#fields,)*
        }

        impl #builder_ident {
            #(#setters)*

            pub fn build(&self) -> Result<#ident, Box<dyn std::error::Error>> {
                Ok(#ident {
                    #(#build_fields,)*
                })
            }
        }
    };
    
    TokenStream::from(expended)
}