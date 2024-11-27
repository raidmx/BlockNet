use derive::{Decode, Encode, Packet};

/// Sent by the server to refresh the player's entitlements.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct RefreshEntitlements;
