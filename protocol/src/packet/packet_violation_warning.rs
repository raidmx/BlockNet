use binary::VarI32;
use derive::{Decode, Encode, Packet};

/// Sent by the client when it receives an invalid packet from the server. It holds some information
/// on the error that occurred.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct PacketViolationWarning {
    /// The type of violation.
    pub violation_type: PacketViolationType,
    /// Specifies the severity of the packet violation. The action the client takes after this
    /// violation depends on the severity sent.
    pub severity: PacketViolationSeverity,
    /// The ID of the invalid packet that was received.
    pub packet_id: VarI32,
    /// A description on the violation of the packet.
    pub violation_context: String,
}

#[derive(Debug, Clone, Encode, Decode)]
#[encoding(type = VarI32)]
pub enum PacketViolationType {
    Malformed,
}

#[derive(Debug, Clone, Encode, Decode)]
#[encoding(type = VarI32)]
pub enum PacketViolationSeverity {
    Warning,
    FinalWarning,
    TerminatingConnection,
}
