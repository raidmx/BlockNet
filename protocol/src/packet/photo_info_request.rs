use binary::v64;
use derive::{Decode, Encode, Packet};

/// Sent by the client to request photo information from the server. This packet was deprecated in
/// 1.19.80.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct PhotoInfoRequest {
    /// The ID of the photo.
    pub photo_id: v64,
}
