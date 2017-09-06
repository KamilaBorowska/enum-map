//! Procedural macro implementing `#[derive(EnumMap)]`
//!
//! This is supposed to used with `enum-map` crate, which provides the
//! actual usage documentation.

#![recursion_limit = "128"]

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use std::iter;

use proc_macro::TokenStream;
use syn::{Body, Ident, Variant, VariantData};
use quote::Tokens;

fn generate_enum_code(name: &Ident, variants: &[Variant]) -> Tokens {
    let mut enum_count = 0usize;
    let mut has_discriminants = false;
    for &Variant {
        ref data,
        ref discriminant,
        ..
    } in variants
    {
        if data != &VariantData::Unit {
            panic!("#[derive(EnumMap)] requires C style style enum");
        }
        if discriminant.is_some() {
            has_discriminants = true;
        }
        enum_count += 1;
    }

    let variant_a = variants.iter().map(|variant| &variant.ident);
    let variant_b = variants.iter().map(|variant| &variant.ident);
    let repeat_name_a = iter::repeat(name).take(variants.len());
    let repeat_name_b = repeat_name_a.clone();
    let counter = 0..variants.len();

    let to_usize = if variants.len() == 0 || has_discriminants {
        let repeat_name = repeat_name_a.clone();
        let variant = variants.iter().map(|variant| &variant.ident);
        let counter = 0..variants.len();
        quote! {
            match self {
                #(
                    #repeat_name::#variant => #counter,
                )*
            }
        }
    } else {
        quote! { self as usize }
    };

    quote! {
        impl<V> ::enum_map::Internal<V> for #name {
            type Array = [V; #enum_count];
            fn slice(array: &Self::Array) -> &[V] {
                array
            }
            fn slice_mut(array: &mut Self::Array) -> &mut [V] {
                array
            }
            fn from_usize(value: usize) -> Self {
                match value {
                    #(
                        #counter => #repeat_name_a::#variant_a,
                    )*
                    _ => unreachable!()
                }
            }
            fn to_usize(self) -> usize {
                #to_usize
            }
            fn from_function<F: FnMut(Self) -> V>(mut _f: F) -> Self::Array {
                [#(
                    _f(#repeat_name_b::#variant_b),
                )*]
            }
        }
    }
}

#[proc_macro_derive(EnumMap)]
pub fn derive_enum_map(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input(&input.to_string()).unwrap();
    match input.body {
        Body::Enum(ref variants) => generate_enum_code(&input.ident, variants),
        _ => panic!("#[derive(EnumMap)] is only defined for enums"),
    }.parse()
        .unwrap()
}
