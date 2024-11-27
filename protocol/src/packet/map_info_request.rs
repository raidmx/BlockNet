use binary::VarI64;
use derive::{Decode, Encode, Packet};

use crate::types::colour::RGBA;

/// Sent by the client to request the server to deliver information of a certain map in the
/// inventory of the player. The server should respond with a ClientBoundMapItemData packet.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct MapInfoRequest {
    /// The unique identifier that represents the map that is requested over network. It remains
    /// consistent across sessions.
    pub map_id: VarI64,
    /// A list of pixels sent from the client to notify the server about the pixels that it isn't
    /// aware of.
    pub client_pixels: Vec<PixelRequest>,
}

/// The request for the colour of a pixel in a MapInfoRequest packet.
#[derive(Debug, Clone, Encode, Decode)]
pub struct PixelRequest {
    colour: RGBA,
    index: u16,
}
