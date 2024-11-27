use derive::{Decode, Encode, Packet};

#[derive(Debug, PartialEq, Clone, Encode, Decode, Packet)]
pub struct SetPlayerInventoryOptions {
    pub left_inventory_tab: InventoryLeftTab,
    pub right_inventory_tab: InventoryRightTab,
    pub filtering: bool,
    pub inventory_layout: InventoryLayout,
    pub crafting_layout: InventoryLayout,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
#[encoding(type = u8)]
pub enum InventoryLayout {
    None,
    Survival,
    RecipeBook,
    Creative,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
#[encoding(type = u8)]
pub enum InventoryLeftTab {
    None,
    Construction,
    Equipment,
    Items,
    Nature,
    Search,
    Survival,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
#[encoding(type = u8)]
pub enum InventoryRightTab {
    None,
    FullScreen,
    Crafting,
    Armour,
}
