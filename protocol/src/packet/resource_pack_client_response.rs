use derive::{Decode, Encode, Packet};

use crate::types::resource_pack::ResourcePackResponse;
use crate::types::SliceU16;

/// Sent by the client in response to resource packets sent by the server. It is used to let the
/// server know what action needs to be taken for the client to have all resource packs ready.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct ResourcePackClientResponse {
    /// The response type the client gave.
    pub response: ResourcePackResponse,
    /// A list of resource pack UUIDs combined with their version that need to be downloaded, if the
    /// `response` field is `SendPacks`.
    pub packs_to_download: SliceU16<String>,
}
