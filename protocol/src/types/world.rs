use bytes::Bytes;
use num_derive::{FromPrimitive, ToPrimitive};
use binary::{Decode, Encode, Reader, v32, v64, Writer};
use derive::{Decode, Encode};
use crate::nbt::{NetworkLittleEndian, NBT};


#[derive(Debug, Clone, Encode, Decode)]
#[encoding(type = v32)]
pub enum SpawnType {
    Player,
    World,
}

#[derive(Debug, Clone, Default, FromPrimitive, ToPrimitive)]
pub enum SubChunkRequestMode {
    #[default]
    Legacy,
    Limitless,
    Limited,
}

#[derive(Debug, Clone, PartialEq, FromPrimitive, ToPrimitive, Encode, Decode)]
#[encoding(type = u8)]
pub enum SubChunkResult {
    Success,
    ChunkNotFound,
    InvalidDimension,
    PlayerNotFound,
    IndexOutOfBounds,
    SuccessAllAir,
}

#[derive(Debug, Copy, Clone, FromPrimitive, ToPrimitive, Encode, Decode)]
#[encoding(type = w32)]
pub enum Difficulty {
    Peaceful,
    Easy,
    Normal,
    Hard,
}

#[derive(Debug, Copy, Default, Clone, FromPrimitive, ToPrimitive, Encode, Decode)]
#[encoding(type = w32)]
pub enum Dimension {
    #[default]
    Overworld,
    Nether,
    End,
}

#[derive(Debug, Clone, PartialEq, FromPrimitive, ToPrimitive, Encode, Decode)]
#[encoding(type = u8)]
pub enum HeightMapType {
    None,
    HasData,
    TooHigh,
    TooLow,
}

#[derive(Debug, Clone, FromPrimitive, ToPrimitive, Encode, Decode)]
#[encoding(type = v32)]
pub enum GameType {
    Survival,
    Creative,
    Adventure,
    SurvivalSpectator,
    CreativeSpectator,
    Default,
    Spectator,
}

#[derive(Debug, Clone, FromPrimitive, ToPrimitive, Encode, Decode)]
#[encoding(type = v32)]
pub enum Generator {
    Legacy,
    Overworld,
    Flat,
    Nether,
    End,
    Void,
}

#[derive(Debug, Clone, FromPrimitive, ToPrimitive, Encode, Decode)]
#[encoding(type = w32)]
pub enum PermissionLevel {
    Visitor,
    Member,
    Operator,
    Custom,
}

#[derive(Debug, Clone, Encode, Decode)]
#[encoding(type = u8)]
pub enum EntityLinkType {
    Remove,
    Rider,
    Passenger,
}

#[derive(Debug, Clone, FromPrimitive, ToPrimitive, Encode, Decode)]
#[encoding(type = w64)]
pub enum UpdateBlockTransition {
    BlockToEntity,
    EntityToBlock,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct BlockEntry<'a> {
    pub name: &'a str,
    pub properties: NBT<'a, NetworkLittleEndian>,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct GenerationFeature {
    name: String,
    json: Bytes,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct DimensionDefinition {
    name: String,
    range: [v32; 2],
    generator: v32,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct EntityLink {
    pub ridden_entity_unique_id: v64,
    pub rider_entity_unique_id: v64,
    pub link_type: EntityLinkType,
    pub immediate: bool,
    pub rider_initiated: bool,
}

#[derive(Debug, Clone)]
pub struct SubChunkEntry<'a> {
    pub offset: SubChunkOffset,
    pub result: SubChunkResult,
    pub raw_payload: &'a [u8],
    pub height_map_type: HeightMapType,
    pub height_map_data: [i8; 256],
    pub blob_hash: u64,
}

impl<'a> SubChunkEntry<'a> {
    pub fn write(&self, w: &mut Writer, cache_enabled: bool) {
        self.offset.encode(w);
        self.result.encode(w);

        if self.result != SubChunkResult::SuccessAllAir || cache_enabled {
            self.raw_payload.encode(w);
        }

        self.height_map_type.encode(w);

        if self.height_map_type == HeightMapType::HasData {
            self.height_map_data.encode(w);
        }

        if !cache_enabled {
            self.blob_hash.encode(w);
        }
    }

    pub fn read(r: &mut Reader<'a>, cache_enabled: bool) -> Option<Self> {
        let mut entry = Self {
            offset: SubChunkOffset::decode(r)?,
            result: SubChunkResult::decode(r)?,
            raw_payload: &r[..0],
            height_map_type: HeightMapType::None,
            height_map_data: [0; 256],
            blob_hash: 0,
        };

        if entry.result != SubChunkResult::SuccessAllAir || cache_enabled {
            entry.raw_payload = <&'a [u8]>::decode(r)?;
        }

        entry.height_map_type = HeightMapType::decode(r)?;

        if entry.height_map_type == HeightMapType::HasData {
            entry.height_map_data = <[i8; 256]>::decode(r)?;
        }

        if !cache_enabled {
            entry.blob_hash = u64::decode(r)?
        }

        Some(entry)
    }
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct SubChunkOffset {
    pub x: i8,
    pub y: i8,
    pub z: i8,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct CacheBlob {
    pub hash: u64,
    pub payload: Bytes,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct ExperimentData {
    pub name: String,
    pub enabled: bool,
}