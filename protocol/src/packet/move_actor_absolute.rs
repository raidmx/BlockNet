use crate::types::Vec3;
use num_derive::{FromPrimitive, ToPrimitive};
use binary::{w64, Decode, Encode, Reader, Writer};
use derive::Packet;

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum MoveFlag {
    OnGround,
    Teleport,
}

/// Sent by the server to move an entity to an absolute position. It is typically used for movements
/// where high accuracy isn't needed, such as for long range teleporting.
#[derive(Debug, Clone, Default, Packet)]
pub struct MoveActorAbsolute {
    /// The runtime ID of the entity. The runtime ID is unique for each world session, and entities
    /// are generally identified in packets using this runtime ID.
    pub entity_runtime_id: w64,
    /// A combination of MoveFlags that specify details of the movement.
    pub flags: u8,
    /// The position to move the entity to. If the entity is on a distance that the player cannot
    /// see it, the entity will still show up if the player moves closer.
    pub position: Vec3,
    /// The rotation of the entity. The first value is the pitch, the second is the head yaw, and
    /// the third is the yaw.
    pub rotation: Vec3
}

impl Encode for MoveActorAbsolute {
    fn encode(&self, w: &mut Writer) {
        self.entity_runtime_id.encode(w);
        self.flags.encode(w);
        self.position.encode(w);

        ((self.rotation.x / (360.0 / 256.0)) as u8).encode(w);
        ((self.rotation.y / (360.0 / 256.0)) as u8).encode(w);
        ((self.rotation.z / (360.0 / 256.0)) as u8).encode(w);
    }
}

impl Decode<'_> for MoveActorAbsolute {
    fn decode(r: &mut Reader<'_>) -> Option<Self> {
        let mut pk = Self {
            entity_runtime_id: w64::decode(r)?,
            flags: u8::decode(r)?,
            position: Vec3::decode(r)?,
            ..Default::default()
        };

        pk.rotation.x = (u8::decode(r)? as f32) * (360.0 / 256.0);
        pk.rotation.y = (u8::decode(r)? as f32) * (360.0 / 256.0);
        pk.rotation.z = (u8::decode(r)? as f32) * (360.0 / 256.0);

        Some(pk)
    }
}