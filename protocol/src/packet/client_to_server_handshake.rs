use derive::{Decode, Encode, Packet};

/// Sent by the client in response to a ServerToClientHandshake packet sent by the server. It is the
/// first encrypted packet in the login handshake and serves as a confirmation that encryption is
/// correctly initialised client side.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct ClientToServerHandshake;
