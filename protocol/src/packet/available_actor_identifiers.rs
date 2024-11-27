use bytes::Bytes;
use derive::{Decode, Encode, Packet};

/// Sent by the server at the start of the game to let the client know all entities that are
/// available on the server.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct AvailableActorIdentifiers {
    /// Network NBT serialised tag of all entity identifiers that are available in the server.
    pub serialised_entity_identifiers: Bytes,
}
