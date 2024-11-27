use derive::{Decode, Encode, Packet};
use crate::types::UBlockPos;

/// Sent by the client when it takes an item out of an item frame.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct ItemFrameDropItem {
    /// The position of the item frame that had its item dropped. There must be a 'block entity'
    /// present at this position.
    pub position: UBlockPos,
}
