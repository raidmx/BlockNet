use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use derive::{Decode, Encode};
use crate::types::{UBlockPos, VarRGBA};

#[derive(Debug, Clone, Encode, Decode)]
#[encoding(type = i32)]
pub enum MapObjectType {
    Entity,
    Block,
}

#[derive(Clone, Copy, Debug, FromPrimitive, ToPrimitive)]
pub enum MapUpdateFlag {
    Texture,
    Decoration,
    Initialisation,
}

impl MapUpdateFlag {
    pub fn flag(&self) -> u32 {
        1 << (*self as u32)
    }
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct MapDecoration {
    pub decoration_type: MapDecorationType,
    pub rotation: u8,
    pub x: u8,
    pub y: u8,
    pub label: String,
    pub colour: VarRGBA,
}

#[derive(Debug, Clone, Encode, Decode)]
#[encoding(type = u8)]
pub enum MapDecorationType {
    MarkerWhite,
    MarkerGreen,
    MarkerRed,
    MarkerBlue,
    CrossWhite,
    TriangleRed,
    SquareWhite,
    MarkerSign,
    MarkerPink,
    MarkerOrange,
    MarkerYellow,
    MarkerTeal,
    TriangleGreen,
    SmallSquareWhite,
    Mansion,
    Monument,
    NoDraw,
    VillageDesert,
    VillagePlains,
    VillageSavanna,
    VillageSnowy,
    VillageTaiga,
    JungleTemple,
    WitchHut,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct MapTrackedObject {
    pub object_type: MapObjectType,
    pub entity_unique_id: i64,
    pub block_position: UBlockPos,
}
