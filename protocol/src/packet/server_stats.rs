use derive::{Decode, Encode, Packet};

/// Sent from the server to update the client on server statistics. It is purely used for telemetry.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct ServerStats {
    /// The server tick when the statistics were collected.
    pub server_time: f32,
    /// The latency between the client and the server, as measured by the server.
    pub network_time: f32,
}
