use binary::v32;
use derive::{Decode, Encode, Packet};
use crate::types::Window;

/// Sent by the server to update specific data of a single container, meaning a block such as a
/// furnace or a brewing stand. This data is usually used by the client to display certain features
/// client-side.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct ContainerSetData {
    /// The window that the packet modifies. It must point to one of the windows that the client
    /// currently has opened.
    pub window: Window,
    /// The key of the property. Multiple properties share the same key, but the functionality
    /// depends on the type of the container that the data is set to.
    pub key: v32,
    /// The value of the property. Its use differs per property.
    pub value: v32,
}
