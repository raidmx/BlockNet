use derive::{Decode, Encode, Packet};

/// Sent by the server to the client. The packet is currently unused by both client and server.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct AddBehaviourTree {
    /// An unused string.
    pub behaviour_tree: String,
}
