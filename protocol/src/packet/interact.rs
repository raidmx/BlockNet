use binary::w64;
use crate::types::Vec3;
use derive::{Decode, Encode, Packet};

/// Sent by the client when it interacts with another entity in some way. It used to be used for
/// normal entity and block interaction, but this is no longer the case now.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct Interact {
    /// The type of action that was executed by the player.
    pub action_type: InteractionAction,
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Encode, Decode)]
#[encoding(type = u8)]
pub enum InteractionAction {
    LeaveVehicle(InteractionLeaveVehicle) = 3,
    MouseOverEntity(InteractionMouseOverEntity),
    NPCOpen(InteractionNPCOpen),
    OpenInventory(OpenInventory),
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
pub struct InteractionLeaveVehicle {
    /// The runtime ID of the entity that the player interacted with.
    pub target_entity_runtime_id: w64,
    /// The position that the player spawns at after leaving the vehicle.
    pub position: Vec3,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
pub struct InteractionMouseOverEntity {
    /// The runtime ID of the entity that the player interacted with.
    pub target_entity_runtime_id: w64,
    /// The position relative to the entity moused over over which the player hovered with its
    /// mouse/touch.
    pub position: Vec3,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
pub struct InteractionNPCOpen {
    /// The runtime ID of the entity that the player interacted with.
    pub target_entity_runtime_id: w64,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
pub struct OpenInventory {
    /// Unused.
    pub target_entity_runtime_id: w64,
}
