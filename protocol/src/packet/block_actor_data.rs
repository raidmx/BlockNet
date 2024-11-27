use derive::{Decode, Encode, Packet};
use crate::nbt::{NetworkLittleEndian, NBT};
use crate::types::UBlockPos;

/// Sent by the server to update data of a block entity, for example the data of a chest.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct BlockActorData<'a> {
    /// The position of the block that holds the block entity. If no block entity is at this
    /// position, the packet is ignored by the client.
    pub position: UBlockPos,
    /// The new data of the block that will be encoded to NBT and applied client-side, so that the
    /// client can see the block update. The NBTData should contain all properties of the block, not
    /// just properties that were changed.
    pub nbt_data: NBT<'a, NetworkLittleEndian>,
}
