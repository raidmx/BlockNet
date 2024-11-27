use derive::{Decode, Encode, Packet};
use crate::nbt::{NetworkLittleEndian, NBT};
use crate::types::structure::StructureTemplateDataRequestType;

/// Sent by the server to send data of a structure to the client in response to a
/// StructureTemplateDataRequest packet.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct StructureTemplateDataResponse<'a> {
    /// The name of the structure that was requested. This is the name used to export the structure
    /// to a file.
    pub structure_name: String,
    /// Contains NBT data of the structure template if a it was found by the StructureName that was
    /// sent in a StructureTemplateDataRequest packet.
    pub structure_template: Option<NBT<'a, NetworkLittleEndian>>,
    /// The response type of the packet. This depends on the RequestType field sent in the
    /// StructureTemplateDataRequest packet.
    pub response_type: StructureTemplateDataRequestType,
}
