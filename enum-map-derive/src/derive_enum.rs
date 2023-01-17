use crate::type_length;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{DataEnum, Fields, FieldsNamed, FieldsUnnamed, Ident, Variant};

pub fn generate(name: Ident, data_enum: DataEnum) -> TokenStream {
    let mut generator = EnumGenerator::empty();
    for variant in &data_enum.variants {
        generator.handle_variant(variant);
    }
    generator.finish(&name)
}

/// Total length is the sum of each variant's length. To represent a variant, its number is added to
/// the sum of previous variant lengths.
#[derive(Debug)]
struct EnumGenerator {
    length: TokenStream,
    from_usize_arms: TokenStream,
    into_usize_arms: TokenStream,
}

impl EnumGenerator {
    fn empty() -> Self {
        Self {
            length: quote! { 0usize },
            from_usize_arms: quote! {},
            into_usize_arms: quote! {},
        }
    }

    fn finish(&self, name: &Ident) -> TokenStream {
        let length = &self.length;
        let from_usize_arms = &self.from_usize_arms;
        let into_usize_arms = &self.into_usize_arms;

        quote! {
            #[automatically_derived]
            impl ::enum_map::Enum for #name {
                type Array<__EnumMapInternalV> = [__EnumMapInternalV; #length];

                #[inline]
                fn from_usize(value: ::enum_map::usize) -> Self {
                    #from_usize_arms {
                        ::enum_map::out_of_bounds()
                    }
                }

                #[inline]
                fn into_usize(self) -> ::enum_map::usize {
                    match self {
                        #into_usize_arms
                    }
                }
            }
        }
    }

    fn handle_variant(&mut self, variant: &Variant) {
        match &variant.fields {
            Fields::Unit => self.handle_unit_variant(&variant.ident),
            Fields::Unnamed(fields) => self.handle_unnamed_variant(&variant.ident, fields),
            Fields::Named(fields) => self.handle_named_variant(&variant.ident, fields),
        }
    }

    /// Becomes simply `1` in counting, since this is the size of the unit.
    fn handle_unit_variant(&mut self, variant: &Ident) {
        let into_arms = &self.into_usize_arms;
        let length = &self.length;
        self.into_usize_arms = quote! { #into_arms Self::#variant => #length, };
        let from_arms = &self.from_usize_arms;
        self.from_usize_arms = quote! {
            #from_arms if value == #length {
                Self::#variant
            } else
        };
        self.length = quote! { (#length + 1) };
    }

    /// Its size is the product of the sizes of its members. To represent this variant, one can
    /// think of this as representing a little-endian number. First member is simply added, but
    /// next members are multiplied before being added.
    fn handle_unnamed_variant(&mut self, variant: &Ident, fields: &FieldsUnnamed) {
        let length = &self.length;
        let mut expr_into = quote! { #length };
        let mut fields_length = quote! { 1usize };
        let mut params_from = quote! {};
        for (i, field) in fields.unnamed.iter().enumerate() {
            let ident = format_ident!("p{}", i);
            let ty = &field.ty;
            let field_length = type_length(ty);

            expr_into = quote! {
                (#expr_into + #fields_length * ::enum_map::Enum::into_usize(#ident))
            };

            params_from = quote! {
                #params_from <#ty as ::enum_map::Enum>::from_usize(
                    (value - #length) / #fields_length % #field_length
                ),
            };

            fields_length = quote! { (#fields_length * #field_length) };
        }

        self.length = quote! { (#length + #fields_length) };

        let length = &self.length;
        let from_arms = &self.from_usize_arms;
        self.from_usize_arms = quote! {
            #from_arms if value < #length {
                Self::#variant(#params_from)
            } else
        };

        let mut params_into = quote! {};
        for i in 0..fields.unnamed.len() {
            let ident = format_ident!("p{}", i);
            params_into = quote! { #params_into #ident, };
        }

        let into_arms = &self.into_usize_arms;
        self.into_usize_arms = quote! {
            #into_arms Self::#variant(#params_into) => #expr_into,
        };
    }

    /// Its size is the product of the sizes of its members. To represent this variant, one can
    /// think of this as representing a little-endian number. First member is simply added, but
    /// next members are multiplied before being added.
    fn handle_named_variant(&mut self, variant: &Ident, fields: &FieldsNamed) {
        let length = &self.length;
        let mut expr_into = quote! { #length };
        let mut fields_length = quote! { 1usize };
        let mut params_from = quote! {};

        for field in fields.named.iter() {
            let ident = field.ident.as_ref().unwrap();
            let ty = &field.ty;
            let field_length = type_length(ty);

            expr_into = quote! {
                (#expr_into + #fields_length * ::enum_map::Enum::into_usize(#ident))
            };

            params_from = quote! {
                #params_from #ident: <#ty as ::enum_map::Enum>::from_usize(
                    (value - #length) / #fields_length % #field_length
                ),
            };

            fields_length = quote! { (#fields_length * #field_length) };
        }

        self.length = quote! { (#length + #fields_length) };

        let length = &self.length;
        let from_arms = &self.from_usize_arms;
        self.from_usize_arms = quote! {
            #from_arms if value < #length {
                Self::#variant { #params_from }
            } else
        };

        let mut params_into = quote! {};
        for field in fields.named.iter() {
            let ident = field.ident.as_ref().unwrap();
            params_into = quote! { #params_into #ident, };
        }

        let into_arms = &self.into_usize_arms;
        self.into_usize_arms = quote! {
            #into_arms Self::#variant { #params_into } => #expr_into,
        };
    }
}
