use binary::{Decode, Encode, Reader, Writer};

#[repr(u32)]
pub enum PacketId {
    ABC
}

pub trait Packet<'a> : Encode + Decode<'a> {
    fn id(&self) -> PacketId;
}

#[derive(Debug)]
pub struct PacketTest {
}

impl Encode for PacketTest {
    fn encode(&self, w: &mut Writer) {
        todo!()
    }
}

impl Decode<'_> for PacketTest {
    fn decode(r: &mut Reader<'_>) -> Option<Self> {
        todo!()
    }
}

impl Packet<'_> for PacketTest {
    fn id(&self) -> PacketId {
        PacketId::ABC
    }
}