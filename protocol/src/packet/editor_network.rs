use derive::{Decode, Encode, Packet};
use crate::nbt::{NetworkLittleEndian, NBT};

/// Sent from the server to the client and vice versa to communicate editor-mode related
/// information. It carries a single compound tag containing the relevant information.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct EditorNetwork<'a> {
    /// A network little endian compound tag holding data relevant to the editor.
    pub payload: NBT<'a, NetworkLittleEndian>,
}
