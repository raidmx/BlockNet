use bytes::Bytes;
use derive::{Decode, Encode, Packet};

/// Sent by the server to let the client know all biomes that are available and implemented on the
/// server side. It is much like the AvailableActorIdentifiers packet, but instead for biomes.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct BiomeDefinitionList {
    /// Network NBT serialised tag of all definitions of biomes that are available on the server.
    pub serialised_biome_definitions: Bytes,
}
