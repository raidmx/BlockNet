use bytes::Bytes;
use derive::{Decode, Encode, Packet};

/// Used to communicate custom messages from the client to the server, or from the server to the
/// client. While the name may suggest this packet is used for the discontinued scripting API, it is
/// likely instead for the GameTest framework.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct ScriptMessage {
    /// The identifier of the message, used by either party to identify the message data sent.
    pub identifier: String,
    /// The data of the message.
    pub data: Bytes,
}
