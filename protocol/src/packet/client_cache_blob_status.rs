use binary::{Decode, Encode, Numeric, Reader, VarU32, Writer};
use derive::Packet;

/// Part of the blob cache protocol. It is sent by the client to let the server know what blobs it
/// needs and which blobs it already has, in an ACK type system.
#[derive(Debug, Clone, Packet)]
pub struct ClientCacheBlobStatus {
    /// A list of blob hashes that the client does not have a blob available for. The server should
    /// send the blobs matching these hashes as soon as possible.
    pub miss_hashes: Vec<u64>,
    /// A list of blob hashes that the client has a blob available for. The blobs hashes here mean
    /// that the client already has them: The server does not need to send the blobs anymore.
    pub hit_hashes: Vec<u64>,
}

impl Encode for ClientCacheBlobStatus {
    fn encode(&self, w: &mut Writer) {
        VarU32::from_usize(self.miss_hashes.len()).encode(w);
        VarU32::from_usize(self.hit_hashes.len()).encode(w);

        for item in self.miss_hashes.iter() {
            item.encode(w);
        }

        for item in self.hit_hashes.iter() {
            item.encode(w);
        }
    }
}

impl Decode<'_> for ClientCacheBlobStatus {
    fn decode(r: &mut Reader<'_>) -> Option<Self> {
        let miss_hashes_len = VarU32::decode(r)?.to_usize();
        let hit_hashes_len = VarU32::decode(r)?.to_usize();

        Some (
            Self {
                miss_hashes: (0..miss_hashes_len).map(|_| u64::decode(r)).collect::<Option<_>>()?,
                hit_hashes: (0..hit_hashes_len).map(|_| u64::decode(r)).collect::<Option<_>>()?
            }
        )
    }
}