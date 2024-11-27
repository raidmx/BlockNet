use binary::b32;
use derive::{Decode, Encode, Packet};

/// Sent by the client to request network settings, such as compression, from the server.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct RequestNetworkSettings {
    /// The protocol version of the player. The player is disconnected if the protocol is
    /// incompatible with the protocol of the server.
    pub client_protocol: b32,
}
