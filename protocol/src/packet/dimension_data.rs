use derive::{Decode, Encode, Packet};

use crate::types::world::DimensionDefinition;

/// A packet sent from the server to the client containing information about data-driven dimensions
/// that the server may have registered. This packet does not seem to be sent by default, rather
/// only being sent when any data-driven dimensions are registered.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct DimensionData {
    /// A list of data-driven dimension definitions registered on the server.
    pub definitions: Vec<DimensionDefinition>,
}
