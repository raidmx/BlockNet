use derive::{Decode, Encode, Packet};

use crate::types::item_stack::ItemStackRequestEntry;

/// Sent by the client to change item stacks in an inventory. It is essentially a replacement of the
/// InventoryTransaction packet added in 1.16 for inventory specific actions, such as moving items
/// around or crafting. The InventoryTransaction packet is still used for actions such as placing
/// blocks and interacting with entities.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct ItemStackRequest<'a> {
    /// A list of item stack requests. These requests are all separate, but the client buffers the
    /// requests, so you might find multiple unrelated requests in this packet.
    pub requests: Vec<ItemStackRequestEntry<'a>>,
}