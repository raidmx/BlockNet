use uuid::Uuid;
use binary::{Decode, Encode, Reader, VarI32, VarU32, Writer};
use derive::{Decode, Encode};
use crate::types::{ItemDescriptorCount, ItemStack};

#[repr(u32)]
#[derive(Debug, Clone, Encode, Decode)]
#[encoding(type = VarU32)]
pub enum Recipe<'a> {
    ShapelessRecipe(ShapelessRecipe<'a>),
    ShapedRecipe(ShapedRecipe<'a>),
    FurnaceRecipe(FurnaceRecipe<'a>),
    FurnaceDataRecipe(FurnaceRecipe<'a>),
    MultiRecipe(MultiRecipe),
    ShulkerBoxRecipe(ShulkerBoxRecipe<'a>),
    ShapelessChemistryRecipe(ShapelessChemistryRecipe<'a>),
    ShapedChemistryRecipe(ShapedChemistryRecipe<'a>),
    SmithingTransform(SmithingTransformRecipe<'a>),
    SmithingTrim(SmithingTrimRecipe<'a>),
}

/// A recipe specifically used for smithing tables. It has two input items and adds them together,
/// resulting in a new item.
#[derive(Debug, Clone, Encode, Decode)]
pub struct SmithingTransformRecipe<'a> {
    /// A unique ID used to identify the recipe over network. Each recipe must have a unique network
    /// ID. Recommended is to just increment a variable for each unique recipe registered. This
    /// field must never be 0.
    pub recipe_network_id: u32,
    /// A unique ID of the recipe. This ID must be unique amongst all other types of recipes too,
    /// but its functionality is not exactly known.
    pub recipe_id: String,
    /// The item that is used to shape the Base item based on the Addition being applied.
    pub template: ItemDescriptorCount,
    /// The item that the Addition is being applied to in the smithing table.
    pub base: ItemDescriptorCount,
    /// The item that is being added to the Base item to result in a modified item.
    pub addition: ItemDescriptorCount,
    /// The resulting item from the two items being added together.
    pub result: ItemStack<'a>,
    /// The block name that is required to create the output of the recipe. The block is not
    /// prefixed with 'minecraft:', so it will look like 'smithing_table' as an example.
    pub block: String,
}

pub type SmithingTrimRecipe<'a> = ShapelessRecipe<'a>;

#[derive(Debug, Clone, Encode, Decode)]
pub struct MultiRecipe {
    pub uuid: Uuid,
    pub recipe_network_id: VarU32,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct FurnaceDataRecipe<'a> {
    pub furnace_recipe: FurnaceRecipe<'a>,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct FurnaceRecipe<'a> {
    pub network_id: VarI32,
    pub output: ItemStack<'a>,
    pub block: String,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct PotionRecipe {
    pub input_potion_id: VarI32,
    pub input_potion_metadata: VarI32,
    pub reagent_item_id: VarI32,
    pub reagent_item_metadata: VarI32,
    pub output_potion_id: VarI32,
    pub output_potion_metadata: VarI32,
}

pub type ShapedChemistryRecipe<'a> = ShapedRecipe<'a>;

#[derive(Debug, Clone, Default)]
pub struct ShapedRecipe<'a> {
    pub recipe_id: &'a str,
    pub width: i32,
    pub height: i32,
    pub input: Vec<ItemDescriptorCount>,
    pub output: Vec<ItemStack<'a>>,
    pub uuid: Uuid,
    pub block: &'a str,
    pub priority: VarI32,
    pub recipe_network_id: VarU32,
}

impl<'a> Encode for ShapedRecipe<'a> {
    fn encode(&self, w: &mut Writer) {
        self.recipe_id.encode(w);
        self.width.encode(w);
        self.height.encode(w);

        for i in 0..self.width * self.height {
            if i >= self.input.len() as i32 {
                ItemDescriptorCount::default().encode(w);
            } else {
                self.input[i as usize].encode(w);
            }
        }

        self.output.encode(w);
        self.uuid.encode(w);
        self.block.encode(w);
        self.priority.encode(w);
        self.recipe_network_id.encode(w);
    }
}

impl<'a> Decode<'a> for ShapedRecipe<'a> {
    fn decode(r: &mut Reader<'a>) -> Option<Self> {
        let recipe_id = <&str>::decode(r)?;
        let width = i32::decode(r)?;
        let height = i32::decode(r)?;
        let input = (0..width*height).map(|_| ItemDescriptorCount::decode(r)).collect::<Option<_>>()?;
        let output = Vec::decode(r)?;
        let uuid = Uuid::decode(r)?;
        let block = <&str>::decode(r)?;
        let priority = VarI32::decode(r)?;
        let recipe_network_id = VarU32::decode(r)?;

        Some(Self {
            recipe_id,
            width,
            height,
            input,
            output,
            uuid,
            block,
            priority,
            recipe_network_id
        })
    }
}

pub type ShapelessChemistryRecipe<'a> = ShapelessRecipe<'a>;

#[derive(Debug, Clone, Encode, Decode)]
pub struct ShapelessRecipe<'a> {
    pub recipe_id: String,
    pub input: Vec<ItemDescriptorCount>,
    pub output: Vec<ItemStack<'a>>,
    pub uuid: Uuid,
    pub block: String,
    pub priority: VarI32,
    pub recipe_network_id: VarU32,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct MaterialReducer {
    pub network_id: i32,
    pub metadata_value: u32,
    pub outputs: Vec<MaterialReducerOutput>,
}

#[derive(Debug, Clone)]
pub struct ItemType {
    pub network_id: i32,
    pub metadata_value: u32,
}

impl Encode for ItemType {
    fn encode(&self, w: &mut Writer) {
        VarI32::new((self.network_id << 16) | (self.metadata_value as i32)).encode(w);
    }
}

impl Decode<'_> for ItemType {
    fn decode(r: &mut Reader<'_>) -> Option<Self> {
        let value = VarI32::decode(r)?.get();
        
        Some(Self {
            network_id: value << 16,
            metadata_value: (value & 0x7fff) as u32,
        })
    }
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct MaterialReducerOutput {
    pub network_id: VarI32,
    pub count: VarI32,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct PotionContainerChangeRecipe {
    pub input_item_id: VarI32,
    pub reagent_item_id: VarI32,
    pub output_item_id: VarI32,
}

pub type ShulkerBoxRecipe<'a> = ShapelessRecipe<'a>;
