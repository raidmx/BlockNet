use derive::{Decode, Encode, Packet};

use crate::types::world::GenerationFeature;

/// Notifies the client about the world generation features the server is currently using. This is
/// used in combination with the client-side world generation system introduced in v1.19.20,
/// allowing the client to completely generate the chunks of the world without having to rely on the
/// server.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct FeatureRegistry {
    /// A list of all registered world generation features.
    pub features: Vec<GenerationFeature>,
}
