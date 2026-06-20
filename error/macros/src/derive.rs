use quote::quote;
use syn::{Data, DeriveInput, LitStr, parse_macro_input};

macro_rules! syn_error {
    ($e:expr) => {
        match {$e} {
            Ok(ok) => ok,
            Err(err) => {let err: syn::Error = err; return err.into_compile_error().into()}
        }
    };
}

pub fn derive_error(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);

    let name = &input.ident;
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
    let Data::Struct(struct_data) = input.data else { return syn::Error::new_spanned(input.ident, "Error must be a struct. Please use the union!() macro to create error enums.").into_compile_error().into() };

    let error_attr = syn_error!(input.attrs.iter()
        .find(|attr| attr.meta.path().is_ident("error"))
        .map(|attr| attr.meta.require_list())
        .transpose());

    let args = struct_data.fields.members()
        .map(|member| match member {
            syn::Member::Named(ident) => quote! { #ident = self.#ident },
            syn::Member::Unnamed(index) => quote! { self.#index }
        });

    let fmt = syn_error!(error_attr
        .map(|list| {
            let tokens = list.tokens.clone().into();
            syn::parse::<LitStr>(tokens).map_err(|err| syn::Error::new_spanned(list, err))
        })
        .transpose());

    let display_impl = fmt.map(|fmt| {
        quote! {
            impl #impl_generics ::std::fmt::Display for #name #type_generics #where_clause {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::write!(f, #fmt, #(#args),*)
                }
            }
        }
    });

    let from_impl = quote! {
        // impl #impl_generics From<#name #type_generics> for errors_module::Error<#name #type_generics> #where_clause {
        //     fn from(value: #name #type_generics) -> errors_module::Error<#name #type_generics> {
        //         errors_module::Error::new(value)
        //     }
        // }
    };

    quote! {
        #display_impl

        #from_impl

        impl #impl_generics ::std::error::Error for #name #type_generics #where_clause {}
    }.into()
}
