use derive::{Decode, Encode, Packet};
use crate::types::item_stack::ItemStackResponseEntry;

/// Sent by the server in response to an ItemStackRequest packet from the client. This packet is
/// used to either approve or reject ItemStackRequests from the client. If a request is approved,
/// the client will simply continue as normal. If rejected, the client will undo the actions so that
/// the inventory should be in sync with the server again.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct ItemStackResponse {
    /// A list of responses to ItemStackRequests sent by the client before. Responses either approve
    /// or reject a request from the client. Vanilla limits the size of this list to 4096.
    pub responses: Vec<ItemStackResponseEntry>,
}
