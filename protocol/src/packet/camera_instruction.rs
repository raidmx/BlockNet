use glam::{Vec2, Vec3};
use derive::{Decode, Encode, Packet};
use crate::types::RGB;

/// Gives a custom camera specific instructions to operate.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct CameraInstruction {
    pub data: Vec<CameraInstructionEntry>,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct CameraInstructionEntry {
    pub set: Option<CameraInstructionSet>,
    pub clear: Option<bool>,
    pub fade: Option<CameraInstructionFade>,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct CameraInstructionSet {
    pub preset: u32,
    pub ease: Option<CameraEase>,
    pub position: Option<Vec3>,
    pub rotation: Option<Vec2>,
    pub facing: Option<Vec3>,
    pub default: Option<bool>,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct CameraEase {
    pub r#type: CameraEaseType,
    pub duration: f32,
}

#[derive(Debug, Clone, Encode, Decode)]
#[encoding(type = u8)]
pub enum CameraEaseType {
    EasingTypeLinear,
    EasingTypeSpring,
    EasingTypeInQuad,
    EasingTypeOutQuad,
    EasingTypeInOutQuad,
    EasingTypeInCubic,
    EasingTypeOutCubic,
    EasingTypeInOutCubic,
    EasingTypeInQuart,
    EasingTypeOutQuart,
    EasingTypeInOutQuart,
    EasingTypeInQuint,
    EasingTypeOutQuint,
    EasingTypeInOutQuint,
    EasingTypeInSine,
    EasingTypeOutSine,
    EasingTypeInOutSine,
    EasingTypeInExpo,
    EasingTypeOutExpo,
    EasingTypeInOutExpo,
    EasingTypeInCirc,
    EasingTypeOutCirc,
    EasingTypeInOutCirc,
    EasingTypeInBounce,
    EasingTypeOutBounce,
    EasingTypeInOutBounce,
    EasingTypeInBack,
    EasingTypeOutBack,
    EasingTypeInOutBack,
    EasingTypeInElastic,
    EasingTypeOutElastic,
    EasingTypeInOutElastic,
}

#[derive(Debug, Clone, Encode, Decode)]
#[encoding(type = u8)]
pub struct CameraInstructionFade {
    pub fade_in_duration: f32,
    pub wait_duration: f32,
    pub fade_out_duration: f32,
    pub colour: RGB,
}
