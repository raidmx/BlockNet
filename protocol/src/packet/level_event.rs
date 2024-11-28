use crate::types::Vec3;
use binary::v32;
use derive::{Decode, Encode, Packet};
use crate::types::level_event::LevelEventType;

/// Sent by the server to make a certain event in the level occur. It ranges from particles, to
/// sounds, and other events such as starting rain and block breaking.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct LevelEvent {
    /// The event that is being 'called'.
    pub event_type: LevelEventType,
    /// The position of the level event. Practically every event requires this Vec3 set for it, as
    /// particles, sounds and block editing relies on it.
    pub position: Vec3,
    /// An integer holding additional data of the event. The type of data held depends on the event
    /// type.
    pub event_data: v32,
}
