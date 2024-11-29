use binary::{v32, Encode, EnumEncoder, EnumDecoder, Writer, Numeric, Decode, Reader};
use derive::Packet;
use crate::types::BlockPos;
use crate::types::world::{Dimension, SubChunkEntry};

/// Sends data about multiple sub-chunks around a center point.
#[derive(Debug, Clone, Default, Packet)]
pub struct SubChunk<'a> {
    /// Whether client chunk caching is enabled or not.
    pub cache_enabled: bool,
    /// The dimension the sub-chunks are in.
    pub dimension: Dimension,
    /// An absolute sub-chunk center point that every SubChunkRequest uses as a reference.
    pub position: BlockPos,
    /// Sub-chunk entries relative to the center point.
    pub sub_chunk_entries: Vec<SubChunkEntry<'a>>,
}

impl<'a> Encode for SubChunk<'a> {
    fn encode(&self, w: &mut Writer) {
        self.cache_enabled.encode(w);
        Dimension::write::<v32>(&self.dimension, w);
        self.position.encode(w);

        u32::from_usize(self.sub_chunk_entries.len()).encode(w);
        self.sub_chunk_entries
            .iter()
            .for_each(|entry| entry.write(w, self.cache_enabled));
    }
}

impl<'a> Decode<'a> for SubChunk<'a> {
    fn decode(r: &mut Reader<'a>) -> Option<Self> {
        let mut pk = Self {
            cache_enabled: bool::decode(r)?,
            dimension: Dimension::read::<v32>(r)?,
            position: BlockPos::decode(r)?,
            ..Default::default()
        };

        let len = u32::decode(r)?.to_usize();
        pk.sub_chunk_entries = (0..len).filter_map(|_| SubChunkEntry::read(r, pk.cache_enabled)).collect();

        Some(pk)
    }
}
