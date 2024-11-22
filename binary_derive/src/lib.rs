extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote, quote_spanned, ToTokens, TokenStreamExt};
use std::collections::HashSet;
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Fields, FieldsNamed, PathArguments, Type};

/// Implements `Readable<T>` and `Writable` for a type named `T`.
///
/// For a struct, it does so by looking at its fields and writing/reading them in the order they are
/// defined. All of these fields must also implement `Readable<S>` and `Writable` for a field with
/// type `S`.
///
/// Vectors are a special case. They do not implement Readable or Writable by themselves, but can
/// still be written of its content does. These vectors do require an attribute to specify what type
/// to use to write the length of the vector. This type needs to be convertable from and to usize.
///
/// The first such attribute is `#[len_type(L)]`, where `L` i the type to use for the vector
/// length. It should be put above the vector in question.
/// ```ignore
/// use zuri_net_derive::proto;
///
/// #[proto]
/// pub struct PacketWithVec {
///     #[len_type(u8)]
///     pub vec: Vec<String>
/// }
/// ```
/// This vector will use a u8 to write / read its length. Does not affect how String is read or
/// written.
///
/// The other attribute that can be used is `#[len_for(V)]`, which, unlike the previous attribute,
/// should be used on a field before the actual field with the vector (named `V`). It will make
/// previous field in a packet act like it is the size of that vector, allowing the size to be
/// written elsewhere than right before the vector's content. The type used will be the type of the
/// field. Note that in the macro expansion, this field will be removed. The field only exists to
/// specify how the packet's data is structured
/// ```ignore
/// use zuri_net_derive::proto;
///
/// #[proto]
/// pub struct PacketWithVec {
///     #[len_for(vec)]
///     __: u16,
///     pub some_field: f32,
///     pub vec: Vec<String>
/// }
/// ```
/// Note that the vector length is written with a u16 here. The field is named `__`, but it can have
/// any name (as it will be removed anyway). This also means that multiple len_for fields can be
/// named the same.
///
/// Enums work slightly differently. First, the discriminant of the variant is written and then
/// any data that might be present in that variant. Using this packet on an enum would look
/// something like this
/// ```ignore
/// use zuri_net_derive::proto;
///
/// #[proto(u8)]
/// #[repr(u8)]
/// pub enum EnumPacket {
///     Variant1,
///     Variant2(Data),
///     Variant3(Data, Data, f32) = 7,
/// }
///
/// #[proto]
/// pub struct Data;
/// ```
/// Here is can be seen that the macro has an extra parameters for enums: the size to use to write
/// and read the discriminant. Variants can also contain any amount of unnamed fields or have an
/// explicit discriminant.
///
/// Sometimes, enum discriminants are written with a different type for the same enum in the
/// minecraft protocol (for some reason). This is also supported. When using this macro on an enum
/// `T`, it automatically implements `EnumReadable<T, D>` and `EnumWritable<D>` for that enum, where
/// `D` refers to the new type used for the discriminant. `D` needs to be convertible from and to
/// the default type specified in the attribute, as well as be writable and readable. To write an
/// enum with a specific discriminant type, `#[enum_header(D)]` can be used.
/// ```ignore
/// use zuri_net_derive::proto;
///
/// #[proto]
/// pub struct PacketWithEnum {
///     #[enum_header(u16)]
///     pub my_enum: MyEnum,
/// }
///
/// #[proto(u8)]
/// pub enum MyEnum {
///     V1, V2
/// }
/// ```
/// In this example `MyEnum`, which is usually written with `u8` when no `enum_header` is specified,
/// will be written using a `u16`.
#[proc_macro_attribute]
pub fn proto(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(_item as DeriveInput);
    let ident = input.ident.clone();

    // Write all errors to a separate token stream. If we have fully executed the macro, and notice
    // something went wrong, we use this as output to show all the errors that occurred while
    // compiling.
    let mut error_stream = proc_macro2::TokenStream::new();

    let mut write_stream = proc_macro2::TokenStream::new();
    let mut read_stream = proc_macro2::TokenStream::new();

    // We maintain a list of IO types for which we execute a different function using the IO trait
    // rather than executing the Binary trait on such types.
    let io_types = vec![
        "i8", "u8", "u16", "i16", "b16", "n16",
        "u24", "u32", "i32", "b32", "n32", "v32",
        "w32", "u64", "i64", "b64", "n64", "v64",
        "w64", "f32", "f64", "d32", "d64"
    ];

    match &mut input.data {
        Data::Struct(struct_data) => {
            let mut read_body_stream = proc_macro2::TokenStream::new();
            let mut read_inner_stream = proc_macro2::TokenStream::new();

            let named_fields: &mut FieldsNamed = match &mut struct_data.fields {
                // In the 'normal' case, a struct will have named fields inside curly brackets `{}`.
                // This is the main path of execution for this function.
                Fields::Named(f) => f,
                Fields::Unnamed(f) => return quote_spanned!(f.span()=> compile_error!("Tuple structs are not supported");).into(),
                // Unit structs do not have fields, so the read and write methods do not have to do
                // anything.
                Fields::Unit => return quote! {
                    #input

                    impl binary::Binary for #ident {
                        #[inline]
                        fn encode(&mut self, writer: &mut binary::Writer) {}

                        #[inline]
                        fn decode(reader: &mut binary::Reader) -> Self {}
                    }
                }.into(),
            };

            // Keep track of which vectors already had their size written previously (due to a
            // len_for attribute).
            let mut vector_size_map = HashSet::new();

            // Keep track of which fields need to be removed from the output struct. Currently, this
            // is every field that has a len_for attribute.
            let mut removal_queue = Vec::<usize>::new();

            'field_loop: for (field_i, field) in named_fields.named.iter_mut().enumerate() {
                let field_ident = field.ident.as_ref().unwrap();
                let field_type = &field.ty;

                let mut attr_remove_queue = Vec::new();

                // If this is Some, this indicates that the current field has a `len_type` or `enum_header`
                // attribute. The value is a span of the entire attribute which is used to show an error at
                // a certain location. Enum Header also contains the type value provided to serialize the
                // variants.
                let mut len_type_attr = None;
                let mut enum_header_attr = None;

                for (attr_i, attr) in field.attrs.iter().enumerate() {
                    // Helper function to parse attribute data of the form `(ident1, ident2)`, and
                    // return the contained idents, or an error (to write to the error token stream)
                    // if parsing was unsuccessful.
                    fn parse_attribute_ident(
                        tokens: proc_macro2::TokenStream,
                    ) -> Result<proc_macro2::Ident, proc_macro2::TokenStream> {
                        let group = match syn::parse2::<proc_macro2::Group>(tokens) {
                            Ok(g) => g,
                            Err(err) => {
                                let err_msg = err.to_compile_error();
                                return Err(quote_spanned!(err.span()=> #err_msg));
                            }
                        };

                        let ident = match syn::parse2::<proc_macro2::Ident>(group.stream()) {
                            Ok(i) => i,
                            Err(err) => {
                                let err_msg = err.to_compile_error();
                                return Err(quote_spanned!(group.span()=> #err_msg));
                            }
                        };

                        Ok(ident)
                    }

                    let path = attr.path.to_token_stream().to_string();
                    match path.as_str() {
                        "len_for" => {
                            // Get the vec name and optionally encoding within delimiters `(` and `)`
                            match parse_attribute_ident(attr.tokens.clone()) {
                                Err(t) => error_stream.append_all(t),
                                Ok(vec_name) => {
                                    let len_var_name = format_ident!("_{}_len", vec_name);

                                    if vector_size_map.contains(vec_name.to_string().as_str()) {
                                        let err = format!(
                                            "duplicate `len_for` for vector `{}`",
                                            vec_name
                                        );
                                        error_stream.append_all(
                                            quote_spanned!(vec_name.span()=> compile_error!(#err);),
                                        );
                                    }

                                    read_body_stream.append_all(quote! {
                                        let mut #len_var_name = Default::default();
                                        reader.#field_type(&mut #len_var_name);
                                    });

                                    write_stream.append_all(quote!{
                                        let mut len = self.#vec_name.len() as #field_type;
                                        writer.#field_type(&mut len);
                                    });

                                    vector_size_map.insert(vec_name.to_string());
                                    removal_queue.push(field_i);

                                    continue 'field_loop;
                                }
                            };
                        }
                        "len_type" => {
                            if vector_size_map.contains(field_ident.to_string().as_str()) {
                                let err = format!("Cannot combine `len_type` specifier with `len_for` for the same vector `{}`", field_ident.to_string());
                                error_stream.append_all(
                                    quote_spanned!(attr.span()=> compile_error!(#err);),
                                );
                                continue 'field_loop;
                            }
                            if !attr_remove_queue.is_empty() {
                                let err = format!(
                                    "Found more than one `len_type` specifier for vector `{}`",
                                    field_ident.to_string()
                                );
                                error_stream.append_all(
                                    quote_spanned!(attr.span()=> compile_error!(#err);),
                                );
                            }

                            match parse_attribute_ident(attr.tokens.clone()) {
                                Err(t) => error_stream.append_all(t),
                                Ok(type_name) => {
                                    let len_var_name = format_ident!("_{}_len", field_ident);
                                    let field_type = syn::parse2::<Type>(type_name.to_token_stream()).unwrap();

                                    read_body_stream.append_all(quote! {
                                        let mut #len_var_name = Default::default();
                                        reader.#type_name(&mut #len_var_name);
                                    });

                                    write_stream.append_all(quote!{
                                        let mut len = self.#field_ident.len() as #field_type;
                                        writer.#type_name(&mut len);
                                    });

                                    vector_size_map.insert(field_ident.to_string());
                                    attr_remove_queue.push(attr_i);
                                    len_type_attr = Some(attr.span());
                                }
                            }
                        }
                        "enum_header" => match parse_attribute_ident(attr.tokens.clone()) {
                            Err(t) => error_stream.append_all(t),
                            Ok(type_name) => {
                                enum_header_attr = Some((attr.span(), type_name));
                                attr_remove_queue.push(attr_i);
                            }
                        },
                        "skip" => {
                            field.attrs.remove(attr_i);

                            read_body_stream
                                .append_all(quote!(let mut #field_ident = Default::default();));
                            read_inner_stream.append_all(quote!(#field_ident,));
                            continue 'field_loop;
                        }
                        _ => continue
                    }
                }
                // Remove all the attributes that should be removed. First we make sure that all
                // the indices are sorted, so we can remove them all in reverse without changing the
                // indices of the next elements that we want to remove.
                attr_remove_queue.sort();
                for to_remove in attr_remove_queue.iter().rev() {
                    field.attrs.remove(*to_remove);
                }

                if let Type::Path(path) = field_type {
                    let last = path.path.segments.last().unwrap();
                    if last.ident == "Vec" {
                        let len_var_name = format_ident!("_{}_len", field_ident);

                        // In this case, the vector's length has not yet been written, so we should
                        // do it here. If the Vector did not have a len_type or len_for attribute
                        // then we should get the default length of the vector
                        if !vector_size_map.contains(field_ident.to_string().as_str()) {
                            vector_size_map.insert(field_ident.to_string());

                            read_body_stream.append_all(quote! {
                                let mut #len_var_name = 0;
                                reader.w32(&mut #len_var_name);
                            });

                            write_stream.append_all(quote! {
                                let mut len = self.#field_ident.len() as w32;
                                writer.w32(&mut len);
                            });
                        }

                        // This part adds the actual writing/reading of the content of the vector.
                        // Should always happen.
                        if let PathArguments::AngleBracketed(generic_type) = &last.arguments {
                            let inner = generic_type.args.first().unwrap();
                            let mut inner_type = inner.to_token_stream();

                            if let syn::GenericArgument::Type(Type::Reference(reference)) = inner {
                                inner_type = reference.elem.to_token_stream();
                            }

                            if io_types.contains(&inner_type.to_string().as_str()) {
                                write_stream.append_all(quote! {
                                    for mut elem in &mut self.#field_ident {
                                        writer.#inner_type(elem);
                                    }
                                });

                                read_body_stream.append_all(quote! {
                                    let mut #field_ident = Vec::with_capacity(#len_var_name as usize);

                                    for _ in 0..#len_var_name {
                                        let mut value = Default::default();
                                        reader.#inner_type(&mut value);

                                        #field_ident.push(value);
                                    }
                                });
                            } else {
                                write_stream.append_all(quote! {
                                    for elem in self.#field_ident {
                                        elem.encode(writer);
                                    }
                                });

                                read_body_stream.append_all(quote! {
                                    let mut #field_ident = Vec::with_capacity(#len_var_name as usize);

                                    for _ in 0..#len_var_name {
                                        #field_ident.push(#inner_type::decode(reader));
                                    }
                                });
                            }

                            read_inner_stream.append_all(quote!(#field_ident,));
                        }
                        continue 'field_loop;
                    }
                }
                // If we reach this part of the code, we know the field's type must not be a vector.
                // If vec_type is not None, then a `len_type` attribute has been specified on this
                // non-vector field, which is not allowed.
                if let Some(span) = len_type_attr {
                    error_stream.append_all(quote_spanned!(span => compile_error!("the `len_type` attribute is only allowed on vectors");));
                }

                let field_name = quote! { #field_type }.to_string();

                if io_types.contains(&field_name.as_str()) {
                    write_stream.append_all(quote! {
                        writer.#field_type(&mut self.#field_ident);
                    });
                    read_body_stream.append_all(quote! {
                        let mut #field_ident = Default::default();
                        reader.#field_type(&mut #field_ident);
                    });
                } else {
                    write_stream.append_all(quote! {
                        self.#field_ident.encode(writer);
                    });
                    read_body_stream.append_all(quote! {
                        let #field_ident = #field_type::decode(reader);
                    });
                }

                read_inner_stream.append_all(quote!(#field_ident,));
            }

            // We can only remove the fields that need to be removed after iterating over them, so
            // we remove them here.
            let mut xi = 0usize;
            named_fields.named = named_fields
                .named
                .clone()
                .into_pairs()
                .filter(|_| {
                    let remove = !removal_queue.contains(&xi);
                    xi += 1;
                    remove
                })
                .collect();

            read_stream.append_all(quote! {
               #read_body_stream

                Self {
                    #read_inner_stream
                }
            });
        },
        Data::Enum(e) => {
            // Get the type name provided in `#[proto(TYPE_NAME)]`.
            let type_name = match syn::parse2::<proc_macro2::Ident>(_attr.clone().into()) {
                Ok(t) => t,
                Err(err) => {
                    let err_msg = if _attr.is_empty() {
                        format!(
                            "expected default variant type for enum `{}`",
                            ident.to_string()
                        )
                    } else {
                        format!(
                            "unexpected token in default variant type for enum `{}`",
                            ident.to_string()
                        )
                    };
                    error_stream.append_all(quote_spanned!(err.span()=> compile_error!(#err_msg);));
                    format_ident!("_")
                }
            };
        }
        Data::Union(u) => error_stream.append_all(
            quote_spanned!(u.union_token.span()=> compile_error!("Unions are not supported");),
        )
    }

    if !error_stream.is_empty() {
        return error_stream.into();
    }

    let tok = quote! {
        #input

        impl binary::Binary for #ident {
            fn encode(&mut self, writer: &mut binary::Writer) {
                use binary::*;
                #write_stream
            }

            fn decode(reader: &mut binary::Reader) -> Self {
                use binary::*;
                #read_stream
            }
        }
    };
    tok.into()
}