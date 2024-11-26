use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, TokenStreamExt};
use syn::spanned::Spanned;
use syn::{parse2, Data, DeriveInput, Error, Fields, LitInt, Result};

use crate::{add_trait_bounds, get_encoding_type, pair_variants_with_discriminants};

pub(super) fn derive_encode(item: TokenStream) -> Result<TokenStream> {
    let mut input = parse2::<DeriveInput>(item)?;

    let input_name = input.ident;

    add_trait_bounds(
        &mut input.generics,
        quote!(binary::Encode),
    );

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    match input.data {
        Data::Struct(struct_) => {
            let encode_fields = match &struct_.fields {
                Fields::Named(fields) => fields
                    .named
                    .iter()
                    .map(|f| {
                        let name = &f.ident.as_ref().unwrap();
                        let field_type = &f.ty;

                        let encoding_type = get_encoding_type(&f.attrs);

                        match encoding_type {
                            Some(et) => quote! {
                                <#field_type as EnumEncoder>::write::<#et>(&self.#name, w);
                            },
                            None => quote! {
                                self.#name.encode(w);
                            }
                        }
                    })
                    .collect(),
                Fields::Unnamed(fields) => (0..fields.unnamed.len())
                    .map(|i| {
                        let lit = LitInt::new(&i.to_string(), Span::call_site());
                        quote! {
                            self.#lit.encode(w);
                        }
                    })
                    .collect(),
                Fields::Unit => TokenStream::new(),
            };

            Ok(quote! {
                #[allow(unused_imports)]
                impl #impl_generics binary::Encode for #input_name #ty_generics
                #where_clause
                {
                    fn encode(&self, w: &mut binary::Writer) {
                        use binary::*;
                        #encode_fields
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

            let mut type1_arms = TokenStream::new();
            let mut type2_arms = TokenStream::new();

            for (disc, variant) in variants.iter() {
                let variant_name = &variant.ident;

                match &variant.fields {
                    Fields::Named(fields) => {
                        let field_names = fields
                            .named
                            .iter()
                            .map(|f| f.ident.as_ref().unwrap())
                            .collect::<Vec<_>>();

                        let encode_fields = field_names
                            .iter()
                            .map(|name| {
                                quote! {
                                        #name.encode(w);
                                    }
                            })
                            .collect::<TokenStream>();

                        type1_arms.append_all(quote! {
                            Self::#variant_name { #(#field_names,)* } => {
                                #encoding_type::from_isize(#disc).encode(w);
                                #encode_fields
                            }
                        });

                        type2_arms.append_all(quote! {
                            Self::#variant_name { #(#field_names,)* } => {
                                V::from_isize(#disc).encode(w);
                                #encode_fields
                            }
                        });
                    }
                    Fields::Unnamed(fields) => {
                        let field_names = (0..fields.unnamed.len())
                            .map(|i| Ident::new(&format!("_{i}"), Span::call_site()))
                            .collect::<Vec<_>>();

                        let encode_fields = field_names
                            .iter()
                            .map(|name| {
                                quote! {
                                        #name.encode(w);
                                    }
                            })
                            .collect::<TokenStream>();

                        type1_arms.append_all(quote! {
                            Self::#variant_name(#(#field_names,)*) => {
                                #encoding_type::from_isize(#disc).encode(w);
                                #encode_fields
                            }
                        });

                        type2_arms.append_all(quote! {
                            Self::#variant_name(#(#field_names,)*) => {
                                V::from_isize(#disc).encode(w);
                                #encode_fields
                            }
                        });
                    }
                    Fields::Unit => {
                        type1_arms.append_all(quote!(Self::#variant_name => #encoding_type::from_isize(#disc).encode(w),));
                        type2_arms.append_all(quote!(Self::#variant_name => V::from_isize(#disc).encode(w),));
                    },
                }
            }

            Ok(quote! {
                impl #impl_generics binary::Encode for #input_name #ty_generics
                #where_clause
                {
                    fn encode(&self, w: &mut binary::Writer) {
                        use binary::*;

                        match self {
                            #type1_arms
                            _ => unreachable!(),
                        }
                    }
                }

                impl #impl_generics binary::EnumEncoder for #input_name #ty_generics
                #where_clause
                {
                    fn write<V: binary::Variant>(&self, w: &mut binary::Writer) {
                        use binary::*;

                        match self {
                            #type2_arms
                            _ => unreachable!(),
                        }
                    }
                }
            })
        }
        Data::Union(u) => Err(Error::new(
            u.union_token.span(),
            "cannot derive `Encode` on unions",
        )),
    }
}