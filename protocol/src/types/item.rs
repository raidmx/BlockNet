use bytes::BytesMut;
use num_derive::{FromPrimitive, ToPrimitive};
use binary::{Decode, Encode, Reader, VarI32, VarU32, Writer};
use derive::{Decode, Encode};
use crate::nbt::{LittleEndian, Tag, NBT};
use crate::types::SliceU32;

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum UseItemAction {
    ClickBlock,
    ClickAir,
    BreakBlock,
}

#[derive(Debug, Clone, FromPrimitive, ToPrimitive, Encode, Decode)]
#[encoding(type = i32)]
pub enum UseItemMethod {
    EquipArmour,
    Eat,
    Attack,
    Consume,
    Throw,
    Shoot,
    Place,
    FillBottle,
    FillBucket,
    PourBucket,
    UseTool,
    Interact,
    Retrieved,
    Dyed,
    Traded,
}

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum UseItemOnEntityAction {
    Interact,
    Attack,
}

#[derive(Debug, Clone, Default)]
pub struct ItemInstance<'a> {
    pub stack_network_id: VarI32,
    pub stack: ItemStack<'a>,
}

impl<'a> Encode for ItemInstance<'a> {
    fn encode(&self, w: &mut Writer) {
        self.stack.network_id.encode(w);
        if *self.stack.network_id == 0 {
            // The item was air, so there's no more data to follow. Return immediately.
            return;
        }

        self.stack.count.encode(w);
        self.stack.metadata_value.encode(w);

        let has_net_id = *self.stack_network_id != 0;
        has_net_id.encode(w);

        if has_net_id {
            self.stack_network_id.encode(w);
        }

        self.stack.block_runtime_id.encode(w);

        let mut extra_data = Writer::default();

        if let Tag::Compound(m) = &self.stack.nbt_data {
            if !m.is_empty() {
                -1_i16.encode(&mut extra_data);
                1_u8.encode(&mut extra_data);
                self.stack.nbt_data.encode(&mut extra_data);
            } else {
                0_i16.encode(&mut extra_data);
            }
        } else {
            panic!("nbt data is not a compound tag");
        }

        self.stack.can_be_placed_on.encode(&mut extra_data);
        self.stack.can_break.encode(&mut extra_data);

        // TODO: Shield Runtime ID
        if *self.stack.network_id == 1 {
            0_i64.encode(&mut extra_data);
        }

        extra_data.encode(w);
    }
}

impl<'a> Decode<'a> for ItemInstance<'a> {
    fn decode(r: &mut Reader<'a>) -> Option<Self> {
        let network_id = VarI32::decode(r)?;

        if *network_id == 0 {
            // The item was air, so there's no more data to follow. Return immediately.
            return Some(Self::default());
        }

        let count = u16::decode(r)?;
        let metadata_value = VarU32::decode(r)?;
        let mut stack_network_id = VarI32::new(0);

        if bool::decode(r)? {
            stack_network_id = VarI32::decode(r)?;
        }

        let block_runtime_id = VarI32::decode(r)?;

        let mut extra_data = BytesMut::decode(r)?.freeze();
        let extra_reader = &mut &extra_data[..];

        let mut nbt_data = NBT::<LittleEndian>::default();

        let length = i16::decode(extra_reader)?;
        if length == -1 {
            let version = u8::decode(extra_reader)?;

            if version == 1 {
                nbt_data = NBT::<LittleEndian>::decode(r)?;
            } else {
                panic!("unknown item user data version {}", version);
            }
        } else if length > 0 {
            nbt_data = NBT::<LittleEndian>::decode(r)?;
        }

        let can_be_placed_on = SliceU32::decode(r)?;
        let can_break = SliceU32::decode(r)?;
        let has_network_id = *network_id == 1; // TODO: Shield Network Id

        let stack = ItemStack {
            network_id,
            count,
            metadata_value,
            block_runtime_id,
            nbt_data,
            can_be_placed_on,
            can_break,
            has_network_id
        };
        
        Some(Self {
            stack_network_id,
            stack
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct ItemStack<'a> {
    pub network_id: VarI32,
    pub metadata_value: VarU32,
    pub block_runtime_id: VarI32,
    pub count: u16,
    pub nbt_data: NBT<'a, LittleEndian>,
    pub can_be_placed_on: SliceU32<&'a str>,
    pub can_break: SliceU32<&'a str>,
    pub has_network_id: bool,
}

impl<'a> Encode for ItemStack<'a> {
    fn encode(&self, w: &mut Writer) {
        self.network_id.encode(w);
        if *self.network_id == 0 {
            // The item was air, so there's no more data to follow. Return immediately.
            return;
        }

        self.count.encode(w);
        self.metadata_value.encode(w);
        self.block_runtime_id.encode(w);

        let mut extra_data = Writer::default();

        if let Tag::Compound(m) = &self.nbt_data {
            if !m.is_empty() {
                -1_i16.encode(&mut extra_data);
                1_u8.encode(&mut extra_data);
                self.nbt_data.encode(&mut extra_data);
            } else {
                0_i16.encode(&mut extra_data);
            }
        } else {
            panic!("nbt data is not a compound tag");
        }

        self.can_be_placed_on.encode(&mut extra_data);
        self.can_break.encode(&mut extra_data);

        // TODO: Shield Runtime ID
        if *self.network_id == 1 {
            0_i64.encode(&mut extra_data);
        }

        extra_data.encode(w);
    }
}

impl<'a> Decode<'a> for ItemStack<'a> {
    fn decode(r: &mut Reader<'a>) -> Option<Self> {
        let network_id = VarI32::decode(r)?;
        if *network_id == 0 {
            // The item was air, so there's no more data to follow. Return immediately.
            return Some(Self::default());
        }

        let count = u16::decode(r)?;
        let metadata_value = VarU32::decode(r)?;
        let block_runtime_id = VarI32::decode(r)?;

        let mut extra_data = BytesMut::decode(r)?.freeze();
        let extra_reader = &mut &extra_data[..];

        let mut nbt_data = NBT::<LittleEndian>::default();

        let length = i16::decode(extra_reader)?;
        if length == -1 {
            let version = u8::decode(extra_reader)?;

            if version == 1 {
                nbt_data = NBT::<LittleEndian>::decode(r)?;
            } else {
                panic!("unknown item user data version {}", version);
            }
        } else if length > 0 {
            nbt_data = NBT::<LittleEndian>::decode(r)?;
        }

        let can_be_placed_on = SliceU32::decode(r)?;
        let can_break = SliceU32::decode(r)?;
        let has_network_id = *network_id == 1; // TODO: Shield Network Id

        Some(Self {
            network_id,
            count,
            metadata_value,
            block_runtime_id,
            nbt_data,
            can_be_placed_on,
            can_break,
            has_network_id
        })
    }
}
