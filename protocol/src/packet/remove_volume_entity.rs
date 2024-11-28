use derive::{Decode, Encode, Packet};
use crate::types::Dimension;

/// Sent to the client to indicate that a volume entity has been removed.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct RemoveVolumeEntity {
    /// The entity runtime ID of the volume entity that was removed.
    pub entity_runtime_id: u64,
    /// The dimension that the volume entity was in.
    #[encoding(type = v32)]
    pub dimension: Dimension,
}
