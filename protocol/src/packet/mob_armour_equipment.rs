use binary::w64;
use derive::{Decode, Encode, Packet};
use crate::types::item::ItemInstance;

/// Sent by the server to the client to update the armour an entity is wearing. It is sent for both
/// players and other entities, such as zombies.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct MobArmourEquipment<'a> {
    /// The runtime ID of the entity. The runtime ID is unique for each world session, and entities
    /// are generally identified in packets using this runtime ID.
    pub entity_runtime_id: w64,
    /// The equipped helmet of the entity. Items that are not wearable on the head will not be
    /// rendered by the client. Unlike in Java Edition, blocks cannot be worn.
    pub helmet: ItemInstance<'a>,
    /// Chestplate is the chestplate of the entity. Items that are not wearable as chestplate will
    /// not be rendered.
    pub chestplate: ItemInstance<'a>,
    /// Leggings are the leggings of the entity. Items that are not wearable as leggings will not be
    /// rendered.
    pub leggings: ItemInstance<'a>,
    /// Boots are the boots of the entity. Items that are not wearable as boots will not be
    /// rendered.
    pub boots: ItemInstance<'a>,
}
