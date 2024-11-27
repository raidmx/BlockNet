#![allow(deprecated)]

use bytes::Bytes;
use derive::{Decode, Encode, Packet};

/// Sent by both the client and the server. It is a way to let scripts communicate with the server,
/// so that the client can let the server know it triggered an event, or the other way around. It is
/// essentially an RPC kind of system.
#[derive(Debug, Clone, Encode, Decode, Packet)]
#[deprecated = "Deprecated as of Bedrock Edition v1.20.10"]
pub struct ScriptCustomEvent {
    /// The name of the event. The script and the server will use this event name to identify the
    /// data that is sent.
    pub event_name: String,
    /// The data of the event. This data is typically a JSON encoded string, that the script is able
    /// to encode and decode too.
    pub event_data: Bytes,
}
