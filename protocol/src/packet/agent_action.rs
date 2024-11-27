use bytes::Bytes;
use derive::{Decode, Encode, Packet};

#[derive(Debug, Clone, Encode, Decode)]
#[encoding(type = VarI32)]
pub enum AgentActionType {
    None,
    Attack,
    Collect,
    Destroy,
    DetectRedstone,
    DetectObstacle,
    Drop,
    DropAll,
    Inspect,
    InspectData,
    InspectItemCount,
    InspectItemDetail,
    InspectItemSpace,
    Interact,
    Move,
    PlaceBlock,
    Till,
    TransferItemTo,
    Turn,
}

/// An Education Edition packet sent from the server to the client to return a response to a
/// previously requested action.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct AgentAction {
    /// JSON identifier referenced in the initial action.
    pub identifier: String,
    /// The action type that was requested.
    pub action: AgentActionType,
    /// JSON containing the response to the action.
    pub response: Bytes,
}
