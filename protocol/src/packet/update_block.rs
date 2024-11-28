use binary::w32;
use derive::{Decode, Encode, Packet};
use crate::types::UBlockPos;

#[derive(Clone, Copy, Debug)]
pub enum BlockUpdate {
    Neighbours,
    Network,
    NoGraphics,
    Priority,
}

impl BlockUpdate {
    pub fn flag(&self) -> u32 {
        1 << (*self as u32)
    }
}

/// Sent by the server to update a block client-side, without resending the entire chunk that the
/// block is located in. It is particularly useful for block breaking/placing.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct UpdateBlock {
    /// The block position at which a block is updated.
    pub position: UBlockPos,
    /// The runtime ID of the new block that is placed at position.
    pub new_block_runtime_id: w32,
    /// A combination of `BlockUpdate` flags that specify the way the block is updated client-side.
    /// Typically, sending only the `Network` flag is sufficient.
    pub flags: w32,
    /// The world layer on which the block is updated. For most blocks, this is the first layer, as
    /// that layer is the default layer to place blocks on.
    pub layer: w32,
}
