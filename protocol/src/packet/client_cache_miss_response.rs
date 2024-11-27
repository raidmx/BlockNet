use derive::{Decode, Encode, Packet};
use crate::types::CacheBlob;

/// Part of the blob cache protocol. It is sent by the server in response to a ClientCacheBlobStatus
/// packet and contains the blob data of all blobs that the client acknowledged not to have yet.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct ClientCacheMissResponse {
    pub blobs: Vec<CacheBlob>,
}
