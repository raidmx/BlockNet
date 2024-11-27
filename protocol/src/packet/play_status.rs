use derive::{Decode, Encode, Packet};

/// Sent by the server to update a player on the play status. This includes failed statuses due to a
/// mismatched version, but also success statuses.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct PlayStatus {
    /// The status of the packet.
    pub status: PlayStatusType,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
#[encoding(type = b32)]
pub enum PlayStatusType {
    LoginSuccess,
    LoginFailedClient,
    LoginFailedServer,
    PlayerSpawn,
    LoginFailedInvalidTenant,
    LoginFailedVanillaEdu,
    LoginFailedEduVanilla,
    LoginFailedServerFull,
    LoginFailedEditorVanilla,
    LoginFailedVanillaEditor,
}
