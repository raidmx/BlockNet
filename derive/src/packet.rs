use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, parse_quote, DeriveInput, Error, Result};
use syn::spanned::Spanned;
use crate::{add_trait_bounds, decode_split_for_impl};

pub(super) fn derive_packet(item: TokenStream) -> Result<TokenStream> {
    let mut input = parse2::<DeriveInput>(item)?;
    let name = input.ident.clone();

    if input.generics.lifetimes().count() > 1 {
        return Err(Error::new(
            input.generics.params.span(),
            "type deriving `Packet` must have no more than one lifetime",
        ));
    }

    // Use the lifetime specified in the type definition or just use default lifetime if not
    // present.
    let lifetime = input
        .generics
        .lifetimes()
        .next()
        .map_or_else(|| parse_quote!('a), |l| l.lifetime.clone());

    add_trait_bounds(
        &mut input.generics,
        quote!(crate::packet::Packet<#lifetime>),
    );

    let (impl_generics, ty_generics, where_clause) = decode_split_for_impl(input.generics, lifetime.clone());

    Ok(quote! {
        impl #impl_generics crate::packet::Packet<#lifetime> for #name #ty_generics
        #where_clause
        {
            fn id(&self) -> crate::packet::PacketId {
                crate::packet::PacketId::#name
            }

            fn write(&self, w: &mut binary::Writer) {
                use binary::*;

                self.id().encode(w);
                self.encode(w);
            }

            fn read(r: &mut binary::Reader<#lifetime>) -> Option<Self> {
                use binary::*;

                Self::decode(r)
            }
        }
    })
}