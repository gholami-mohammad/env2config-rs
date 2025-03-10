use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(FromEnv, attributes(env))]
pub fn derive_from_env(input: TokenStream) -> TokenStream {
    // Parse the input (the struct definition) into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    let the_struct = match input.data {
        syn::Data::Struct(ref data) => data,
        _ => panic!("FromEnv can only be derived for structs"),
    };

    let fields = match &the_struct.fields {
        syn::Fields::Named(named_fields) => &named_fields.named,
        _ => panic!("FromEnv can only be derived for structs with named fields"),
    };

    // Generate initialization code for each field
    let field_inits: Vec<_> = fields
        .iter()
        .map(|field| {
            // Get the field name (e.g., `host`)
            match field.ident.as_ref() {
                Some(field_ident) => {
                    // Find the `#[env("VAR")]` attribute
                    let env_attr = field
                        .attrs
                        .iter()
                        .find(|attr| attr.path().is_ident("env"))
                        .expect("Each field must have an #[env] attribute");

                    let params_token = match &env_attr.meta {
                        syn::Meta::List(l) => Some(
                            l.tokens
                                .clone()
                                .into_iter()
                                .filter(|t| t.to_string() != ",")
                                .map(|tt| quote! {#tt})
                                .collect::<Vec<_>>(),
                        ),
                        syn::Meta::NameValue(_nv) => None,
                        syn::Meta::Path(_p) => None,
                    };

                    if params_token.is_none() {
                        panic!("no valid parameter applied to env attribute");
                    }
                    let params_token = params_token.unwrap();
                    if params_token.len() == 0 {
                        panic!("no parameter passed to env attribute");
                    }

                    let env_var = &params_token[0];
                    let mut default_value = &quote! {""};
                    if params_token.len() >= 2 {
                        default_value = &params_token[1];
                    }
                    let ty: &syn::Type = &field.ty;

                    match ty.to_token_stream().to_string().as_str() {
                        "Vec < String >" => {
                            quote! {
                                #field_ident: std::env::var(#env_var)
                                    .unwrap_or(String::from(#default_value))
                                    .split(",")
                                    .into_iter()
                                    .map(|v| v.parse::<String>().expect("failed to parse"))
                                    .collect::<#ty>()
                            }
                        }
                        "Vec < bool >" => {
                            quote! {
                                #field_ident: std::env::var(#env_var)
                                    .unwrap_or("false".to_string())
                                    .split(",")
                                    .into_iter()
                                    .map(|v| v.parse::<bool>().expect("failed to parse"))
                                    .collect::<#ty>()
                            }
                        }
                        "Vec < u8 >" => {
                            quote! {
                                #field_ident: std::env::var(#env_var)
                                    .unwrap_or(String::from(#default_value))
                                    .split(",")
                                    .into_iter()
                                    .map(|v| {
                                        if v == "" {
                                            return 0;
                                        }

                                        return v.parse::<u8>().expect("failed to parse");
                                    })
                                    .collect::<#ty>()
                            }
                        }
                        "Vec < u16 >" => {
                            quote! {
                                #field_ident: std::env::var(#env_var)
                                    .unwrap_or(String::from(#default_value))
                                    .split(",")
                                    .into_iter()
                                    .map(|v| {
                                        if v == "" {
                                            return 0;
                                        }

                                        return v.parse::<u16>().expect("failed to parse");
                                    })
                                    .collect::<#ty>()
                            }
                        }
                        "Vec < u32 >" => {
                            quote! {
                                #field_ident: std::env::var(#env_var)
                                    .unwrap_or(String::from(#default_value))
                                    .split(",")
                                    .into_iter()
                                    .map(|v| {
                                        if v == "" {
                                            return 0;
                                        }

                                        return v.parse::<u32>().expect("failed to parse");
                                    })
                                    .collect::<#ty>()
                            }
                        }
                        "Vec < u64 >" => {
                            quote! {
                                #field_ident: std::env::var(#env_var)
                                    .unwrap_or(String::from(#default_value))
                                    .split(",")
                                    .into_iter()
                                    .map(|v| {
                                        if v == "" {
                                            return 0;
                                        }

                                        return v.parse::<u64>().expect("failed to parse");
                                    })
                                    .collect::<#ty>()
                            }
                        }
                        "Vec < u128 >" => {
                            quote! {
                                #field_ident: std::env::var(#env_var)
                                    .unwrap_or(String::from(#default_value))
                                    .split(",")
                                    .into_iter()
                                    .map(|v| {
                                        if v == "" {
                                            return 0;
                                        }

                                        return v.parse::<u128>().expect("failed to parse");
                                    })
                                    .collect::<#ty>()
                            }
                        }
                        "Vec < i8 >" => {
                            quote! {
                                #field_ident: std::env::var(#env_var)
                                    .unwrap_or(String::from(#default_value))
                                    .split(",")
                                    .into_iter()
                                    .map(|v| {
                                        if v == "" {
                                            return 0;
                                        }

                                        return v.parse::<i8>().expect("failed to parse");
                                    })
                                    .collect::<#ty>()
                            }
                        }
                        "Vec < i16 >" => {
                            quote! {
                                #field_ident: std::env::var(#env_var)
                                    .unwrap_or(String::from(#default_value))
                                    .split(",")
                                    .into_iter()
                                    .map(|v| {
                                        if v == "" {
                                            return 0;
                                        }

                                        return v.parse::<i16>().expect("failed to parse");
                                    })
                                    .collect::<#ty>()
                            }
                        }
                        "Vec < i32 >" => {
                            quote! {
                                #field_ident: std::env::var(#env_var)
                                    .unwrap_or(String::from(#default_value))
                                    .split(",")
                                    .into_iter()
                                    .map(|v| {
                                        if v == "" {
                                            return 0;
                                        }

                                        return v.parse::<i32>().expect("failed to parse");
                                    })
                                    .collect::<#ty>()
                            }
                        }
                        "Vec < i64 >" => {
                            quote! {
                                #field_ident: std::env::var(#env_var)
                                    .unwrap_or(String::from(#default_value))
                                    .split(",")
                                    .into_iter()
                                    .map(|v| {
                                        if v == "" {
                                            return 0;
                                        }

                                        return v.parse::<i64>().expect("failed to parse");
                                    })
                                    .collect::<#ty>()
                            }
                        }
                        "Vec < i128 >" => {
                            quote! {
                                #field_ident: std::env::var(#env_var)
                                    .unwrap_or(String::from(#default_value))
                                    .split(",")
                                    .into_iter()
                                    .map(|v| {
                                        if v == "" {
                                            return 0;
                                        }

                                        return v.parse::<i128>().expect("failed to parse");
                                    })
                                    .collect::<#ty>()
                            }
                        }
                        _ => {
                            // let tt = ty.to_token_stream().to_string();
                            quote! {
                                #field_ident: std::env::var(#env_var)
                                    .unwrap_or(String::from(#default_value))
                                    .parse::<#ty>()
                                    .unwrap_or(Default::default())
                            }
                        }
                    }
                }
                _ => {
                    quote! {}
                }
            }
        })
        .collect();

    // Get the struct name (e.g., `Config`)
    let struct_name = input.ident;

    // Generate the trait implementation
    let expanded = quote! {
        // The generated impl.
        impl env2config::FromEnv for #struct_name {
            fn from_env() -> Self {
                Self {
                    #(#field_inits),* // Expand the list of field initializations
                }
            }
        }
    };

    // Hand the output tokens back to the compiler.
    TokenStream::from(expanded)
}
