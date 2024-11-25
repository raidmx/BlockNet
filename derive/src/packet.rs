use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, DeriveInput, Result};

use crate::add_trait_bounds;

pub(super) fn derive_packet(item: TokenStream) -> Result<TokenStream> {
    let mut input = parse2::<DeriveInput>(item)?;
    let name = input.ident.clone();

    add_trait_bounds(&mut input.generics, quote!(::std::fmt::Debug));
    
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics crate::packet::Packet for #name #ty_generics
        #where_clause
        {
            fn id(&self) -> crate::packet::PacketId {
                crate::packet::PacketId::#name
            }
        }
    })
}