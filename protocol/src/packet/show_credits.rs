use derive::{Decode, Encode, Packet};

/// Sent by the server to show the Minecraft credits screen to the client. It is typically sent when
/// the player beats the ender dragon and leaves the End.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct ShowCredits {
    /// The entity runtime ID of the player to show the credits to. It's not clear why this field is
    /// actually here in the first place.
    pub player_runtime_id: u64,
    /// The status type of the credits. It either starts or stops the credits.
    pub status_type: ShowCreditsStatus,
}

#[derive(Debug, Clone, Encode, Decode)]
#[encoding(type = v32)]
pub enum ShowCreditsStatus {
    Start,
    End,
}
