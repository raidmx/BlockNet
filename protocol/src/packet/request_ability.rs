use derive::{Decode, Encode, Packet};
use crate::types::ability::Ability;

/// Sent by the client to the server to request permission for a specific ability from the server.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct RequestAbility {
    /// The ability that the client is requesting.
    pub ability: Ability,
    // /// The value of the ability.
    //pub value: dyn Any, // TODO
}