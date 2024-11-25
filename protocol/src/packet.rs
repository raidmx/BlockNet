use binary::Encode;
use derive::{Decode, Encode};

#[derive(Debug, Encode, Decode)]
#[encoding(type = VarU32)]
pub enum PacketId {
    PlayStatus = 129
}

pub trait Packet : Encode {
    fn id(&self) -> PacketId;
}