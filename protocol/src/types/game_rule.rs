use binary::VarU32;
use derive::{Decode, Encode};

#[derive(Debug, Clone, Encode, Decode)]
pub struct GameRule {
    pub name: String,
    pub can_be_modified_by_player: bool,
    pub value: GameRuleValue,
}

#[repr(u32)]
#[derive(Debug, Clone, Encode, Decode)]
#[encoding(type = VarU32)]
pub enum GameRuleValue {
    Bool(bool) = 1,
    Int(VarU32) = 2,
    Float(f32) = 3,
}