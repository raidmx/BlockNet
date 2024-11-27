use binary::VarU64;
use derive::{Decode, Encode, Packet};

/// Sent by the server to the client. Its function is not entirely clear: It does not remove an
/// entity in the sense of an in-game entity, but has to do with the ECS that Minecraft uses.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct RemoveEntity {
    /// The network ID of the entity that should be removed.
    pub entity_network_id: VarU64,
}
