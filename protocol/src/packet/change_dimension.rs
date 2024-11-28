use crate::types::Vec3;
use derive::{Decode, Encode, Packet};
use crate::types::world::Dimension;

/// Sent by the server to the client to send a dimension change screen client-side. Once the screen
/// is cleared client-side, the client will send a PlayerAction packet with the dimension change
/// done action attached.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct ChangeDimension {
    /// The dimension that the client should be changed to. The dimension must be different from the
    /// one the player is currently in, otherwise the client will freeze on the screen.
    #[encoding(type = v32)]
    pub dimension: Dimension,
    /// The position in the new dimension that the player is spawned in.
    pub position: Vec3,
    /// Specifies if the dimension change was respawn based, meaning that the player died in one
    /// dimension and got respawned into another. The client will send a PlayerAction packet with
    /// dimension change request attached as the action if it dies in another dimension, indicating
    /// that it needs a DimensionChange packet with respawn set to true.
    pub respawn: bool,
}
