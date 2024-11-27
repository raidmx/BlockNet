use derive::{Decode, Encode, Packet};
use crate::types::recipe::{
    MaterialReducer, PotionContainerChangeRecipe, PotionRecipe, Recipe,
};

/// Sent by the server to let the client know all crafting data that the server maintains. This
/// includes shapeless crafting, crafting table recipes, furnace recipes etc. Each crafting
/// station's recipes are included in it.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct CraftingData<'a> {
    /// List of all recipes available on the server. It includes among others shapeless, shaped and
    /// furnace recipes. The client will only be able to craft these recipes.
    pub recipes: Vec<Recipe<'a>>,
    /// List of all potion mixing recipes which may be used in the brewing stand.
    pub potion_recipes: Vec<PotionRecipe>,
    /// List of all recipes to convert a potion from one type to another, such as from a drinkable
    /// potion to a splash potion, or from a splash potion to a lingering potion.
    pub potion_container_change_recipes: Vec<PotionContainerChangeRecipe>,
    /// List of all material reducers. These are primarily used in the Education Edition chemistry
    /// system.
    pub material_reducers: Vec<MaterialReducer>,
    /// Indicates if all recipes currently active on the client should be cleaned. Doing this means
    /// that the client will have no recipes active by itself: any CraftingData packets previously
    /// sent will also be discarded, and only the recipes in this CraftingData packet will be used.
    pub clear_recipes: bool,
}