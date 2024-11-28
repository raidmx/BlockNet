use crate::types::Vec3;
use binary::v64;
use derive::{Decode, Encode};
use crate::types::UBlockPos;

#[derive(Debug, Clone, Encode, Decode)]
#[encoding(type = w32)]
pub enum StructureBlockType {
    Data,
    Save,
    Load,
    Corner,
    Invalid,
    Export,
}

#[derive(Debug, Clone)]
pub enum StructureMirrorAxis {
    None,
    X,
    Z,
    Both,
}

#[derive(Debug, Clone, Encode, Decode)]
#[encoding(type = v32)]
pub enum StructureRedstoneSaveMode {
    Memory,
    Disk,
}

#[derive(Debug, Clone)]
pub enum StructureRotation {
    None,
    Rotate90,
    Rotate180,
    Rotate270,
}

#[derive(Debug, Clone, Encode, Decode)]
#[encoding(type = u8)]
pub enum StructureTemplateDataRequestType {
    None,
    ExportFromSave,
    ExportFromLoad,
    QuerySavedStructure,
    ImportFromSave,
}

#[derive(Debug, Clone)]
pub enum StructureTemplateDataResponseType {
    Export,
    Query,
    Import,
}

#[derive(Debug, Clone, Encode, Decode)]
#[encoding(type = u8)]
pub enum AnimationMode {
    None,
    Layers,
    Blocks,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct StructureSettings {
    pub palette_name: String,
    pub ignore_entities: bool,
    pub ignore_blocks: bool,
    pub allow_non_ticking_chunks: bool,
    pub size: UBlockPos,
    pub offset: UBlockPos,
    pub last_editing_player_unique_id: v64,
    pub rotation: u8,
    pub mirror: u8,
    pub animation_mode: AnimationMode,
    pub animation_duration: f32,
    pub integrity: f32,
    pub seed: u32,
    pub pivot: Vec3,
}
