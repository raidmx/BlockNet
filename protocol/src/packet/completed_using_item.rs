use derive::{Decode, Encode, Packet};

use crate::types::item::UseItemMethod;

/// Sent by the server to notify client that it should be done using the item it is currently using.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct CompletedUsingItem {
    /// The item ID of the item that the client completed using. This should typically be the ID of
    /// the item held in the hand.
    pub used_item_id: i16,
    /// The method of the using of the item that was completed.
    pub use_method: UseItemMethod,
}
