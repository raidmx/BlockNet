use binary::w64;
use derive::{Decode, Encode, Packet};

/// An Education Edition packet sent from the server to the client to return a response to a
/// previously requested action.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct AgentAnimation {
    /// The ID of the animation that the agent should perform. As of its implementation, there are
    /// no IDs that can be used in the regular client.
    pub animation: u8,
    /// The runtime ID of the target entity.
    pub entity_runtime_id: w64,
}
