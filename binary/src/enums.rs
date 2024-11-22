use bytes::{Bytes, BytesMut};
use crate::Numeric;

pub trait EnumEncoder<N: Numeric> {
    fn encode(&self, writer: &mut BytesMut);
    fn decode(reader: &mut Bytes) -> Self;
}