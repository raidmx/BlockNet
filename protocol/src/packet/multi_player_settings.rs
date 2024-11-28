use derive::{Decode, Encode, Packet};

/// Sent by the client to update multi-player related settings server-side and sent back to online
/// players by the server. The MultiPlayerSettings packet is a Minecraft: Education Edition packet.
/// It has no functionality for the base game.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct MultiPlayerSettings {
    /// The action that should be done when this packet is sent.
    pub action_type: MultiPlayerSettingsAction,
}

#[derive(Debug, Clone, Encode, Decode)]
#[encoding(type = v32)]
pub enum MultiPlayerSettingsAction {
    Enable,
    Disable,
    RefreshJoinCode,
}
