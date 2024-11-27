use derive::{Decode, Encode, Packet};

use crate::types::world::EntityLink;

/// Sent by the server to initiate an entity link client-side, meaning one entity will start riding
/// another.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct SetActorLink {
    /// The link to be set client-side. It links two entities together, so that one entity rides
    /// another. Note that players that see those entities later will not see the link, unless it is
    /// also sent in the AddActor and AddPlayer packets.
    pub entity_link: EntityLink,
}
