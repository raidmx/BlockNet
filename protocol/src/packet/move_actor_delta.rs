use crate::types::Vec3;
use num_derive::{FromPrimitive, ToPrimitive};
use binary::{w64, Decode, Encode, Reader, Writer};

/// Sent by the server to move an entity. The packet is specifically optimised to save as much space
/// as possible, by only writing non-zero fields. As of 1.16.100, this packet no longer actually
/// contains any deltas.
#[derive(Debug, Clone, Default)]
pub struct MoveActorDelta {
    /// The runtime ID of the entity that is being moved. The packet works provided a non-player
    /// entity with this runtime ID is present.
    pub entity_runtime_id: w64,
    /// A list of flags that specify what data is in the packet.
    pub flags: u16,
    /// The new position that the entity was moved to.
    pub position: Vec3,
    /// The new absolute rotation. Unlike the position, it is not actually a delta. If any of the
    /// values of this rotation are not sent, these values are zero and no flag for them is present.
    pub rotation: Vec3,
}

impl Encode for MoveActorDelta {
    fn encode(&self, w: &mut Writer) {
        self.entity_runtime_id.encode(w);
        self.flags.encode(w);

        if self.flags & MoveActorDeltaFlag::HasX.flag() != 0 {
            self.position.x.encode(w);
        }
        if self.flags & MoveActorDeltaFlag::HasY.flag() != 0 {
            self.position.y.encode(w);
        }
        if self.flags & MoveActorDeltaFlag::HasZ.flag() != 0 {
            self.position.z.encode(w);
        }
        if self.flags & MoveActorDeltaFlag::HasRotX.flag() != 0 {
            ((self.rotation.x / (360.0 / 256.0)) as u8).encode(w);
        }
        if self.flags & MoveActorDeltaFlag::HasRotY.flag() != 0 {
            ((self.rotation.y / (360.0 / 256.0)) as u8).encode(w);
        }
        if self.flags & MoveActorDeltaFlag::HasRotZ.flag() != 0 {
            ((self.rotation.z / (360.0 / 256.0)) as u8).encode(w);
        }
    }
}

impl Decode<'_> for MoveActorDelta {
    fn decode(r: &mut Reader<'_>) -> Option<Self> {
        let mut pk = Self {
            entity_runtime_id: w64::decode(r)?,
            flags: u16::decode(r)?,
            ..Default::default()
        };

        if pk.flags & MoveActorDeltaFlag::HasX.flag() != 0 {
            pk.position.x = f32::decode(r)?;
        }
        if pk.flags & MoveActorDeltaFlag::HasY.flag() != 0 {
            pk.position.y = f32::decode(r)?;
        }
        if pk.flags & MoveActorDeltaFlag::HasZ.flag() != 0 {
            pk.position.z = f32::decode(r)?;
        }
        if pk.flags & MoveActorDeltaFlag::HasRotX.flag() != 0 {
            pk.rotation.x = (u8::decode(r)? as f32) * (360.0 / 256.0);
        }
        if pk.flags & MoveActorDeltaFlag::HasRotY.flag() != 0 {
            pk.rotation.y = (u8::decode(r)? as f32) * (360.0 / 256.0);
        }
        if pk.flags & MoveActorDeltaFlag::HasRotZ.flag() != 0 {
            pk.rotation.z = (u8::decode(r)? as f32) * (360.0 / 256.0);
        }

        Some(pk)
    }
}

#[derive(Clone, Copy, Debug, Default, FromPrimitive, ToPrimitive)]
pub enum MoveActorDeltaFlag {
    #[default]
    HasX,
    HasY,
    HasZ,
    HasRotX,
    HasRotY,
    HasRotZ,
    OnGround,
    Teleport,
    ForceMove,
}

impl MoveActorDeltaFlag {
    pub fn flag(&self) -> u16 {
        1 << (*self as u16)
    }
}
