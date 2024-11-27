use derive::{Decode, Encode, Packet};

/// Sent by the server to render the different fogs in the Stack. The types of fog are controlled by
/// resource packs to change how they are rendered, and the ability to create custom fog.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct PlayerFog {
    /// A list of fog identifiers to be sent to the client. Examples of fog identifiers are
    pub stack: Vec<String>,
}
