use binary::VarI32;
use derive::{Decode, Encode, Packet};

/// Sent by the client to the server when it jumps while riding an entity that has the
/// WASDControlled entity flag set, for example when riding a horse.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct PassengerJump {
    /// The strength of the jump, depending on how long the rider has held the jump button.
    pub jump_strength: VarI32,
}
