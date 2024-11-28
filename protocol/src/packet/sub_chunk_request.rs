use derive::{Decode, Encode, Packet};

use crate::types::world::{Dimension, SubChunkOffset};
use crate::types::{BlockPos, SliceU32};

/// Requests specific sub-chunks from the server using a center point.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct SubChunkRequest {
    /// The dimension of the sub-chunks.
    #[encoding(type = v32)]
    pub dimension: Dimension,
    /// An absolute sub-chunk center point used as a base point for all sub-chunks requested. The X
    /// and Z coordinates represent the chunk coordinates, while the Y coordinate is the absolute
    /// sub-chunk index.
    pub position: BlockPos,
    /// Requested offsets around the center point.
    pub offsets: SliceU32<SubChunkOffset>,
}
