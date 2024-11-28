#![allow(deprecated)]

use binary::w64;
use derive::{Decode, Encode, Packet};
use crate::types::event::EventType;

/// Sent by the server to send an event with additional data. It is typically sent to the client for
/// telemetry reasons, much like the SimpleEvent packet.
#[derive(Debug, Clone, Encode, Decode, Packet)]
#[deprecated = "Deprecated as of Bedrock Edition v1.20.10"]
pub struct LegacyTelemetryEvent {
    /// The runtime ID of the player. The runtime ID is unique for each world session, and entities
    /// are generally identified in packets using this runtime ID.
    pub entity_runtime_id: w64,
    /// The parsed event data.
    pub event_data: EventType,
}
