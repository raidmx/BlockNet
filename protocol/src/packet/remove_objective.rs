use derive::{Decode, Encode, Packet};

/// Sent by the server to remove a scoreboard objective. It is used to stop showing a scoreboard to
/// a player.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct RemoveObjective {
    /// The name of the objective that the scoreboard currently active has. This name must be
    /// identical to the one sent in the SetDisplayObjective packet.
    pub objective_name: String,
}
