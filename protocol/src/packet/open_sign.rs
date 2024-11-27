use derive::{Decode, Encode, Packet};
use crate::types::UBlockPos;

/// Sent by the server to open a sign for editing. As of 1.19.80, the player can interact with a
/// sign to edit the text on either side instead of just the front.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct OpenSign {
    /// The position of the sign to edit. The client uses this position to get the data of the sign,
    /// including the existing text and formatting etc.
    pub position: UBlockPos,
    /// FrontSide dictates whether the front side of the sign should be opened for editing. If
    /// false, the back side is assumed to be edited.
    pub front_size: bool,
}