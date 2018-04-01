//! Procedural macro implementing `#[derive(EnumMap)]`
//!
//! This is supposed to used with `enum-map` crate, which provides the
//! actual usage documentation.

#![recursion_limit = "128"]

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate syn;

use std::iter;

use syn::{Data, DataEnum, DeriveInput, Fields, Ident, Variant};

fn generate_enum_code(name: Ident, data_enum: DataEnum) -> quote::Tokens {
    let enum_count = data_enum.variants.len();
    let mut has_discriminants = false;

    for &Variant { ref fields, ref discriminant, .. } in &data_enum.variants {
        match *fields {
            Fields::Unit => (),
            _ => panic!("#[derive(EnumMap)] requires C style style enum"),
        }

        if discriminant.is_some() {
            has_discriminants = true;
        }
    }

    let variants_names_a = data_enum.variants.iter().map(|variant| &variant.ident);
    let variants_names_b = data_enum.variants.iter().map(|variant| &variant.ident);
    let repeat_name_a = iter::repeat(name);
    let repeat_name_b = repeat_name_a.clone();
    let counter = 0..enum_count;

    let to_usize = if enum_count == 0 || has_discriminants {
        let variants_names = data_enum.variants.iter().map(|variant| &variant.ident);
        let repeat_name = repeat_name_a.clone();
        let counter = counter.clone();

        quote! {
            match self {
                #(
                    #repeat_name::#variants_names => #counter,
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
                        #counter => #repeat_name_a::#variants_names_a,
                    )*
                    _ => unreachable!()
                }
            }
            fn to_usize(self) -> usize {
                #to_usize
            }
            fn from_function<F: FnMut(Self) -> V>(mut _f: F) -> Self::Array {
                [#(
                    _f(#repeat_name_b::#variants_names_b),
                )*]
            }
        }
    }
}

#[proc_macro_derive(EnumMap)]
pub fn derive_enum_map(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();

    let result = match input.data {
        Data::Enum(data_enum) => generate_enum_code(input.ident, data_enum),
        _ => panic!("#[derive(EnumMap)] is only defined for enums"),
    };

    result.into()
}
