use bytes::Bytes;
use derive::{Decode, Encode};
use crate::types::SliceU32;

#[derive(Debug, Default, Clone, Encode, Decode)]
pub struct Skin<'a> {
    pub skin_id: &'a str,
    pub play_fab_id: &'a str,
    pub skin_resource_patch: &'a [u8],
    pub skin_image_width: u32,
    pub skin_image_height: u32,
    pub skin_data: &'a [u8],
    pub animations: SliceU32<SkinAnimation>,
    pub cape_image_width: u32,
    pub cape_image_height: u32,
    pub cape_data: &'a [u8],
    pub skin_geometry: &'a [u8],
    pub geometry_data_engine_version: &'a [u8],
    pub animation_data: &'a [u8],
    pub cape_id: &'a str,
    pub full_id: &'a str,
    pub arm_size: &'a str,
    pub skin_colour: &'a str,
    pub persona_pieces: SliceU32<PersonaPiece>,
    pub piece_tint_colours: SliceU32<PersonaPieceTintColour>,
    pub premium_skin: bool,
    pub persona_skin: bool,
    pub persona_cape_on_classic_skin: bool,
    pub primary_user: bool,
    #[skip]
    pub trusted: bool,
    /// Specifies if the skin should override the player's skin that is equipped client-side. When
    /// false, the client will reject the skin and continue to use the skin that the player has
    /// equipped.
    pub override_appearance: bool,
}

#[derive(Debug, Clone, Default, Encode, Decode)]
pub struct SkinAnimation {
    pub image_width: u32,
    pub image_height: u32,
    pub image_data: Bytes,
    pub animation_type: u32,
    pub frame_count: f32,
    pub expression_type: u32,
}

#[derive(Debug, Clone, Default, Encode, Decode)]
pub struct PersonaPiece {
    pub piece_id: String,
    pub piece_type: String,
    pub pack_id: String,
    pub default: bool,
    pub product_id: String,
}

#[derive(Debug, Clone, Default, Encode, Decode)]
pub struct PersonaPieceTintColour {
    pub piece_type: String,
    pub colours: SliceU32<String>,
}
