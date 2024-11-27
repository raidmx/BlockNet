use derive::{Decode, Encode, Packet};
use crate::nbt::{NetworkLittleEndian, NBT};
use crate::types::UBlockPos;
use crate::types::world::Dimension;

/// Sends a volume entity's definition and metadata from server to client.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct AddVolumeEntity<'a> {
    /// The runtime ID of the volume. The runtime ID is unique for each world session, and entities
    /// are generally identified in packets using this runtime ID.
    pub entity_runtime_id: u64,
    /// A compound tag of entity metadata, which includes flags and data properties that alter in
    /// particular the way the volume functions or looks.
    pub entity_metadata: NBT<'a, NetworkLittleEndian>,
    /// The unique identifier for the volume. It must be of the form 'namespace:name', where
    /// namespace cannot be 'minecraft'.
    pub encoding_identifier: &'a str,
    /// The identifier of a fog definition.
    pub instance_identifier: &'a str,
    /// The volume's bounds. The first value is the minimum bounds, and the second value is the
    /// maximum bounds.
    pub bounds: [UBlockPos; 2],
    /// The dimension in which the volume exists.
    pub dimension: Dimension,
    /// The engine version the entity is using, for example, '1.17.0'.
    pub engine_version: &'a str,
}