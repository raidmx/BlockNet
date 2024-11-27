use derive::{Decode, Encode, Packet};

#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct TrimData {
    pub patterns: Vec<TrimPattern>,
    pub materials: Vec<TrimMaterial>,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct TrimPattern {
    pub item_name: String,
    pub pattern_id: String,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct TrimMaterial {
    pub material_id: String,
    pub colour: String,
    pub item_name: String,
}
