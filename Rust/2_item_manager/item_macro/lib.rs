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

/// This macro derives std::fmt::Display to 'show' the content of given data structure
/// in a more decent way
#[proc_macro_derive(Show, attributes(color))]
pub fn derive_show(input: TokenStream) -> TokenStream {
    let DeriveInput {ident, ref data, .. } = parse_macro_input!(input);

    // Because in both struct and union, there is a Punctuated<field, comma> type,
    // where in enum, there is none but a Punctuated<variant, comma> type,
    // so the enum type should be considered separatly
    let enum_content = match data {
        syn::Data::Enum(syn::DataEnum {variants, .. }) => Some(variants),
        _ => None
    };

    let named_content = match data {
        syn::Data::Struct(syn::DataStruct {fields, ..}) => {
            match fields {
                syn::Fields::Named(syn::FieldsNamed {named, ..}) => Some(named),
                // ignore unnamed fields and unit type because it doesn't make sense
                _ => unimplemented!()
            }
        },
        syn::Data::Union(syn::DataUnion {fields: syn::FieldsNamed {named, ..}, ..}) => Some(named),
        _ => None
    };

    let contents = match enum_content {
        Some(v) => {
            //println!("variants: {:?}", v);
            let var_list = v.iter().map(|f| {
                let name = &f.ident;
                println!("{}", name);

                let mut result = format!("{}", name);

                // super ugly code, fix later
                for attr in &f.attrs {
                    if let Ok(syn::Meta::NameValue(syn::MetaNameValue {path: syn::Path {segments, ..}, lit, ..})) = attr.parse_meta() {
                        for seg in &segments {
                            if seg.ident == "color" {
                                if let syn::Lit::Str(ref lstr) = lit {
                                    let lit_val = lstr.value();
                                    result = format!("{}{}\x1b[0m", color_platte(lit_val.as_str()), name);
                                }
                            }
                        }
                    }
                }

                quote! {std::println!("{}", #result);}
            });

            quote! {#(#var_list)*}
        },
        None => {
            match named_content {
                Some(n) => {
                    //println!("fields: {:?}", n);
                    let named_fields_quote = n.iter().map(|f| {
                        let name = &f.ident;
                        let name_title = match name {
                            Some(s) => s.to_string(),
                            None => String::from("_")
                        };

                        quote! {
                            println!("{}: {:?}", #name_title, &self.#name);
                        }
                    });
                    quote! {#(#named_fields_quote)*}
                },
                None => panic!("Unsupported type for implementation.)")
            }
        }
    };

    let result_quote = quote! {
        impl std::fmt::Display for #ident {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                //std::println!("\n{}\n", #sep_line);
                #contents
                write!(f, "")
            }
        }
    };

    TokenStream::from(result_quote)
}

/// Function to get specific color hex formatting string base on color name.
/// 
/// # Example
/// 
/// ```rust
/// let red_color = color_platte("red");
/// assert_eq!(red_color, "\x1b[31;1m{}\x1b[0m");
/// ```
fn color_platte(color: &str) -> String {
    match color {
        "red" => String::from("\x1b[31;1m"),
        "yellow" => String::from("\x1b[33;1m"),
        "purple" => String::from("\x1b[35;1m"),
        "blue" => String::from("\x1b[34;1m"),
        "cyan" => String::from("\x1b[36;1m"),
        "white" => String::from("\x1b[37;1m"),
        _ => unimplemented!("The color you input has not been implement yet.")
    }
}