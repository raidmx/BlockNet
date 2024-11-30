use bytes::Bytes;
use binary::{w32, Decode, Encode, Reader, Writer};
use crate::types::{IVec2, SubChunkRequestMode};
use derive::Packet;

/// Sent by the server to provide the client with a chunk of a world data (16xYx16 blocks).
/// Typically, a certain amount of chunks is sent to the client before sending it the spawn
/// PlayStatus packet, so that the client spawns in a loaded world.
#[derive(Debug, Clone, Default, Packet)]
pub struct LevelChunk {
    /// The X and Z coordinates of the chunk sent. You can convert a block coordinate to a chunk
    /// coordinate by right-shifting it four bits.
    pub position: IVec2,
    /// Specifies the mode in which chunks are sent. If this is anything but legacy, the sub-chunk
    /// request system is used.
    pub sub_chunk_request_mode: SubChunkRequestMode,
    /// The highest sub-chunk at the position that is not all air. It is only set if the sub
    /// chunk count is set to limited.
    pub highest_sub_chunk: u16,
    /// The amount of sub-chunks that are part of the chunk sent. Depending on if the cache is
    /// enabled, a list of blob hashes will be sent, or, if disabled, the sub-chunk data.
    pub sub_chunk_count: w32,
    /// Specifies if the client blob cache should be enabled. This system is based on hashes of
    /// blobs which are consistent and saved by the client in combination with that blob, so that
    /// the server does not have the same chunk multiple times. If the client does not yet have a
    /// blob with the hash sent, it will send a ClientCacheBlobStatus packet containing the hashes
    /// it does not have the data of.
    pub cache_enabled: bool,
    /// A list of all blob hashes used in the chunk. It is composed of `sub_chunk_count + 1` hashes,
    /// with the first SubChunkCount hashes being those of the sub-chunks and the last one that of
    /// the biome of the chunk. If caching is not enabled, this can be left empty.
    pub blob_hashes: Vec<u64>,
    /// A serialised string of chunk data. The data held depends on if CacheEnabled is set to true.
    /// If set to false, the payload is composed of multiple sub-chunks, each of which carry a
    /// version which indicates the way they are serialised, followed by biomes, border blocks and
    /// tile entities. If caching is enabled, the payload consists out of the border blocks and tile
    /// entities only.
    pub raw_payload: Bytes,
}

impl Encode for LevelChunk {
    fn encode(&self, w: &mut Writer) {
        self.position.encode(w);

        match self.sub_chunk_request_mode {
            SubChunkRequestMode::Legacy => {
                self.sub_chunk_count.encode(w);
            }
            SubChunkRequestMode::Limitless => {
                w32::new(u32::MAX).encode(w);
            }
            SubChunkRequestMode::Limited => {
                w32::new(u32::MAX - 1).encode(w);
                self.highest_sub_chunk.encode(w);
            }
        }
        self.cache_enabled.encode(w);

        if self.cache_enabled {
            self.blob_hashes.encode(w);
        }

        self.raw_payload.encode(w);
    }
}

impl Decode<'_> for LevelChunk {
    fn decode(r: &mut Reader<'_>) -> Option<Self> {
        let mut pk = Self {
            position: IVec2::decode(r)?,
            ..Default::default()
        };

        let sub_chunk_count = w32::decode(r)?.value();

        if sub_chunk_count == u32::MAX {
            pk.sub_chunk_request_mode = SubChunkRequestMode::Limitless;
        } else if sub_chunk_count == u32::MAX - 1 {
            pk.sub_chunk_request_mode = SubChunkRequestMode::Limited;
            pk.highest_sub_chunk = u16::decode(r)?;
        } else {
            pk.sub_chunk_count = sub_chunk_count.into();
        }

        pk.cache_enabled = bool::decode(r)?;

        if pk.cache_enabled {
            pk.blob_hashes = Vec::decode(r)?;
        }

        pk.raw_payload = Bytes::decode(r)?;

        Some(pk)
    }
}
