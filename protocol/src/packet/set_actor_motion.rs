use glam::Vec3;
use binary::VarU64;
use derive::{Decode, Encode, Packet};

/// Sent by the server to change the client-side velocity of an entity. It is usually used in
/// combination with server-side movement calculation.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct SetActorMotion {
    /// The runtime ID of the entity. The runtime ID is unique for each world session, and entities
    /// are generally identified in packets using this runtime ID.
    pub entity_runtime_id: VarU64,
    /// The new velocity the entity gets. This velocity will initiate the client-side movement of
    /// the entity.
    pub velocity: Vec3,
}
