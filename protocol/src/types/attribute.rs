use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use binary::{Decode, Encode, Reader, Writer};
use derive::{Decode, Encode};
use crate::types::SliceU32;

#[derive(Debug, Clone, FromPrimitive, ToPrimitive, Encode, Decode)]
#[encoding(type = i32)]
pub enum AttributeModifierOperand {
    Min,
    Max,
    Current,
}

#[derive(Debug, Clone, FromPrimitive, ToPrimitive, Encode, Decode)]
#[encoding(type = i32)]
pub enum AttributeModifierOperation {
    Addition,
    MultiplyBase,
    MultiplyTotal,
    Cap,
}

#[derive(Debug, Clone, Default)]
pub struct AttributeValue<'a> {
    pub name: &'a str,
    pub min: f32,
    pub max: f32,
    pub value: f32,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct AttributeModifier<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub amount: f32,
    pub operation: AttributeModifierOperation,
    pub operand: AttributeModifierOperand,
    pub serializable: bool,
}

#[derive(Debug, Clone, Default)]
pub struct Attribute<'a> {
    pub value: AttributeValue<'a>,
    pub default: f32,
    pub modifiers: SliceU32<AttributeModifier<'a>>,
}

impl<'a> Encode for Attribute<'a> {
    fn encode(&self, w: &mut Writer) {
        self.value.min.encode(w);
        self.value.max.encode(w);
        self.value.value.encode(w);
        self.default.encode(w);
        self.value.name.encode(w);
        self.modifiers.encode(w);
    }
}

impl<'a> Decode<'a> for Attribute<'a> {
    fn decode(r: &mut Reader<'a>) -> Option<Self> {
        let min = f32::decode(r)?;
        let max = f32::decode(r)?;
        let value = f32::decode(r)?;
        let default = f32::decode(r)?;
        let name = <&str>::decode(r)?;
        let modifiers = SliceU32::decode(r)?;

        let attribute_value = AttributeValue {
            name,
            min,
            max,
            value
        };

        Some(Self {
            value: attribute_value,
            default,
            modifiers
        })
    }
}