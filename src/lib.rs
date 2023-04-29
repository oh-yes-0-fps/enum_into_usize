extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(IntoUsize)]
pub fn into_usize_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let enum_name = input.ident;
    let data_enum = match input.data {
        syn::Data::Enum(data_enum) => data_enum,
        _ => panic!("IntoUsize can only be derived for enums"),
    };

    let mut counter: usize = 0;
    let match_arms = data_enum.variants.iter().map(|variant| {
        let ident = &variant.ident;
        counter += 1;
        quote! {
            #enum_name::#ident => #counter
        }
    });

    let expanded = quote! {
        impl Into<usize> for #enum_name {
            fn into(self) -> usize {
                match self {
                    #(#match_arms,)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}