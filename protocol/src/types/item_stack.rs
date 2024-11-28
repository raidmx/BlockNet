use std::fmt::Debug;

use num_derive::{FromPrimitive, ToPrimitive};
use binary::{v32, w32};
use derive::{Decode, Encode};
use crate::nbt::{NetworkLittleEndian, NBT};
use crate::types::{ItemDescriptorCount, ItemStack};

#[repr(u8)]
#[derive(Debug, Clone, Encode, Decode)]
#[encoding(type = u8)]
pub enum StackRequestAction<'a> {
    TakeStackRequestAction(TakeStackRequestAction),
    PlaceStackRequestAction(PlaceStackRequestAction),
    SwapStackRequestAction(SwapStackRequestAction),
    DropStackRequestAction(DropStackRequestAction),
    DestroyStackRequestAction(DestroyStackRequestAction),
    ConsumeStackRequestAction(ConsumeStackRequestAction),
    CreateStackRequestAction(CreateStackRequestAction),
    PlaceInContainerStackRequestAction(PlaceInContainerStackRequestAction),
    TakeOutContainerStackRequestAction(TakeOutContainerStackRequestAction),
    LabTableCombineStackRequestAction,
    BeaconPaymentStackRequestAction(BeaconPaymentStackRequestAction),
    MineBlockStackRequestAction(MineBlockStackRequestAction),
    CraftRecipeStackRequestAction(CraftRecipeStackRequestAction),
    AutoCraftRecipeStackRequestAction(AutoCraftRecipeStackRequestAction),
    CraftCreativeStackRequestAction(CraftCreativeStackRequestAction),
    CraftRecipeOptionalStackRequestAction(CraftRecipeOptionalStackRequestAction),
    CraftGrindstoneRecipeStackRequestAction(CraftGrindstoneRecipeStackRequestAction),
    CraftLoomRecipeStackRequestAction(CraftLoomRecipeStackRequestAction),
    CraftNonImplementedStackRequestAction,
    CraftResultsDeprecatedStackRequestAction(CraftResultsDeprecatedStackRequestAction<'a>),
}

#[derive(Debug, Copy, Clone, Default, Encode, Decode)]
#[encoding(type = i32)]
pub enum FilterCause {
    #[default]
    ServerChatPublic,
    ServerChatWhisper,
    SignText,
    AnvilText,
    BookAndQuillText,
    CommandBlockText,
    BlockActorDataText,
    JoinEventText,
    LeaveEventText,
    SlashCommandChat,
    CartographyText,
    KickCommand,
    TitleCommand,
    SummonCommand,
}

#[derive(Debug, Clone, Default, Encode, Decode)]
pub struct ItemStackRequestEntry<'a> {
    pub request_id: v32,
    pub actions: Vec<StackRequestAction<'a>>,
    pub filter_strings: Vec<&'a str>,
    pub filter_cause: FilterCause,
}

#[derive(Debug, Clone, PartialEq, FromPrimitive, ToPrimitive, Encode, Decode)]
#[encoding(type = u8)]
pub enum ItemStackResponseStatus {
    Ok,
    Error,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct ItemComponentEntry<'a> {
    pub name: &'a str,
    pub data: NBT<'a, NetworkLittleEndian>,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct ItemEnchantments {
    pub slot: i32,
    pub enchantments: [Vec<EnchantmentInstance>; 3],
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct EnchantmentInstance {
    pub enchantment_type: u8,
    pub level: u8,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct EnchantmentOption {
    pub cost: w32,
    pub enchantments: ItemEnchantments,
    pub name: String,
    pub recipe_network_id: w32,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct ItemEntry<'a> {
    pub name: &'a str,
    pub runtime_id: i16,
    pub component_based: bool,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct ItemStackResponseEntry {
    pub status: ItemStackResponseStatus,
    pub request_id: v32,
    pub container_info: Vec<StackResponseContainerInfo>,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct StackRequestSlotInfo {
    pub container_id: u8,
    pub slot: u8,
    pub stack_network_id: v32,
}
#[derive(Debug, Clone, Encode, Decode)]
pub struct DestroyStackRequestAction {
    pub count: u8,
    pub source: StackRequestSlotInfo,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct DropStackRequestAction {
    pub count: u8,
    pub source: StackRequestSlotInfo,
    pub randomly: bool,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct AutoCraftRecipeStackRequestAction {
    pub recipe_network_id: u32,
    pub times_crafted: u8,
    pub ingredients: Vec<ItemDescriptorCount>,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct BeaconPaymentStackRequestAction {
    pub primary_effect: v32,
    pub secondary_effect: v32,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct MineBlockStackRequestAction {
    pub hotbar_slot: v32,
    pub predicted_durability: v32,
    pub stack_network_id: v32,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct TakeStackRequestAction {
    pub count: u8,
    pub source: StackRequestSlotInfo,
    pub destination: StackRequestSlotInfo,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct SwapStackRequestAction {
    pub source: StackRequestSlotInfo,
    pub destination: StackRequestSlotInfo,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct TakeOutContainerStackRequestAction {
    pub count: u8,
    pub source: StackRequestSlotInfo,
    pub destination: StackRequestSlotInfo,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct ConsumeStackRequestAction {
    pub count: u8,
    pub source: StackRequestSlotInfo,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct CraftCreativeStackRequestAction {
    pub creative_item_network_id: w32,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct CraftGrindstoneRecipeStackRequestAction {
    pub recipe_network_id: w32,
    pub cost: v32,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct CraftLoomRecipeStackRequestAction {
    pub pattern: String,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct CraftRecipeOptionalStackRequestAction {
    pub recipe_network_id: w32,
    pub filter_string_index: i32,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct CraftRecipeStackRequestAction {
    pub recipe_network_id: w32,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct CraftResultsDeprecatedStackRequestAction<'a> {
    pub result_items: Vec<ItemStack<'a>>,
    pub times_crafted: u8,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct CreateStackRequestAction {
    pub results_slot: u8,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct PlaceInContainerStackRequestAction {
    pub count: u8,
    pub source: StackRequestSlotInfo,
    pub destination: StackRequestSlotInfo,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct PlaceStackRequestAction {
    pub count: u8,
    pub source: StackRequestSlotInfo,
    pub destination: StackRequestSlotInfo,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct StackResponseContainerInfo {
    pub container_id: u8,
    pub slot_info: Vec<StackResponseSlotInfo>,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct StackResponseSlotInfo {
    pub slot: u8,
    pub hotbar_slot: u8,
    pub count: u8,
    pub stack_network_id: v32,
    pub custom_name: String,
    pub durability_correction: v32,
}