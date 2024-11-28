use uuid::Uuid;
use binary::w64;
use derive::{Decode, Encode, Packet};

/// Sent by the client every time it joins the server and when it equips new emotes. It may be used
/// by the server to find out which emotes the client has available. If the player has no emotes
/// equipped, this packet is not sent. Under certain circumstances, this packet is also sent from
/// the server to the client, but I was unable to find when this is done.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct EmoteList {
    /// The runtime ID of the player that owns the emote pieces below. If sent by the client, this
    /// player runtime ID is always that of the player itself.
    pub player_runtime_id: w64,
    /// A list of emote pieces that the player with the runtime ID above has.
    pub emote_pieces: Vec<Uuid>,
}
