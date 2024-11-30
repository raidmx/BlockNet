pub mod connected_ping;
pub mod unconnected_ping;
pub mod connected_pong;
pub mod open_connection_request_1;
pub mod open_connection_reply_1;
pub mod open_connection_request_2;
pub mod open_connection_reply_2;
pub mod incompatible_protocol;
pub mod unconnected_pong;
pub mod game;

pub use connected_ping::*;
pub use unconnected_ping::*;
pub use connected_pong::*;
pub use open_connection_request_1::*;
pub use open_connection_reply_1::*;
pub use open_connection_request_2::*;
pub use open_connection_reply_2::*;
pub use incompatible_protocol::*;
pub use unconnected_pong::*;
pub use game::*;

use binary::{Decode, Encode, Reader, Writer};
use derive::{Decode, Encode};

#[derive(Debug, Clone, PartialOrd, PartialEq, Encode, Decode)]
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
    fn write(&self, w: &mut Writer);
    fn read(r: &mut Reader<'a>) -> Option<Self>;
}