use derive::{Decode, Encode, Packet};

/// Sent by the server to show a Marketplace store offer to a player. It opens a window client-side
/// that displays the item. The ShowStoreOffer packet only works on the partnered servers: Servers
/// that are not partnered will not have a store button show up in the in-game pause menu and will,
/// as a result, not be able to open store offers on the client side. Sending the packet does
/// therefore not work when using a proxy that is not connected to with the domain of one of the
/// partnered servers.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct ShowStoreOffer {
    /// A string that identifies the offer for which a window should be opened. While typically a
    /// UUID, the ID could be anything.
    pub offer_id: String,
    /// The type of offer to show to the player.
    pub typ: StoreOfferType,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
#[encoding(type = u8)]
pub enum StoreOfferType {
    Marketplace,
    DressingRoom,
    ServerRange,
}
