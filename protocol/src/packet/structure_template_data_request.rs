use derive::{Decode, Encode, Packet};

use crate::types::structure::{StructureSettings, StructureTemplateDataRequestType};
use crate::types::UBlockPos;

/// Sent by the client to request data of a structure.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct StructureTemplateDataRequest {
    /// Name of the structure that was set in the structure block's UI. This is the name used to
    /// export the structure to a file.
    pub structure_name: String,
    /// The position of the structure block that has its template data requested.
    pub position: UBlockPos,
    /// Settings that should be used for exporting the structure. These settings are identical to
    /// the last sent in the StructureBlockUpdate packet by the client.
    pub settings: StructureSettings,
    /// The type of template data request that the player sent.
    pub request_type: StructureTemplateDataRequestType,
}
