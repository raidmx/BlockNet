use derive::{Decode, Encode, Packet};

/// Gives the client a list of custom camera presets.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct CameraPresets {
    pub data: Vec<CameraPresetEntry>,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct CameraPresetEntry {
    pub name: String,
    pub parent: String,
    pub pos_x: Option<f32>,
    pub pos_y: Option<f32>,
    pub pos_z: Option<f32>,
    pub rot_x: Option<f32>,
    pub rot_y: Option<f32>,
    pub listener: Option<AudioListener>,
    pub player_effects: Option<bool>,
}

#[derive(Debug, Clone, Encode, Decode)]
#[encoding(type = u8)]
pub enum AudioListener {
    Camera,
    Player,
}
