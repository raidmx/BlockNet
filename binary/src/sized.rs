use bytes::{Buf, BufMut, Bytes, BytesMut};
use crate::{w32, Binary, UsizeCodec};

pub trait SizedEncoder<E: UsizeCodec> {
    fn encode(&self, writer: &mut BytesMut);
    fn decode(reader: &mut Bytes) -> Self;
}

impl<E: UsizeCodec> SizedEncoder<E> for String {
    fn encode(&self, writer: &mut BytesMut) {
        E::write(writer, self.len());
        writer.put_slice(self.as_bytes());
    }

    fn decode(reader: &mut Bytes) -> Self {
        let len = E::read(reader);

        let mut vec = vec![0u8; len];
        reader.copy_to_slice(&mut vec);

        String::from_utf8(vec).unwrap()
    }
}

impl Binary for String {
    fn encode(&self, writer: &mut BytesMut) {
        w32::write(writer, self.len());
        writer.put_slice(self.as_bytes());
    }

    fn decode(reader: &mut Bytes) -> Self {
        let len = w32::read(reader);

        let mut vec = vec![0u8; len];
        reader.copy_to_slice(&mut vec);

        String::from_utf8(vec).unwrap()
    }
}

impl<E: UsizeCodec, T: Binary> SizedEncoder<E> for Vec<T> {
    fn encode(&self, writer: &mut BytesMut) {
        E::write(writer, self.len());

        for item in self {
            item.encode(writer);
        }
    }

    fn decode(reader: &mut Bytes) -> Self {
        let len = E::read(reader);

        let mut vec = Vec::with_capacity(len);
        for _ in 0..len {
            vec.push(T::decode(reader));
        }

        vec
    }
}

impl<T: Binary> Binary for Vec<T> {
    fn encode(&self, writer: &mut BytesMut) {
        w32::write(writer, self.len());

        for item in self {
            item.encode(writer);
        }
    }

    fn decode(reader: &mut Bytes) -> Self {
        let len = w32::read(reader);

        let mut vec = Vec::with_capacity(len);
        for _ in 0..len {
            vec.push(T::decode(reader));
        }

        vec
    }
}