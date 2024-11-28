use crate::types::{Rotation, Vec3};
use num_derive::{FromPrimitive, ToPrimitive};
use binary::VarU64;
use derive::{Decode, Encode, Packet};

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum MoveFlag {
    OnGround,
    Teleport,
}

/// Sent by the server to move an entity to an absolute position. It is typically used for movements
/// where high accuracy isn't needed, such as for long range teleporting.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct MoveActorAbsolute {
    /// The runtime ID of the entity. The runtime ID is unique for each world session, and entities
    /// are generally identified in packets using this runtime ID.
    pub entity_runtime_id: VarU64,
    /// A combination of MoveFlags that specify details of the movement.
    pub flags: u8,
    /// The position to move the entity to. If the entity is on a distance that the player cannot
    /// see it, the entity will still show up if the player moves closer.
    pub position: Vec3,
    /// The rotation of the entity. The first value is the pitch, the second is the head yaw, and
    /// the third is the yaw.
    pub rotation: Rotation,
}