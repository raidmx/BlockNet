
use proc_macro::TokenStream as StdTokenStream;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{parse_quote, Attribute, Expr, GenericParam, Generics, Lifetime, LifetimeParam, Result, Variant};

mod encode;
mod decode;
mod packet;

#[proc_macro_derive(Encode, attributes(encoding, skip))]
pub fn derive_encode(item: StdTokenStream) -> StdTokenStream {
    match encode::derive_encode(item.into()) {
        Ok(tokens) => tokens.into(),
        Err(e) => e.into_compile_error().into(),
    }
}

#[proc_macro_derive(Decode, attributes(encoding, skip))]
pub fn derive_decode(item: StdTokenStream) -> StdTokenStream {
    match decode::derive_decode(item.into()) {
        Ok(tokens) => tokens.into(),
        Err(e) => e.into_compile_error().into(),
    }
}

#[proc_macro_derive(Packet)]
pub fn derive_packet(item: StdTokenStream) -> StdTokenStream {
    match packet::derive_packet(item.into()) {
        Ok(tokens) => tokens.into(),
        Err(e) => e.into_compile_error().into(),
    }
}

fn get_encoding_type(attrs: &Vec<Attribute>) -> Option<Expr> {
    let mut encoding_type = None;

    for attr in attrs {
        if attr.path().is_ident("encoding") {
            if let Err(_) = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("type") {
                    encoding_type = Some(meta.value()?.parse::<Expr>()?);
                }
                Ok(())
            }) {
                return None;
            }
        }
    }

    encoding_type
}

fn should_skip(attrs: &Vec<Attribute>) -> bool {
    for attr in attrs.iter() {
        if attr.path().is_ident("skip") {
            return true;
        }
    }

    false
}

fn pair_variants_with_discriminants(
    variants: impl IntoIterator<Item = Variant>,
) -> Result<Vec<(isize, Variant)>> {
    let mut discriminant = 0;
    variants
        .into_iter()
        .map(|v| {
            if let Some(i) = v.discriminant.as_ref() {
                discriminant = i
                    .1
                    .to_token_stream()
                    .to_string()
                    .chars()
                    .filter(|c| !c.is_ascii_whitespace())
                    .collect::<String>()
                    .parse::<isize>().unwrap();
            }

            let pair = (discriminant, v);
            discriminant += 1;
            Ok(pair)
        })
        .collect::<Result<_>>()
}

/// Adding our lifetime to the generics before calling `.split_for_impl()` would
/// also add it to the resulting `ty_generics`, which we don't want. So I'm
/// doing this hack.
fn decode_split_for_impl(
    mut generics: Generics,
    lifetime: Lifetime,
) -> (TokenStream, TokenStream, TokenStream) {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut impl_generics = impl_generics.to_token_stream();
    let ty_generics = ty_generics.to_token_stream();
    let where_clause = where_clause.to_token_stream();

    if generics.lifetimes().next().is_none() {
        generics
            .params
            .push(GenericParam::Lifetime(LifetimeParam::new(lifetime)));

        impl_generics = generics.split_for_impl().0.to_token_stream();
    }

    (impl_generics, ty_generics, where_clause)
}

fn add_trait_bounds(generics: &mut Generics, trait_: TokenStream) {
    for param in &mut generics.params {
        if let GenericParam::Type(type_param) = param {
            type_param.bounds.push(parse_quote!(#trait_))
        }
    }
}