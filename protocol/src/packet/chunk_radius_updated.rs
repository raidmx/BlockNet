use binary::VarI32;
use derive::{Decode, Encode, Packet};

/// Sent by the server in response to a RequestChunkRadius packet. It defines the chunk radius that
/// the server allows the client to have. This may be lower than the chunk radius requested by the
/// client in the RequestChunkRadius packet.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct ChunkRadiusUpdated {
    /// The final chunk radius that the client will adapt when it receives the packet. It does not
    /// have to be the same as the requested chunk radius.
    pub chunk_radius: VarI32,
}
