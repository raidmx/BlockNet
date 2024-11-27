use binary::VarI64;
use derive::{Decode, Encode, Packet};
use crate::types::container::ContainerType;
use crate::types::inventory::Window;
use crate::types::UBlockPos;

/// Sent by the server to open a container client-side. This container must be physically present in
/// the world, for the packet to have any effect. Unlike Java Edition, Bedrock Edition requires that
/// chests for example must be present and in range to open its inventory.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct ContainerOpen {
    /// The window that is being opened. It may be used later to close the container using a
    /// ContainerClose packet.
    #[encoding(type = u8)]
    pub window: Window,
    /// The type of the container that is being opened when opening the container at the position of
    /// the packet. It depends on the block/entity, and could, for example, be a chest or a hopper,
    /// but also a horse inventory.
    #[encoding(type = u8)]
    pub container_type: ContainerType,
    /// The position of the container opened. The position must point to a block entity that
    /// actually has a container. If that is not the case, the window will not be opened and the
    /// packet will be ignored, if a valid container entity unique id has not also been provided.
    pub container_position: UBlockPos,
    /// The unique ID of the entity container that was opened. It is only used if the ContainerType
    /// is one that points to an entity, for example a horse.
    pub container_entity_unique_id: VarI64,
}