use derive::{Decode, Encode, Packet};

/// Sent by the client to the server to notify the server it purchased an item from the Marketplace
/// store that was offered by the server. The packet is only used for partnered servers.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct PurchaseReceipt {
    /// A list of receipts, or proofs of purchases, for the offers that have been purchased by the
    /// player. This is used for server-side verification of the purchase.
    pub receipts: Vec<String>,
}
