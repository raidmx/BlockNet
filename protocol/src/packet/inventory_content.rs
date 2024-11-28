use derive::{Decode, Encode, Packet};

use crate::types::inventory::Window;
use crate::types::item::ItemInstance;

/// Sent by the server to update the full content of a particular inventory. It is usually sent for
/// the main inventory of the player, but also works for other inventories that are currently opened
/// by the player.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct InventoryContent<'a> {
    /// One of the windows that the client currently has opened, or a consistent one such as the
    /// main inventory.
    #[encoding(type = w32)]
    pub window: Window,
    /// The new content of the inventory. The length of this list must be equal to the full size of
    /// the inventory window that was updated.
    pub content: Vec<ItemInstance<'a>>,
}
