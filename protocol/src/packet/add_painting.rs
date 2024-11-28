use crate::types::Vec3;
use binary::{v32, v64, w64};
use derive::{Decode, Encode, Packet};

/// Sent by the server to the client to make a painting entity show up. It is one of the few
/// entities that cannot be sent using the AddActor packet.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct AddPainting {
    /// The unique ID of the entity. The unique ID is a value that remains consistent across
    /// different sessions of the same world, but most servers simply fill the runtime ID of the
    /// entity out for this field.
    pub entity_unique_id: v64,
    /// The runtime ID of the entity. The runtime ID is unique for each world session, and entities
    /// are generally identified in packets using this runtime ID.
    pub entity_runtime_id: w64,
    /// The position to spawn the entity on. If the entity is on a distance that the player cannot
    /// see it, the entity will still show up if the player moves closer.
    pub position: Vec3,
    /// The facing direction of the painting.
    pub direction: v32,
    /// The title of the painting. It specifies the motive of the painting. The title of the
    /// painting must be valid.
    pub title: String,
}
