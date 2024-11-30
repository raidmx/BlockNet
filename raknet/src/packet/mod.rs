pub mod connected_ping;
pub mod unconnected_ping;
pub mod connected_pong;
pub mod detect_lost_connections;
pub mod open_connection_request_1;
pub mod open_connection_reply_1;
pub mod open_connection_request_2;
pub mod open_connection_reply_2;

pub use connected_ping::*;
pub use unconnected_ping::*;
pub use detect_lost_connections::*;
pub use open_connection_request_1::*;
pub use open_connection_reply_1::*;
pub use open_connection_request_2::*;
pub use open_connection_reply_2::*;

use binary::{Decode, Encode};
use derive::{Decode, Encode};

#[derive(Debug, Clone, Encode, Decode)]
#[encoding(type = u8)]
pub enum PacketId {
    ConnectedPing,
    UnconnectedPing,
    UnconnectedPingOC,
    ConnectedPong,
    DetectLostConnections,
    OpenConnectionRequest1,
    OpenConnectionReply1,
    OpenConnectionRequest2,
    OpenConnectionReply2,
    ConnectionRequest,
    ConnectionRequestAccepted = 16,
    NewIncomingConnection = 19,
    Disconnect = 21,
    IncompatibleProtocol = 25,
    UnconnectedPong = 28,
    Game = 254
}

pub trait Packet<'a> : Encode + Decode<'a> {
    fn id(&self) -> PacketId;
}