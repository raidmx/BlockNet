pub mod encoding;
pub mod tag;

use std::collections::HashMap;
pub use encoding::*;
pub use tag::*;

use bytes::BufMut;
use binary::{generate, Decode, Encode, Reader, Writer};

generate!(NBT, <E: Encoding>, Tag<'a>, 'a);
generate!(CompoundTag, <E: Encoding>, Compound<'a>, 'a);
generate!(ListTag, <E: Encoding>, List<'a>, 'a);

impl<'a, E: Encoding> Encode for NBT<'a, E> {
    fn encode(&self, w: &mut Writer) {
        encode_tag_id(self.id(), w);
        E::write_str(w, "");
        encode::<E>(&self, w);
    }
}

impl<'a, E: Encoding> Decode<'a> for NBT<'a, E> {
    fn decode(r: &mut Reader<'a>) -> Option<Self> {
        let tag = decode_tag_id(r)?;
        E::read_str(r)?;
        Some(decode::<E>(tag, r)?.into())
    }
}

impl<'a, E:Encoding> Encode for CompoundTag<'a, E> {
    fn encode(&self, w: &mut Writer) {
        encode_tag_id(TagId::Compound, w);
        E::write_str(w, "");

        for (name, item) in self.iter() {
            encode_tag_id(item.id(), w); // TypeID of the NBT object
            E::write_str(w, name.as_str()); // Name of the NBT object
            encode::<E>(item, w); // The NBT object encoded
        }

        encode_tag_id(TagId::End, w) // Tag End to signify end of Compound
    }
}

impl<'a, E: Encoding> Decode<'a> for CompoundTag<'a, E> {
    fn decode(r: &mut Reader<'a>) -> Option<Self> {
        decode_tag_id(r)?;
        E::read_str(r)?;

        let mut compound = Compound::new(HashMap::new());

        loop {
            let tag = decode_tag_id(r)?;

            // We encountered the end of a compound tag. Break the loop.
            if tag == TagId::End {
                break;
            }

            let name = E::read_str(r)?.to_owned();

            if let Some(value) = decode::<E>(tag, r) {
                compound.insert(name, value);
            } else {
                return None;
            }
        }

        Some(compound.into())
    }
}

impl<'a, E: Encoding> Encode for ListTag<'a, E> {
    fn encode(&self, w: &mut Writer) {
        encode_tag_id(TagId::List, w);
        E::write_str(w, "");

        encode_tag_id(self.list_type(), w);
        E::write_int(w, self.len() as i32);

        for item in self.iter() {
            encode::<E>(item, w);
        }
    }
}

impl<'a, E: Encoding> Decode<'a> for ListTag<'a, E> {
    fn decode(r: &mut Reader<'a>) -> Option<Self> {
        decode_tag_id(r)?;
        E::read_str(r)?;

        let list_type = decode_tag_id(r)?;
        let mut len = E::read_int(r)?;

        if list_type == TagId::End {
            len = 0;
        }

        let mut list = List::with_capacity(list_type, len as usize);

        for _ in 0..len {
            if let Some(element) = decode::<E>(list_type, r) {
                list.push(element);
            } else {
                return None;
            }
        }

        Some(list.into())
    }
}

/// Encodes a [`TagId`] to the Writer.
#[inline]
pub fn encode_tag_id(id: TagId, w: &mut Writer) {
    (id as u8).encode(w);
}

/// Decodes a [`TagId`] from the Reader and returns it.
#[inline]
pub fn decode_tag_id(r: &mut Reader) -> Option<TagId> {
    Some(TagId::from_byte(u8::decode(r)?)?)
}

/// Encodes a [`Tag`] of the specified [`TagId`] to the [`Writer`].
/// Uses the specified [`Encoding`] to encode the tag.
pub fn encode<E: Encoding>(tag: &Tag, w: &mut Writer) {
    match tag {
        Tag::End => {},
        Tag::Byte(v) => v.encode(w),
        Tag::Short(v) => v.encode(w),
        Tag::Int(v) => E::write_int(w, *v),
        Tag::Long(v) => E::write_long(w, *v),
        Tag::Float(v) => v.encode(w),
        Tag::Double(v) => v.encode(w),
        Tag::ByteArray(v) => {
            E::write_int(w, v.len() as i32);

            unsafe {
                let slice: &&[u8] = std::mem::transmute(v);
                w.put_slice(slice);
            }
        }
        Tag::String(v) => E::write_str(w, v),
        Tag::List(v) => {
            encode_tag_id(v.list_type(), w);
            E::write_int(w, v.len() as i32);

            for item in v.iter() {
                encode::<E>(item, w);
            }
        }
        Tag::Compound(v) => {
            for (name, item) in v.iter() {
                encode_tag_id(item.id(), w); // TypeID of the NBT object
                E::write_str(w, name.as_str()); // Name of the NBT object
                encode::<E>(item, w); // The NBT object encoded
            }

            encode_tag_id(TagId::End, w) // Tag End to signify end of Compound
        }
        Tag::IntArray(v) => {
            E::write_int(w, v.len() as i32);

            for item in v.iter() {
                E::write_int(w, *item);
            }
        }
        Tag::LongArray(v) => {
            E::write_int(w, v.len() as i32);

            for item in v.iter() {
                E::write_long(w, *item);
            }
        }
    }
}

/// Decodes a [`Tag`] of the specified [`TagId`] from the [`Reader`] and returns it if successfully
/// decoded. Uses the specified [`Encoding`] to decode the tag.
pub fn decode<'a, E: Encoding>(id: TagId, r: &mut Reader<'a>) -> Option<Tag<'a>> {
    match id {
        TagId::End => None,
        TagId::Byte => Some(Tag::Byte(i8::decode(r)?)),
        TagId::Short => Some(Tag::Short(i16::decode(r)?)),
        TagId::Int => Some(Tag::Int(E::read_int(r)?)),
        TagId::Long => Some(Tag::Long(E::read_long(r)?)),
        TagId::Float => Some(Tag::Float(f32::decode(r)?)),
        TagId::Double => Some(Tag::Double(f64::decode(r)?)),
        TagId::ByteArray => {
            let len = E::read_int(r)? as usize;
            let slice = &r[..len];

            unsafe {
                let val: &[i8] = std::mem::transmute(slice);
                Some(Tag::ByteArray(val))
            }
        }
        TagId::String => {
            let string = E::read_str(r)?;
            Some(Tag::String(string))
        }
        TagId::List => {
            let list_type = decode_tag_id(r)?;
            let mut len = E::read_int(r)?;

            if list_type == TagId::End {
                len = 0;
            }

            let mut list = List::with_capacity(list_type, len as usize);

            for _ in 0..len {
                if let Some(element) = decode::<E>(list_type, r) {
                    list.push(element);
                } else {
                    return None;
                }
            }

            Some(Tag::List(list))
        }
        TagId::Compound => {
            let mut compound = Compound::new(HashMap::new());

            loop {
                let tag = decode_tag_id(r)?;

                // We encountered the end of a compound tag. Break the loop.
                if tag == TagId::End {
                    break;
                }

                let name = E::read_str(r)?.to_owned();

                if let Some(value) = decode::<E>(tag, r) {
                    compound.insert(name, value);
                } else {
                    return None;
                }
            }

            Some(Tag::Compound(compound))
        }
        TagId::IntArray => {
            let len = E::read_int(r)?;
            let mut array = Vec::with_capacity(len as usize);

            for _ in 0..len {
                let data = E::read_int(r)?;
                array.push(data);
            }

            Some(Tag::IntArray(array))
        }
        TagId::LongArray => {
            let len = E::read_int(r)?;
            let mut array = Vec::with_capacity(len as usize);

            for _ in 0..len {
                let data = E::read_long(r)?;
                array.push(data);
            }

            Some(Tag::LongArray(array))
        }
    }
}