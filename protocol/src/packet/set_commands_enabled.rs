use derive::{Decode, Encode, Packet};

/// Sent by the server to enable or disable the ability to execute commands for the client. If
/// disabled, the client itself will stop the execution of commands.\
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct SetCommandsEnabled {
    /// Defines if the commands should be enabled, or if false, disabled.
    pub enabled: bool,
}
