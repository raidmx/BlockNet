use derive::{Decode, Encode, Packet};

/// Sent by the server to send a 'simple event' to the client, meaning an event without any
/// additional event data. The event is typically used by the client for telemetry.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct SimpleEvent {
    /// The type of the event to be called.
    pub event_type: SimpleEventType,
}

#[derive(Debug, Clone, Encode, Decode)]
#[encoding(type = i16)]
pub enum SimpleEventType {
    None,
    CommandsEnabled,
    CommandsDisabled,
    UnlockWorldTemplateSettings,
}
