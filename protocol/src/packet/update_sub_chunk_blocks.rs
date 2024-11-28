use binary::{w32, w64};
use derive::{Decode, Encode, Packet};
use crate::types::BlockPos;
use crate::types::world::UpdateBlockTransition;

/// Essentially just the UpdateBlock packet, however for a set of blocks in a sub-chunk.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct UpdateSubChunkBlocks {
    /// The position of the sub-chunk being referred to.
    pub position: BlockPos,
    /// Each updated block change entry.
    pub blocks: Vec<BlockChangeEntry>,
    /// Each updated block change entry for the second layer, usually for waterlogged blocks.
    pub extra: Vec<BlockChangeEntry>,
}

/// Used by the UpdateSubChunkBlocks packet to specify a block change entry.
#[derive(Debug, Clone, Encode, Decode)]
pub struct BlockChangeEntry {
    /// The position of the block being changed.
    pub block_pos: BlockPos,
    /// The runtime ID of the block.
    pub block_runtime_id: w32,
    /// A combination of flags that specify the way the block is updated client-side.
    pub flags: w32,
    /// The unique ID of the falling block entity that the block transitions to or that the entity
    /// transitions from. Note that for both possible values for TransitionType, the
    /// `entity_unique_id` should point to the falling block entity involved.
    pub synced_update_entity_unique_id: w64,
    /// The type of the transition that happened. It is either `BlockToEntity`, when a block placed
    /// becomes a falling entity, or `EntityToBlock`, when a falling entity hits the ground and
    /// becomes a solid block again.
    #[encoding(type = w32)]
    pub synced_update_type: UpdateBlockTransition,
}
