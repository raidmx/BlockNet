use derive::{Decode, Encode, Packet};

/// Sent by the client to request the settings specific to the server. These settings are shown in a
/// separate tab client-side, and have the same structure as a custom form.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct ServerSettingsRequest;
