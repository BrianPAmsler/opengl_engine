use itertools::Itertools;
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenTree};
use quote::{quote};
use syn::{Ident, Token, TypePath, parse::{Parse, ParseStream}, parse_macro_input, parse2, punctuated::Punctuated};

mod kw {
    syn::custom_keyword!(into);
}

struct Types(Punctuated<TypePath, Token![,]>);

impl Parse for Types {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Types(input.parse_terminated(TypePath::parse, Token![,])?))
    }
}

struct Input {
    types: Punctuated<TypePath, Token![,]>,
    custom_ident: Option<Ident>,
    into_ident: Option<Punctuated<TypePath, Token![,]>>
}

impl Parse for Input {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        enum ParseMode {
            Types,
            As,
            Into
        }

        let mut types = proc_macro2::TokenStream::new();
        let mut as_tokens = proc_macro2::TokenStream::new();
        let mut into_tokens = proc_macro2::TokenStream::new();
        let mut _as: Option<Token![as]> = None;
        let mut _into: Option<kw::into> = None;
        let mut mode = ParseMode::Types;
        while !input.is_empty() {
            let next: proc_macro2::TokenStream = input.parse::<TokenTree>()?.into();

            if _as.is_none() && let Ok(token) = parse2::<Token![as]>(next.clone()) {
                _as = Some(token);
                mode = ParseMode::As;
                continue;
            }

            if _into.is_none() &&let Ok(token) = parse2::<kw::into>(next.clone()) {
                _into = Some(token);
                mode = ParseMode::Into;
                continue;
            }

            match mode {
                ParseMode::Types => types.extend(next),
                ParseMode::As => as_tokens.extend(next),
                ParseMode::Into => into_tokens.extend(next),
            }
        }

        let types: Types = syn::parse2(types)?;
        let types = types.0;

        let custom_ident = _as.map(|_| syn::parse2(as_tokens)).transpose()?;
        let into_ident = _into.map(|_| syn::parse2::<Types>(into_tokens)).transpose()?.map(|val| val.0);
        Ok(Input {
            types,
            custom_ident,
            into_ident
        })
    }
}

#[proc_macro]
pub fn error_union(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Input);

    let union_name = input.custom_ident.unwrap_or_else(|| {
        let union_name = input.types.iter()
            .map(|path| path.path.segments.last().unwrap())
            .map(|name| {
                name.ident.to_string()
            })
            .chain(std::iter::once("Union".to_owned()))
            .reduce(|a, b| a + &b)
            .unwrap();
        Ident::new(&union_name, Span::call_site())
    });

    let variants = input.types.iter()
        .map(|path| {
            let name = path.path.segments.last().unwrap();
            quote! {
                #[error(transparent)]
                #name(#[from] #path)
            }
        })
        .collect_vec();

    let into = match input.into_ident {
        Some(ident) => {
            ident.iter()
                .map(|ident| {
                    let variants = input.types.iter()
                    .map(|path| path.path.segments.last().unwrap())
                    .map(|variant| quote! { #union_name::#variant(value) => value.into() })
                    .collect_vec();

                    quote! {
                        impl From<#union_name> for #ident {
                            fn from(value: #union_name) -> #ident {
                                match value {
                                    #(#variants),*
                                }
                            }
                        }

                        impl From<crate::error::Error<#union_name>> for crate::error::Error<#ident> {
                            fn from(value: crate::error::Error<#union_name>) -> crate::error::Error<#ident> {
                                crate::error::Error::from_existing(value)
                            }
                        }
                    }
                })
                .reduce(|mut a, b| {
                    a.extend(b);
                    a
                })
                .unwrap()
        },
        _ => proc_macro2::TokenStream::new()
    };

    let from = input.types.iter()
        .map(|path| {
            quote! {
                impl From<crate::error::Error<#path>> for crate::error::Error<#union_name> {
                    fn from(value: crate::error::Error<#path>) -> crate::error::Error<#union_name> {
                        crate::error::Error::from_existing(value)
                    }
                }
            }
        })
        .collect_vec();

    quote! {
        #[derive(thiserror::Error, Debug)]
        pub enum #union_name {
            #(#variants),*
        }

        impl EngineError for #union_name {}

        #into

        #(#from)*
    }.into()
}