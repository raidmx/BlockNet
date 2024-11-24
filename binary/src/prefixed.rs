use bytes::{Buf, BufMut, Bytes, BytesMut};
use crate::{w32, Binary, Numeric};

impl Binary for String {
    fn serialize(&self, writer: &mut BytesMut) {
        w32::write_usize(writer, self.len());
        writer.put_slice(self.as_bytes());
    }

    fn deserialize(reader: &mut Bytes) -> Option<Self> {
        let len = w32::read_usize(reader)?;

        let mut vec = vec![0u8; len];
        reader.copy_to_slice(&mut vec);

        match String::from_utf8(vec) {
            Ok(str) => Some(str),
            Err(_) => None
        }
    }
}

impl<T: Binary> Binary for Vec<T> {
    fn serialize(&self, writer: &mut BytesMut) {
        w32::write_usize(writer, self.len());

        for item in self {
            item.serialize(writer);
        }
    }

    fn deserialize(reader: &mut Bytes) -> Option<Self<>> {
        let len = w32::read_usize(reader)?;
        let mut vec = Vec::with_capacity(len);

        for _ in 0..len {
            vec.push(T::deserialize(reader)?);
        }

        Some(vec)
    }
}