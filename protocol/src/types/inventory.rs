use std::fmt::Debug;

use bytes::Bytes;
use crate::types::Vec3;
use binary::{Decode, Encode, Reader, v32, w32, w64, Writer};
use derive::{Decode, Encode};
use crate::types::{BlockPos, ItemInstance, UBlockPos};

#[derive(Debug, Clone, Encode, Decode)]
#[encoding(type = v32)]
pub enum Window {
    Inventory = 0,
    OffHand = 119,
    Armour = 120,
    UI = 124,
}

#[repr(u32)]
#[derive(Debug, Clone, Encode, Decode)]
#[encoding(type = w32)]
pub enum InventoryActionSource {
    Container {
        window: Window
    } = 0,
    World {
        source_flags: w32
    } = 2,
    Creative = 3,
    TODO {
        window: Window
    } = 99999,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct InventoryAction<'a> {
    pub source_type: InventoryActionSource,
    pub inventory_slot: u32,
    pub old_item: ItemInstance<'a>,
    pub new_item: ItemInstance<'a>,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct LegacySetItemSlot {
    pub container_id: u8,
    pub slots: Bytes,
}

#[derive(Debug, Clone, Default, Encode, Decode)]
#[encoding(type = U8)]
pub enum InventoryTransactionData {
    #[default]
    NormalTransaction = 0,
    MismatchTransaction = 1,
    UseItemTransactionData = 2,
    UseItemOnEntityTransaction = 3,
    ReleaseItemTransaction = 4,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct MismatchTransaction;

#[derive(Debug, Clone, Encode, Decode)]
pub struct NormalTransaction {}

#[derive(Debug, Clone, Encode, Decode)]
pub struct ReleaseItemTransaction<'a> {
    pub action_type: ReleaseItemAction,
    pub hot_bar_slot: v32,
    pub held_item: ItemInstance<'a>,
    pub head_position: Vec3,
}

#[derive(Debug, Clone, Encode, Decode)]
#[encoding(type = w32)]
pub enum ReleaseItemAction {
    Release,
    Consume,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct UseItemOnEntityTransaction<'a> {
    pub target_entity_runtime_id: w64,
    pub action_type: w32,
    pub hot_bar_slot: v32,
    pub held_item: ItemInstance<'a>,
    pub position: Vec3,
    pub clicked_position: Vec3,
}

#[derive(Debug, Clone, Default, Encode, Decode)]
pub struct UseItemTransactionData<'a> {
    pub action_type: w32,
    pub block_position: UBlockPos,
    pub block_face: v32,
    pub hot_bar_slot: v32,
    pub held_item: ItemInstance<'a>,
    pub position: Vec3,
    pub clicked_position: Vec3,
    pub block_runtime_id: w32,
}

#[derive(Debug, Clone, Default)]
pub struct PlayerInventoryAction<'a> {
    pub legacy_request_id: v32,
    pub legacy_set_item_slots: Vec<LegacySetItemSlot>,
    pub actions: Vec<InventoryAction<'a>>,
    pub action_type: w32,
    pub block_position: BlockPos,
    pub block_face: v32,
    pub hot_bar_slot: v32,
    pub held_item: ItemInstance<'a>,
    pub position: Vec3,
    pub clicked_position: Vec3,
    pub block_runtime_id: w32,
}

impl<'a> Encode for PlayerInventoryAction<'a> {
    fn encode(&self, w: &mut Writer) {
        self.legacy_request_id.encode(w);

        if *self.legacy_request_id < -1 && (*self.legacy_request_id & 1) == 0 {
            self.legacy_set_item_slots.encode(w);
        }

        self.actions.encode(w);
        self.action_type.encode(w);
        self.block_position.encode(w);
        self.block_face.encode(w);
        self.hot_bar_slot.encode(w);
        self.held_item.encode(w);
        self.position.encode(w);
        self.clicked_position.encode(w);
        self.block_runtime_id.encode(w);
    }
}

impl<'a> Decode<'a> for PlayerInventoryAction<'a> {
    fn decode(r: &mut Reader<'a>) -> Option<Self> {
        let legacy_request_id = v32::decode(r)?;
        let read_legacy_set_item_slots = *legacy_request_id < -1 && (*legacy_request_id & 1) == 0;

        let legacy_set_item_slots = if read_legacy_set_item_slots {
            Vec::<LegacySetItemSlot>::decode(r)?
        } else {
            Vec::new()
        };

        let actions = Vec::<InventoryAction>::decode(r)?;
        let action_type = w32::decode(r)?;
        let block_position = BlockPos::decode(r)?;
        let block_face = v32::decode(r)?;
        let hot_bar_slot = v32::decode(r)?;
        let held_item = ItemInstance::decode(r)?;
        let position = Vec3::decode(r)?;
        let clicked_position = Vec3::decode(r)?;
        let block_runtime_id = w32::decode(r)?;
        
        Some(Self {
            legacy_request_id,
            legacy_set_item_slots,
            actions,
            action_type,
            block_position,
            block_face,
            hot_bar_slot,
            held_item,
            position,
            clicked_position,
            block_runtime_id
        })
    }
}