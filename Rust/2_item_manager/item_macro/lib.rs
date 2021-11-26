use proc_macro::TokenStream;
use syn::{AngleBracketedGenericArguments, DataStruct, DeriveInput, TypePath, parse_macro_input};
use quote::quote;

/// Option type unwrapper for Builder macro.
/// 
/// Unwrap option type if the field is already known,
/// if the given type is not an Option<> type, do nothing.
/// 
/// # Example
/// 
/// ---------------------------------------
/// ```rust
/// let (success, unwrapped) = unwrap_option(ty);
/// if success {
///     println!("type unwrapped to: {:?}", unwrapped);
/// } else {
///     println!("origin type is not Option<> wrapped, return the original reference");
/// }
/// ```
fn unwrap_option(ty: &syn::Type) -> (bool, &syn::Type){
    let mut is_already_option: bool = false;

    if let syn::Type::Path(TypePath {ref path, ..}) = ty {
        if path.segments.len() != 0 && path.segments[0].ident == "Option" {
            is_already_option = true;
            
            if let syn::PathArguments::AngleBracketed (
                AngleBracketedGenericArguments { args, .. }
            ) = &path.segments[0].arguments {

                if args.len() != 1 {
                    panic!("I've never seen anything like this!");
                }

                if let syn::GenericArgument::Type(ref new_ty) = args[0] {
                    return (is_already_option, new_ty);
                }
            }
        }
    }

    (is_already_option, ty)
}

#[proc_macro_derive(Builder)]
pub fn derivce_builder(input: TokenStream) -> TokenStream {
    let DeriveInput {ident, data, .. } = parse_macro_input!(input);
    let builder_name = format!("{}Builder", ident);
    let builder_ident = syn::Ident::new(&builder_name, ident.span());
    
    let named_fields = match data {
        syn::Data::Struct(
            DataStruct {fields: syn::Fields::Named(syn::FieldsNamed {ref named, .. }), .. }
        ) => {
            named
        },
        _ => unimplemented!()
    };

    let fields = named_fields.iter().map(|n| {
        let name = &n.ident;
        let ty = &n.ty;
        let (is_option, _) = unwrap_option(&n.ty);

        if is_option {
            quote! {#name: #ty}
        } else {
            quote! {#name: std::option::Option<#ty>}
        }
    });
    let setters = named_fields.iter().map(|n| {
        let name = &n.ident;
        let (_, ty) = unwrap_option(&n.ty);

        quote! {
            pub fn #name(&mut self, #name: #ty) -> &mut Self {
                self.#name = std::option::Option::Some(#name);
                self
            }
        }
    });
    let build_fields = named_fields.iter().map(|n| {
        let name = &n.ident;
        let (is_option, _) = unwrap_option(&n.ty);
        if is_option {
            quote! { #name: self.#name.clone() }
        } else {
            quote! { #name: self.#name.clone().ok_or(concat!("Missing important field: ", stringify!(#name)))? }
        }
    });
    let empty_fields = named_fields.iter().map(|n| {
        let name = &n.ident;

        quote! { #name: None }
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

        impl #ident {
            pub fn builder() -> #builder_ident {
                #builder_ident {
                    #(#empty_fields,)*
                }
            }
        }
    };
    
    TokenStream::from(expended)
}

/// TODO: Finish this
/// This macro can 'show' the content of given data structure in a more decent way
#[proc_macro_derive(Show)]
pub fn derive_show(input: TokenStream) -> TokenStream {
    let DeriveInput {data, .. } = parse_macro_input!(input);

    let contents = match data {
        syn::Data::Struct(DataStruct { fields, .. }) => {
            match fields {
                syn::Fields::Named(syn::FieldsNamed { named, .. }) => named,
                syn::Fields::Unnamed(syn::FieldsUnnamed { unnamed, .. }) => unnamed,
                syn::Fields::Unit => unimplemented!()
            }
        },
        //syn::Data::Enum(syn::DataEnum) => {},
        //syn::Data::Union(syn::DataUnion) => {},
        _ => unimplemented!()
    };

    println!("{:?}", contents.len());

    TokenStream::new()
}