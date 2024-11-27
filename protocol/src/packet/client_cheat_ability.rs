#![allow(deprecated)]

use derive::{Decode, Encode, Packet};
use crate::types::AbilityData;

/// Functions the same as UpdateAbilities. It is unclear why these two are separated.
#[derive(Debug, Clone, Encode, Decode, Packet)]
#[deprecated = "Deprecated as of Bedrock Edition v1.20.10"]
pub struct ClientCheatAbility {
    /// Various data about the abilities of a player, such as ability layers or permissions.
    pub ability_data: AbilityData,
}
