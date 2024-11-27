use derive::{Decode, Encode, Packet};

/// Allows players to export photos from their portfolios into items in their inventory. This packet
/// only works on the Education Edition version of Minecraft.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct CreatePhoto {
    /// The unique ID of the entity.
    pub entity_unique_id: i64,
    /// The name of the photo.
    pub photo_name: String,
    /// The name of the photo as an item.
    pub item_name: String,
}
