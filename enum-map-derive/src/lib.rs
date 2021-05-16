//! Procedural macro implementing `#[derive(Enum)]`
//!
//! This is supposed to used with `enum-map` crate, which provides the
//! actual usage documentation.

#![recursion_limit = "128"]

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate syn;

use syn::spanned::Spanned;
use syn::{Data, DataEnum, DeriveInput, Fields, Ident, Variant};

fn generate_enum_code(name: Ident, data_enum: DataEnum) -> proc_macro2::TokenStream {
    let enum_count = data_enum.variants.len();
    let mut has_discriminants = false;

    for variant in &data_enum.variants {
        let Variant {
            fields,
            discriminant,
            ..
        } = variant;
        match fields {
            Fields::Unit => (),
            _ => {
                return syn::Error::new(fields.span(), "#[derive(Enum)] requires C style enum")
                    .to_compile_error();
            }
        }

        if discriminant.is_some() {
            has_discriminants = true;
        }
    }

    let variants_names = data_enum.variants.iter().map(|variant| &variant.ident);
    let counter = 0..enum_count;

    let into_usize = if enum_count == 0 || has_discriminants {
        let variants_names = variants_names.clone();
        let counter = counter.clone();

        quote! {
            match self {
                #(
                    Self::#variants_names => #counter,
                )*
            }
        }
    } else {
        quote! { self as usize }
    };

    quote! {
        #[automatically_derived]
        impl<V> ::enum_map::Enum<V> for #name {
            type Array = [V; #enum_count];

            #[inline]
            fn from_usize(value: usize) -> Self {
                match value {
                    #(
                        #counter => Self::#variants_names,
                    )*
                    _ => unreachable!()
                }
            }

            #[inline]
            fn into_usize(self) -> usize {
                #into_usize
            }
        }
    }
}

/// Procedural derive generating `enum_map::Enum` implementation.
///
/// # Examples
///
/// ```
/// # extern crate enum_map;
/// use enum_map::Enum;
///
/// #[derive(Enum)]
/// enum A {
///     B,
///     C,
///     D,
/// }
///
/// assert_eq!(Enum::<()>::into_usize(A::C), 1);
/// ```
#[proc_macro_derive(Enum)]
pub fn derive_enum_map(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();

    let result = match input.data {
        Data::Enum(data_enum) => generate_enum_code(input.ident, data_enum),
        _ => quote!(compile_error! {"#[derive(Enum)] is only defined for enums"}),
    };

    result.into()
}
