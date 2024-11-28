use num_derive::{FromPrimitive, ToPrimitive};
use binary::w64;
use derive::{Decode, Encode, Packet};

#[derive(Debug, Copy, Clone, FromPrimitive, ToPrimitive)]
pub enum EmoteFlag {
    ServerSide,
    MuteChat,
}

impl EmoteFlag {
    pub fn flag(&self) -> u8 {
        1 << (*self as u8)
    }
}

/// Sent by both the server and the client. When the client sends an emote, it sends this packet to
/// the server, after which the server will broadcast the packet to other players online.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct Emote {
    /// The entity that sent the emote. When a player sends this packet, it has this field set as
    /// its own entity runtime ID.
    pub entity_runtime_id: w64,
    /// The ID of the emote to send.
    pub emote_id: String,
    /// The Xbox User ID of the player that sent the emote. It is only set when the emote is used by
    /// a player that is authenticated with Xbox Live.
    pub xuid: String,
    /// An identifier only set for particular platforms when using an emote (presumably only for
    /// Nintendo Switch). It is otherwise an empty string, and is used to decide which players are
    /// able to emote with each other.
    pub platform_id: String,
    /// A combination of flags that change the way the Emote packet operates. When the server sends
    /// this packet to other players, the server side emote flag must be present.
    pub flags: u8,
}
