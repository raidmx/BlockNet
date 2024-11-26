use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{parse2, parse_quote, Data, DeriveInput, Error, Fields, Result};

use crate::{add_trait_bounds, decode_split_for_impl, get_encoding_type, pair_variants_with_discriminants};

pub(super) fn derive_decode(item: TokenStream) -> Result<TokenStream> {
    let mut input = parse2::<DeriveInput>(item)?;

    let input_name = input.ident;

    if input.generics.lifetimes().count() > 1 {
        return Err(Error::new(
            input.generics.params.span(),
            "type deriving `Decode` must have no more than one lifetime",
        ));
    }

    // Use the lifetime specified in the type definition or just use default lifetime if not
    // present.
    let lifetime = input
        .generics
        .lifetimes()
        .next()
        .map_or_else(|| parse_quote!('a), |l| l.lifetime.clone());

    match input.data {
        Data::Struct(struct_) => {
            let decode_fields = match struct_.fields {
                Fields::Named(fields) => {
                    let init = fields.named.iter().map(|f| {
                        let name = f.ident.as_ref().unwrap();
                        let encoding_type = get_encoding_type(&f.attrs);

                        match encoding_type {
                            Some(et) => quote! {
                                #name: EnumDecoder::read::<#et>(r)?,
                            },
                            None => quote! {
                                #name: Decode::decode(r)?,
                            }
                        }
                    });

                    quote! {
                        Self {
                            #(#init)*
                        }
                    }
                }
                Fields::Unnamed(fields) => {
                    let init = (0..fields.unnamed.len())
                        .map(|_| {
                            quote! {
                                Decode::decode(r)?,
                            }
                        })
                        .collect::<TokenStream>();

                    quote! {
                        Self(#init)
                    }
                }
                Fields::Unit => quote!(Self),
            };

            add_trait_bounds(
                &mut input.generics,
                quote!(binary::Decode<#lifetime>),
            );

            let (impl_generics, ty_generics, where_clause) =
                decode_split_for_impl(input.generics, lifetime.clone());

            Ok(quote! {
                #[allow(unused_imports)]
                impl #impl_generics binary::Decode<#lifetime> for #input_name #ty_generics
                #where_clause
                {
                    fn decode(r: &mut &#lifetime [u8]) -> Option<Self> {
                        use binary::*;

                        Some(#decode_fields)
                    }
                }
            })
        }
        Data::Enum(e) => {
            let variants = pair_variants_with_discriminants(e.variants)?;
            let encoding_type = get_encoding_type(&input.attrs);

            if encoding_type.is_none() {
                return Err(Error::new(e.enum_token.span, "You must provide the #[encoding(T)] tag to specify the Enum Encoder."))
            }

            let decode_arms = variants
                .iter()
                .map(|(disc, variant)| {
                    let name = &variant.ident;

                    match &variant.fields {
                        Fields::Named(fields) => {
                            let fields = fields
                                .named
                                .iter()
                                .map(|f| {
                                    let field = f.ident.as_ref().unwrap();
                                    quote! {
                                        #field: Decode::decode(r)?,
                                    }
                                })
                                .collect::<TokenStream>();

                            quote! {
                                #disc => Some(Self::#name { #fields }),
                            }
                        }
                        Fields::Unnamed(fields) => {
                            let init = (0..fields.unnamed.len())
                                .map(|_| {
                                    quote! {
                                        Decode::decode(r)?,
                                    }
                                })
                                .collect::<TokenStream>();

                            quote! {
                                #disc => Some(Self::#name(#init)),
                            }
                        }
                        Fields::Unit => quote!(#disc => Some(Self::#name),),
                    }
                })
                .collect::<TokenStream>();

            add_trait_bounds(
                &mut input.generics,
                quote!(binary::Decode<#lifetime>),
            );

            let (impl_generics, ty_generics, where_clause) =
                decode_split_for_impl(input.generics, lifetime.clone());

            Ok(quote! {
                impl #impl_generics binary::Decode<#lifetime> for #input_name #ty_generics
                #where_clause
                {
                    fn decode(r: &mut &#lifetime [u8]) -> Option<Self> {
                        use binary::*;

                        let disc = #encoding_type::decode(r)?.to_isize();

                        match disc {
                            #decode_arms
                            n => None,
                        }
                    }
                }

                impl #impl_generics binary::EnumDecoder<#lifetime> for #input_name #ty_generics
                #where_clause
                {
                    fn read<V: binary::Variant>(r: &mut &#lifetime [u8]) -> Option<Self> {
                        use binary::*;

                        let disc = V::decode(r)?.to_isize();

                        match disc {
                            #decode_arms
                            n => None,
                        }
                    }
                }
            })
        }
        Data::Union(u) => Err(Error::new(
            u.union_token.span(),
            "cannot derive `Decode` on unions",
        )),
    }
}