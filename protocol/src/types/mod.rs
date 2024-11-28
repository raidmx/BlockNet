pub mod ability;
pub mod actor_event;
pub mod attribute;
pub mod colour;
pub mod command;
pub mod container;
pub mod device;
pub mod education;
pub mod entity_data;
pub mod event;
pub mod game_rule;
pub mod inventory;
pub mod item;
pub mod item_descriptor;
pub mod item_stack;
pub mod level_event;
pub mod map;
pub mod player;
pub mod recipe;
pub mod resource_pack;
pub mod scoreboard;
pub mod skin;
pub mod sound_event;
pub mod structure;
pub mod world;

pub use ability::*;
pub use actor_event::*;
pub use attribute::*;
pub use colour::*;
pub use command::*;
pub use container::*;
pub use device::*;
pub use education::*;
pub use entity_data::*;
pub use event::*;
pub use game_rule::*;
pub use inventory::*;
pub use item::*;
pub use item_descriptor::*;
pub use item_stack::*;
pub use level_event::*;
pub use map::*;
pub use player::*;
pub use recipe::*;
pub use resource_pack::*;
pub use scoreboard::*;
pub use skin::*;
pub use sound_event::*;
pub use structure::*;
pub use world::*;

use binary::{Array, Decode, Encode, Reader, v32, w32, Writer};

pub type SliceU8<T> = Array<u8, T>;
pub type SliceU16<T> = Array<u16, T>;
pub type SliceU32<T> = Array<u32, T>;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct BlockPos {
    pub x: i32,
    pub y: i32,
    pub z: i32
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct UBlockPos {
    pub x: i32,
    pub y: i32,
    pub z: i32
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct IVec2 {
    pub x: i32,
    pub y: i32
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Rotation {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Encode for BlockPos {
    fn encode(&self, w: &mut Writer) {
        v32::new(self.x).encode(w);
        v32::new(self.y).encode(w);
        v32::new(self.z).encode(w);
    }
}

impl Decode<'_> for BlockPos {
    fn decode(r: &mut Reader<'_>) -> Option<Self> {
        Some(Self {
            x: v32::decode(r)?.get(),
            y: v32::decode(r)?.get(),
            z: v32::decode(r)?.get()
        }.into())
    }
}

impl Encode for UBlockPos {
    fn encode(&self, w: &mut Writer) {
        v32::new(self.x).encode(w);
        w32::new(self.y as u32).encode(w);
        v32::new(self.z).encode(w);
    }
}

impl Decode<'_> for UBlockPos {
    fn decode(r: &mut Reader<'_>) -> Option<Self> {
        Some(Self {
            x: v32::decode(r)?.get(),
            y: w32::decode(r)?.get() as i32,
            z: v32::decode(r)?.get()
        }.into())
    }
}

impl Encode for IVec2 {
    fn encode(&self, w: &mut Writer) {
        self.x.encode(w);
        self.y.encode(w);
    }
}

impl Decode<'_> for IVec2 {
    fn decode(r: &mut Reader<'_>) -> Option<Self> {
        Some(Self {
            x: i32::decode(r)?,
            y: i32::decode(r)?
        })
    }
}

impl Encode for Vec2 {
    fn encode(&self, w: &mut Writer) {
        self.x.encode(w);
        self.y.encode(w);
    }
}

impl Decode<'_> for Vec2 {
    fn decode(r: &mut Reader<'_>) -> Option<Self> {
        Some(Self {
            x: f32::decode(r)?,
            y: f32::decode(r)?
        })
    }
}

impl Encode for Vec3 {
    fn encode(&self, w: &mut Writer) {
        self.x.encode(w);
        self.y.encode(w);
        self.z.encode(w);
    }
}

impl Decode<'_> for Vec3 {
    fn decode(r: &mut Reader<'_>) -> Option<Self> {
        Some(Self {
            x: f32::decode(r)?,
            y: f32::decode(r)?,
            z: f32::decode(r)?
        })
    }
}

impl Encode for Rotation {
    fn encode(&self, w: &mut Writer) {
        ((self.x / (360.0 / 256.0)) as u8).encode(w);
        ((self.y / (360.0 / 256.0)) as u8).encode(w);
        ((self.z / (360.0 / 256.0)) as u8).encode(w);
    }
}

impl Decode<'_> for Rotation {
    fn decode(r: &mut Reader<'_>) -> Option<Self> {
        Some(Self {
            x: (u8::decode(r)? as f32) * (360.0 / 256.0),
            y: (u8::decode(r)? as f32) * (360.0 / 256.0),
            z: (u8::decode(r)? as f32) * (360.0 / 256.0)
        })
    }
}