use binary::w32;
use derive::{Decode, Encode, Packet};

use crate::types::inventory::Window;
use crate::types::item::ItemInstance;

/// Sent by the server to update a single slot in one of the inventory windows that the client
/// currently has opened. Usually this is the main inventory, but it may also be the off hand or,
/// for example, a chest inventory.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct InventorySlot<'a> {
    /// The window that the packet modifies. It must point to one of the windows that the client
    /// currently has opened.
    #[encoding(type = w32)]
    pub window: Window,
    /// The index of the slot that the packet modifies. The new item will be set to the slot at this
    /// index.
    pub slot: w32,
    /// The item to be put in the slot. It will overwrite any item that may currently be present in
    /// that slot.
    pub new_item: ItemInstance<'a>,
}
