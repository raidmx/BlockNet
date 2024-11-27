use derive::{Decode, Encode, Packet};

/// Provides the client a list of recipes that have been unlocked, restricting the recipes that
/// appear in the recipe book.
#[derive(Debug, Clone, Encode, Decode, Packet)]
pub struct UnlockedRecipes {
    /// The type of unlock that this packet will cause.
    pub unlock_type: UnlockedRecipesType,
    /// A list of recipe names that have been unlocked.
    pub recipes: Vec<String>,
}

/// Controls the type of unlock that a [UnlockedRecipes] will cause.
#[derive(Debug, Clone, Encode, Decode)]
#[encoding(type = u32)]
pub enum UnlockedRecipesType {
    Empty,
    InitiallyUnlocked,
    NewlyUnlocked,
    RemoveUnlocked,
    RemoveAllUnlocked,
}
