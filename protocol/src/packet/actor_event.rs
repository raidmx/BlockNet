use binary::{v32, w64};
use derive::{Decode, Encode, Packet};
use crate::types::ActorEventType;

/// Sent by the server when a particular event happens that has to do with an entity. Some of these
/// events are entity-specific, for example a wolf shaking itself dry, but others are used for each
/// entity, such as dying.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct ActorEvent {
    /// The runtime ID of the entity. The runtime ID is unique for each world session, and entities
    /// are generally identified in packets using this runtime ID.
    pub entity_runtime_id: w64,
    /// The type of event to be called.
    pub event_type: ActorEventType,
    /// Optional data associated with a particular event. The data has a different function for
    /// different events, however most events don't use this field at all.
    pub event_data: v32,
}