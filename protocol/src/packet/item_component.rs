use derive::{Decode, Encode, Packet};
use crate::types::ItemComponentEntry;

/// Sent by the server to attach client-side components to a custom item.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct ItemComponent<'a> {
    /// A list of all custom items with their respective components set.
    pub items: Vec<ItemComponentEntry<'a>>,
}
