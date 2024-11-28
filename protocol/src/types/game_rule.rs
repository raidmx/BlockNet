use binary::w32;
use derive::{Decode, Encode};

#[derive(Debug, Clone, Encode, Decode)]
pub struct GameRule {
    pub name: String,
    pub can_be_modified_by_player: bool,
    pub value: GameRuleValue,
}

#[repr(u32)]
#[derive(Debug, Clone, Encode, Decode)]
#[encoding(type = w32)]
pub enum GameRuleValue {
    Bool(bool) = 1,
    Int(w32) = 2,
    Float(f32) = 3,
}