use derive::{Decode, Encode, Packet};
use crate::nbt::{NetworkLittleEndian, NBT};

/// An alternative to synced actor data. It is not exactly clear how it functions.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct SyncActorProperty<'a> {
    /// The purpose of this field is unknown.
    pub property_data: NBT<'a, NetworkLittleEndian>,
}
