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

use glam::IVec3;
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

use binary::{generate, Array, Decode, Encode, Reader, VarI32, VarU32, Writer};

pub type SliceU8<T> = Array<u8, T>;
pub type SliceU16<T> = Array<u16, T>;
pub type SliceU32<T> = Array<u32, T>;

generate!(BlockPos, <>, IVec3);
generate!(UBlockPos, <>, IVec3);

impl Encode for BlockPos {
    fn encode(&self, w: &mut Writer) {
        VarI32::new(self.x).encode(w);
        VarI32::new(self.y).encode(w);
        VarI32::new(self.z).encode(w);
    }
}

impl Decode<'_> for BlockPos {
    fn decode(r: &mut Reader<'_>) -> Option<Self> {
        Some(IVec3 {
            x: VarI32::decode(r)?.get(),
            y: VarI32::decode(r)?.get(),
            z: VarI32::decode(r)?.get()
        }.into())
    }
}

impl Encode for UBlockPos {
    fn encode(&self, w: &mut Writer) {
        VarI32::new(self.x).encode(w);
        VarU32::new(self.y as u32).encode(w);
        VarI32::new(self.z).encode(w);
    }
}

impl Decode<'_> for UBlockPos {
    fn decode(r: &mut Reader<'_>) -> Option<Self> {
        Some(IVec3 {
            x: VarI32::decode(r)?.get(),
            y: VarU32::decode(r)?.get() as i32,
            z: VarI32::decode(r)?.get()
        }.into())
    }
}